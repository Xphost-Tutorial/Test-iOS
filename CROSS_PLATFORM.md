# 这里简要记录一下 Ren'Rs 的跨平台性！

目前 Ren'Rs 自动换了框架之后，不仅性能++，跨平台性也++了！

我们使用了 [winit](https://crates.io/crates/winit) + [softbuffer](https://crates.io/crates/softbuffer) 的方式去绘制我们的 UI，性能接近原生！

与此同时，我们的框架的跨平台性也 ++++ 了！！

适配了许多冷门的架构和操作系统！

参见下表：

|操作系统|架构|兼容度|
|----|----|----|
|Windows|x86_64|100%|
|Windows|arm64|100%|
|Windows|i686|100%|
|macOS|x86_64|100%|
|macOS|arm64|100%|
|Linux|x86_64|100%|
|Linux|arm64|100%|
|Android|arm64|100%|
|Android|x86_64|100%|
|Android|armv7|100%|
|iOS|arm64|缺少测试|

接下来是一些冷门操作系统与架构！

|操作系统|架构|兼容度|
|----|----|----|
|FreeBSD|x86_64|100%|
|NetBSD|x86_64|100%|
|OpenBSD|x86_64|100%|
|DragonflyBSD|x86_64|100%|
|illumOS|x86_64|100%|
|Loongnix|loongarch64|100%|
|Loongnix|mips64el|100%|
|统信UOS|loongarch64|100%|
|麒麟|loongarch64|100%|
|OpenHarmony|aarch64|缺少测试|
|BianBu|riscv64|缺少测试|
|Debian|powerpc64le|缺少测试|
|Linux|sparc64|缺少测试|
|Apple tvOS|aarch64|缺少测试|
|Apple watchOS|aarch64|缺少测试|

如果有好心人能够帮我补齐上述的缺少测试架构的列表，我将不胜感激！！

如果各位要是发现了还有什么架构在我上面未能说到的，也请直接提交 Pull Request！！
