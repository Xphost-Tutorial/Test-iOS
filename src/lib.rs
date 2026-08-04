mod utils;

mod start;

// Android 日志
#[cfg(target_os = "android")]
fn init_logging() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("[RenRs]"),
    );
}
// 非 Android 日志
#[cfg(not(target_os = "android"))]
fn init_logging() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();
}

// 桌面端 入口
pub fn desktop_main() {
    init_logging();
    #[cfg(not(target_os = "android"))]
    utils::path::set_config_local_dir().expect("Lifecycle run get config local dir is failed!");
    start::run();
}
#[cfg(any(target_os = "android", target_os = "ios"))]
fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(t) => t,
        Err(e) => {
            let msg = if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else {
                "non-string panic payload".to_string()
            };
            log::error!("Lifecycle run unwind out of `rust` is failed err: \"{}\"", msg);
            std::process::abort();
        }
    }
}
#[cfg(any(target_os = "android", target_os = "ios"))]
fn _start_app() {
    stop_unwind(|| desktop_main());
}
// 移动端 入口
#[unsafe(no_mangle)]
#[inline(never)]
#[cfg(any(target_os = "android", target_os = "ios"))]
pub extern "C" fn start_app() {
    #[cfg(target_os = "android")]
    {
        tao::android_binding!(
            com_xphost,renrs,
            Rust,
            wry::android_setup,
            _start_app,
            ::tao
        );
        wry::android_binding!(com_xphost,renrs);
    }

    #[cfg(target_os = "ios")]
    _start_app()
}

#[unsafe(no_mangle)]
#[cfg(target_os = "android")]
pub extern "system" fn Java_com_xphost_renrs_MainActivity_initSysdirs(
    mut env: jni::JNIEnv,
    _: jni::objects::JClass,
    files_dir: jni::objects::JString,
) {
    let path: String = env.get_string(&files_dir).expect("Lifecycle run android env get files dir is failed!").into();
    utils::path::CONFIG_LOCAL_DIR.set(std::path::PathBuf::from(path)).expect("Lifecycle run android config local dir set is failed!");
}