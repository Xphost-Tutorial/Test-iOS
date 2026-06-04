
fn main() {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    ren_rs_refactor::desktop_main();
}
