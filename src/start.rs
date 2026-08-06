use std::sync::{Arc, mpsc};
use axum::{
    Extension, Router, extract::Path, http::{StatusCode, header}, response::{Html, IntoResponse}, routing::{get, put},
};
use tower_http::cors::{CorsLayer, Any};

use crate::utils::constant::*;

const WINDOW_ICON: &[u8] = include_bytes!("../icon.png");

#[derive(PartialEq, Eq)]
enum UserEvent {
    Start,
    ExitWindow,
    FullScreen,
    ExitScreen,
}

type EventCallback = Arc<dyn Fn(UserEvent) + Send + Sync + 'static>;
async fn get_user(Path(user_id): Path<usize>) -> Result<String, StatusCode> {
    const USERNAMES: &[&str] = &["张三", "李四", "王五"];
    match USERNAMES.get(user_id) {
        Some(name) => {
            Ok(name.to_string())
        },
        None => {
            Err(StatusCode::NOT_FOUND)
        }
    }
}
async fn exit_window(Extension(callback): Extension<EventCallback>) -> StatusCode {
    callback(UserEvent::ExitWindow);
    StatusCode::OK
}
async fn get_icon() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "image/png")],
        WINDOW_ICON
    )
}
async fn full_screen(Extension(callback): Extension<EventCallback>, is_full_screen: String) -> StatusCode {
    #[cfg(not(all(target_os = "android", target_os = "ios")))]
    if is_full_screen == "1" {
        callback(UserEvent::FullScreen);
    }else{
        callback(UserEvent::ExitScreen);
    }
    StatusCode::OK
}
fn run_backend_server(tx: mpsc::Sender<UserEvent>, callback: EventCallback) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);
        let app = Router::new()
            .route("/", get(async || -> Html<&'static str> {
                Html(include_str!("frontend/index.html"))
            }))
            .route("/api/exit_window", put(exit_window))
            .route("/api/set_full_screen", put(full_screen))
            .route("/api/get/user/{id}", get(get_user))
            .route("/icon.png", get(get_icon))
            .layer(cors)
            .layer(Extension(callback));

        let addr = "127.0.0.1:8899";
        let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind port 8899");

        log::info!("backend launch success, port: 8899");
        tx.send(UserEvent::Start).unwrap();

        if let Err(e) = axum::serve(listener, app).await {
            log::error!("Server error: {}", e);
        }
    });
}

#[cfg(all(feature = "enable-desktop", feature = "enable-webbrowser", not(target_os = "android"), not(target_os = "ios")))]
pub mod my_window {
    pub fn run() {
        compile_error!("Cannot enable two-show \"enable-desktop\" and \"enable-webbrowsers\" features! it's cannot to compiler!!");
    }
}
#[cfg(all(not(feature = "enable-desktop"), not(feature = "enable-webbrowser"), not(target_os = "android"), not(target_os = "ios")))]
pub mod my_window {
    pub fn run() {
        compile_error!("Cannot enable non-window features! it's cannot to compiler!!");
    }
}
#[cfg(all(not(feature = "enable-desktop"), feature = "enable-webbrowser", not(target_os = "android"), not(target_os = "ios")))]
pub mod my_window {
    use super::*;
    pub fn run() {
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();

        let callback = Arc::new(move |event| {
            match event {
                UserEvent::ExitWindow => {
                    let _ = tx2.send(UserEvent::ExitWindow);
                }
                _ => ()
            }
        });
        std::thread::spawn(move || {
            run_backend_server(tx1, callback);
        });
        if rx1.recv().unwrap() != UserEvent::Start {
            unreachable!()
        }
        log::info!("get backend signal, start init window!");
        let url = "http://localhost:8899";
        if let Err(e) = webbrowser::open(url) {
            log::error!("Cannot open default browser: {:?}，please open it manually: {}", e, url);
        } else {
            log::error!("Open browser success! service launch in {}", url)
        }
        if rx2.recv().unwrap() == UserEvent::ExitWindow {
            return;
        }
    }
}
#[cfg(any(all(feature = "enable-desktop", not(feature = "enable-webbrowser")), target_os = "android", target_os = "ios"))]
pub mod my_window {
    use super::*;
    use tao::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoopBuilder, EventLoopWindowTarget},
        window::{Window, WindowBuilder},
    };
    use wry::{WebView, WebViewBuilder};
    pub fn run() {
        let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
        let (tx, rx) = mpsc::channel();

        let proxy = event_loop.create_proxy();

        let exit_callback = Arc::new(move |event| {
            match event {
                UserEvent::ExitWindow => {
                    let _ = proxy.send_event(UserEvent::ExitWindow);
                }
                UserEvent::FullScreen => {
                    let _ = proxy.send_event(UserEvent::FullScreen);
                }
                UserEvent::ExitScreen => {
                    let _ = proxy.send_event(UserEvent::ExitScreen);
                }
                _ => ()
            }
        });

        std::thread::spawn(move || {
            run_backend_server(tx, exit_callback);
        });
        log::info!("get backend signal, start init window!");
        if rx.recv().unwrap() != UserEvent::Start {
            unreachable!();
        }
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
                Event::UserEvent(UserEvent::ExitWindow) => {
                    webview.take();
                    *control_flow = ControlFlow::Exit;
                }
                Event::UserEvent(UserEvent::FullScreen) => {
                    if let Some(webview) = webview.as_ref() {
                        webview.0.set_fullscreen(Some(tao::window::Fullscreen::Borderless(None)));
                    }
                }
                Event::UserEvent(UserEvent::ExitScreen) => {
                    if let Some(webview) = webview.as_ref() {
                        webview.0.set_fullscreen(None);
                    }
                }
                _ => (),
            }
        });
    }

    fn build_webview(
        event_loop: &EventLoopWindowTarget<UserEvent>,
    ) -> Result<(Window, WebView), Box<dyn std::error::Error>> {
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        let window = WindowBuilder::new()
            .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 720.0))
            .with_title(PACKAGE_NAME)
            .with_window_icon(
                if let Ok(img) = image::load_from_memory(WINDOW_ICON) {
                    let rgba = img
                        .resize(64, 64, image::imageops::FilterType::Nearest)
                        .to_rgba8();
                    tao::window::Icon::from_rgba(rgba.into_raw(), 64, 64).ok()
                } else {
                    None
                })
            .build(&event_loop)?;
        #[cfg(any(target_os = "android", target_os = "ios"))]
        let window = WindowBuilder::new()
            .with_title(PACKAGE_NAME)
            .build(&event_loop)?;
        let builder = WebViewBuilder::new()
            .with_background_color((0, 0, 0, 255))
            .with_url("http://localhost:8899");
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
}
