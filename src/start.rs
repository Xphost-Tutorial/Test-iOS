use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::WindowBuilder,
};
use wry::{WebView, WebViewBuilder, http};

#[allow(unused)]
use crate::utils::constant::*;

pub fn run() {
    let event_loop = EventLoop::new();
    let mut webview = None;
    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(StartCause::Init) => {
                webview = Some(
                    build_webview(event_loop)
                        .expect("Lifecycle run create webview window is failed in Wry!"),
                );
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
) -> Result<WebView, Box<dyn std::error::Error>> {
    let window = WindowBuilder::new()
        .with_title("Ren Rs")
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
        .with_html(
            r#"<html>
        <body>
            <h1>Hello Wry!!</h1>
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

    Ok(webview)
}
