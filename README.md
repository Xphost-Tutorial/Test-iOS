# Ren Rs Refactor

这里是 Ren-Rs-Refactor！也就是使用 Rust 脱离 Tauri 并完成全平台构建打包的一个项目！

是的！我们完全脱离了 Tauri！采取全部 UI 都使用 winit + softbuffer 去绘制！

如何进行 PC 开发？

1. 安装 Rust（无需 Node）
2. 直接在项目根目录运行 `cargo run` 即可！Cargo 会自动帮你拉取所有依赖！

如何进行 Android 开发？

1. 安装 Android Studio
2. 安装 Java
3. 安装 [cargo-mobile2](https://github.com/tauri-apps/cargo-mobile2)
4. 在 Android Studio 中下载 Android 模拟器或者连接真机
5. 在项目根目录运行：`cargo-mobile2 run`

如何进行 iOS 开发？（仅适用于 macOS）

1. 安装 Xcode
2. 安装 [cargo-mobile2](https://github.com/tauri-apps/cargo-mobile2)
3. 在 Xcode 中下载 iOS 模拟器或者连接真机
4. 在项目根目录运行：`cargo-mobile2 run`

好了！如果还有啥不懂的操作请观看我的 [RenRsAction](https://github.com/xphost008/RenRsAction) 项目哦~！

哦对了，直接使用 actions 生成的 Android 或 iOS App 是没有签名的！你需要要么自己手动签名一个，要么参考我的 github actions 自己使用 keytool 签名。。

Android 签名流程：你需要在你的电脑上安装 `Java`，随后使用内置的 keytool 参考我的 github actions 签名！
iOS 签名流程：你需要开通一个 Apple Developer 账号，然后在你的账户中创建一个 App ID，然后自行使用该 App ID 进行签名。

