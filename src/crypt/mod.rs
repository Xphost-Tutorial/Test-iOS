use aes::cipher::{BlockDecrypt, KeyInit};
use aes::Aes256;
use std::collections::HashMap;

#[allow(unused)]
fn decrypt_in_place<'a>(
    data: &'a mut [u8],
    key: &Aes256,
    iv: &[u8; 16],
) -> Result<&'a [u8], Box<dyn std::error::Error>> {
    let mut prev_iv = *iv;

    // 逐 16 字节块原地解密
    for chunk in data.chunks_exact_mut(16) {
        let current_cipher: [u8; 16] = chunk.try_into().unwrap();
        key.decrypt_block(chunk.into());
        for i in 0..16 {
            chunk[i] ^= prev_iv[i];
        }
        prev_iv = current_cipher;
    }

    // 显式强校验 PKCS7 填充 (密钥错误的唯一逃生通道)
    let pad_len = *data.last().unwrap() as usize;
    if pad_len == 0 || pad_len > 16 {
        return Err("Invalid Key! PKCS7 validation failed.".into());
    }
    for &b in &data[data.len() - pad_len..] {
        if b != pad_len as u8 {
            return Err("Invalid Key! PKCS7 padding mismatch!".into());
        }
    }

    // 返回去除填充后的纯净明文切片
    Ok(&data[..data.len() - pad_len])
}

#[allow(unused)]
pub fn decrypt_to_memory(
    key: &[u8; 32],
    enc_file: Vec<u8>,
) -> Result<HashMap<String, Vec<u8>>, Box<dyn std::error::Error>> {
    // 将外部传入的数据作为可变内存，准备原地解密
    let mut data = enc_file;
    let mut pos = 0;

    // 解析明文区：[IV1(16)][密文1长度(4)]
    if data.len() < 20 {
        return Err("Invalid enc file: too short.".into());
    }
    let iv1: [u8; 16] = data[pos..pos + 16].try_into()?;
    pos += 16;
    let cipher_len1 = u32::from_le_bytes(data[pos..pos + 4].try_into()?) as usize;
    pos += 4;
    // 提取并原地解密 Block1
    let cipher = Aes256::new(key.into());
    let block1_plain = decrypt_in_place(&mut data[pos..pos + cipher_len1], &cipher, &iv1)?;
    let mut b1_pos = 0;

    // 3. 解析 Block1：[NameLen(1)][DataLen(4)]...[IV2(16)]
    let mut headers = Vec::new();
    // 最后 16 字节是 IV2，所以循环条件是 b1_pos + 16 < 长度
    while b1_pos + 16 < block1_plain.len() {
        let name_len = block1_plain[b1_pos];
        b1_pos += 1;
        let data_len = u32::from_le_bytes(block1_plain[b1_pos..b1_pos + 4].try_into()?);
        b1_pos += 4;
        headers.push((name_len, data_len));
    }
    if b1_pos + 16 != block1_plain.len() {
        return Err("Block 1 format mismatch.".into());
    }
    let iv2: [u8; 16] = block1_plain[b1_pos..b1_pos + 16].try_into()?;

    // 移动全局指针，跳过已处理的 Block1
    pos += cipher_len1;

    // 4. 计算 Block2 密文长度
    let mut plain_len_2: usize = 16 + 4; // [IV3(16)] + [TextPlainLen(4)]
    for &(name_len, data_len) in &headers {
        plain_len_2 += name_len as usize + data_len as usize;
    }
    let cipher_len_2 = plain_len_2 + (16 - plain_len_2 % 16);

    // 5. 提取并原地解密 Block2
    let block2_plain = decrypt_in_place(&mut data[pos..pos + cipher_len_2], &cipher, &iv2)?;
    let mut b2_pos = 0;

    // 6. 解析 Block2：[Name][Data]...[IV3(16)][TextPlainLen(4)]
    let mut map: HashMap<String, Vec<u8>> = HashMap::new();
    for &(name_len, data_len) in &headers {
        let name = String::from_utf8(block2_plain[b2_pos..b2_pos + name_len as usize].to_vec())?;
        b2_pos += name_len as usize;
        let file_data = block2_plain[b2_pos..b2_pos + data_len as usize].to_vec();
        b2_pos += data_len as usize;
        map.insert(name, file_data);
    }

    let iv3: [u8; 16] = block2_plain[b2_pos..b2_pos + 16].try_into()?;
    b2_pos += 16;
    let text_plain_len = u32::from_le_bytes(block2_plain[b2_pos..b2_pos + 4].try_into()?) as usize;

    // 移动全局指针，跳过已处理的 Block2
    pos += cipher_len_2;

    // 7. 计算 Block3 密文长度并提取、原地解密
    let cipher_len_3 = if text_plain_len == 0 {
        16
    } else {
        text_plain_len + (16 - text_plain_len % 16)
    };
    let block3_plain = decrypt_in_place(&mut data[pos..pos + cipher_len_3], &cipher, &iv3)?;

    map.insert("main".to_string(), block3_plain[..text_plain_len].to_vec());
    Ok(map)
}
