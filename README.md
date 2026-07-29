# Ren'Rs

这里是 Ren'Rs！也就是使用 Rust 并完成全平台构建打包的一个项目！

一款你的下一代视觉小说引擎！

编译器和运行器分开处理！每一个 .rrs 都是一个完整的游戏！

Lua 教程可以参考[官方教程 gitbook](https://candysharkstudio.gitbook.io/renrs-lua-tutorial)，需要挂七根木棍才能上

请注意！使用本教程的 Lua 已经可以解决 99% 的 UI/UX 问题，视觉小说完全可以直接用这个做！还有 1% 的问题是可能实在是引擎部分解决不了的，需要手动修改 Ren'Rs 的源代码的。。

是的！我们采取全部 UI 都使用（未知）框架去绘制！不仅性能++，跨平台性也++了！

本次跨平台的整合架构以及后端架构选择都在[这里](./CROSS_PLATFORM.md)

### 如何方便的修改项目名？

- 很方便，下载 Python，修改 ren-rs.config.json 里面的内容，随后直接运行 change_by_json 即可！
- 你也可以无需修改 ren-rs.config.json，直接跑 change_by_ci.py，但这是交给 CI 用的（用于在命令行里面修改项目名等基本信息。。

### 如何进行 PC 开发？

1. 安装 Rust（无需 Node）
2. 直接在项目根目录运行 `cargo run` 即可！Cargo 会自动帮你拉取所有依赖！
3. 默认未使用 enable-desktop，默认未启用 enable-frontend，请各位按需启用。

- **本地开发使用 change_by_json 去替换各位的项目基本信息！请修改 ren-rs.config.json 里面的内容！其余文件请酌情修改。。**
- 在你不确定是否要提交 pr 时，请尽量不要随意修改源代码里面的内容！除了你要为那 1% 的内容进行开发除外。。但也尽量只动 ui 文件夹里的内容即可，别的地方暂时就不要动啦~

### 如何进行 Android 开发？

1. 安装 Android Studio
2. 安装 Java
3. 安装 [cargo-mobile2](https://github.com/tauri-apps/cargo-mobile2)
4. 在 Android Studio 中下载 Android 模拟器或者连接真机
5. 设置环境变量：CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=你的 NDK 链接目录（实在不会可以看我的 .cargo）
6. 在项目根目录运行：`cargo android run`

### 如何进行 iOS 开发？（仅适用于 macOS）

1. 安装 Xcode
2. 安装 [cargo-mobile2](https://github.com/tauri-apps/cargo-mobile2)
3. 在 Xcode 中下载 iOS 模拟器或者连接真机
4. 在 gen/apple 目录下运行 `xcodegen generate`
5. 在项目根目录运行：`cargo apple run`

好了！如果还有啥不懂的操作请观看我的 [RenRsAction](https://github.com/xphost008/RenRsAction) 项目哦~！

哦对了，直接使用 actions 生成的 Android 或 iOS App 是没有签名的！你需要要么自己手动签名一个，要么参考我的 github actions 自己使用 keytool 签名。。

Android 签名流程：你需要在你的电脑上安装 `Java`，随后使用内置的 keytool 参考我的 github actions 签名！
iOS 签名流程：你需要开通一个 Apple Developer 账号，然后在你的账户中创建一个 App ID，然后自行使用该 App ID 进行签名。

## 注意事项

在开发中，请一定要记得！Ren'Rs 支持且仅支持如下音频格式、如下图片格式以及如下字体格式：

音频：
1. mp3
2. wav
3. flac
4. ogg
5. aac

图片：
1. png
2. jpg
3. bmp
4. gif
5. webp
6. avif
7. svg

字体：
1. ttf
2. otf
3. woff
4. woff2

除此之外，Ren'Rs 均不支持以上任意以外的格式！例如音频的 m4a、图片的 tiff 等等全不支持。（除非你提交一个 PR 来支持它）

# 开源协议

以 MIT MIT MIT 协议开放源代码！各位仅需在【帮助】页鸣谢一下原作者即可！我同样允许各位以闭源形式发布各位的视觉小说！

# 鸣谢

1. 鲛糖拿铁（xphost008）：框架开发者
2. 小万泥（Firedragon0659）：框架开发者

# 使用事宜

本框架允许使用交叉编译！请各位自行添加：

```bash
rustup target add <你的架构>
```
