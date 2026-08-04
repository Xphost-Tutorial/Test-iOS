use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};
use wry::{WebView, WebViewBuilder, http};

#[allow(unused)]
use crate::utils::constant::*;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
const WINDOW_ICON: &[u8] = include_bytes!("../icon.png");

pub fn run() {
    let event_loop = EventLoop::new();
    let mut webview = None;
    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(StartCause::Init) => {
                webview = build_webview(event_loop).ok();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested { .. },
                ..
            } => {
                webview.take();
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    });
}

fn build_webview(
    event_loop: &EventLoopWindowTarget<()>,
) -> Result<(Window, WebView), Box<dyn std::error::Error>> {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    let icon: Option<tao::window::Icon> = None;
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    let icon: Option<tao::window::Icon> = {
        if let Ok(img) = image::load_from_memory(WINDOW_ICON) {
            let rgba = img
                .resize(64, 64, image::imageops::FilterType::Nearest)
                .to_rgba8();
            tao::window::Icon::from_rgba(rgba.into_raw(), 64, 64).ok()
        } else {
            None
        }
    };

    let window = WindowBuilder::new()
        .with_title("Ren Rsoooo")
        .with_window_icon(icon)
        .build(&event_loop)?;

    let builder = WebViewBuilder::new()
        .with_custom_protocol(String::from("renrs"), |_, _request| {
            http::Response::builder()
                .header(http::header::CONTENT_TYPE, "text/html")
                .body(
                    r#"<html>
                <body>
                  Hello Wry!!
                </body>
                </html>"#
                        .as_bytes()
                        .into(),
                )
                .unwrap_or_default()
        })
        .with_background_color((0, 0, 0, 255))
        .with_html(
            r#"<html>
        <body>
            <h1 style="color: white;">Hello Wry!!</h1>
        </body>
        </html>"#,
        );
    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    let webview = builder.build(&window)?;
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    let webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        builder.build_gtk(vbox)?
    };

    Ok((window, webview))
}
