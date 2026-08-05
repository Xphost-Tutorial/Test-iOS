#[cfg(
    all(
        feature = "enable-desktop",
        feature = "enable-steamworks",
        any(
            all(target_os = "windows", target_arch = "x86_64"),
            all(target_os = "macos", target_arch = "aarch64"),
            target_os = "linux",
        )
    )
)]
mod steamworks {}
#[cfg(
    not(
        all(
            feature = "enable-desktop",
            feature = "enable-steamworks",
            any(
                all(target_os = "windows", target_arch = "x86_64"),
                all(target_os = "macos", target_arch = "aarch64"),
                target_os = "linux",
            )
        )
    )
)]
mod steamworks {}
