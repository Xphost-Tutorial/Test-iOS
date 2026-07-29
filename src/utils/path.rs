use crate::utils::constant::*;
use std::{path::PathBuf, sync::OnceLock};

pub static CONFIG_LOCAL_DIR: OnceLock<PathBuf> = OnceLock::new();

pub fn set_config_local_dir() -> Option<()> {
    #[cfg(target_os = "android")]
    {
        // 标准的通过 ndk-context 获取到 Android Path 的实例。
        use jni::objects::{JObject, JString};
        let ctx = ndk_context::android_context();
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.ok()?;
        let mut env = vm.attach_current_thread().ok()?;
        let context = unsafe { JObject::from_raw(ctx.context().cast()) };
        let file_obj = env
            .call_method(&context, "getFilesDir", "()Ljava/io/File;", &[])
            .ok()?
            .l()
            .ok()?;
        let path_obj = env
            .call_method(&file_obj, "getAbsolutePath", "()Ljava/lang/String;", &[])
            .ok()?
            .l()
            .ok()?;

        let jstr: JString = path_obj.into();
        let path: String = env.get_string(&jstr).ok()?.into();
        CONFIG_LOCAL_DIR.set(PathBuf::from(path)).ok()?;
        Some(())
    }
    #[cfg(target_os = "ios")]
    {
        CONFIG_LOCAL_DIR.set(
            PathBuf::from(std::env::var("HOME").ok()?)
                .join("Library")
                .join("Application Support")
                .join(PACKAGE_IDENTIFIER)
                .join("files"),
        ).ok()?;
        Some(())
    }
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        CONFIG_LOCAL_DIR
            .set(
                dirs::data_local_dir()?
                    .join(PACKAGE_IDENTIFIER)
                    .join("files"),
            )
            .ok()?;
        Some(())
    }
}
