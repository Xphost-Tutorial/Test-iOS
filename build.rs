#[cfg(target_os = "windows")]
extern crate winres;
fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("windows") {
        #[cfg(target_os = "windows")]
        {
            let mut res = winres::WindowsResource::new();
            res.set_icon("icon.ico");
            res.compile().unwrap();
        }
    }
}
