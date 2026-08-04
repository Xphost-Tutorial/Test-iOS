#[cfg(not(target_os = "android"))]
use crate::utils::constant::PACKAGE_IDENTIFIER;

use std::{path::PathBuf, sync::OnceLock};

pub static CONFIG_LOCAL_DIR: OnceLock<PathBuf> = OnceLock::new();

#[cfg(not(target_os = "android"))]
pub fn set_config_local_dir() -> Option<()> {
    #[cfg(target_os = "ios")]
    CONFIG_LOCAL_DIR.set(
        PathBuf::from(std::env::var("HOME").ok()?)
            .join("Library")
            .join("Application Support")
            .join(PACKAGE_IDENTIFIER)
            .join("files"),
    ).ok()?;
    #[cfg(not(target_os = "ios"))]
    CONFIG_LOCAL_DIR
        .set(
            dirs::data_local_dir()?
                .join(PACKAGE_IDENTIFIER)
                .join("files"),
        )
        .ok()?;
    Some(())
}
