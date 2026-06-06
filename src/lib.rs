use rodio::{Decoder, Player, Source};
use softbuffer::{Context, Surface};
use std::io::Cursor;
use std::sync::Arc;
use tiny_skia::{Color, Pixmap, PixmapPaint, Transform};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Fullscreen, Window, WindowAttributes};

fn load_image_as_pixmap(bin: &[u8]) -> Option<Pixmap> {
    let img = image::load_from_memory(bin).ok()?.to_rgba8();
    let width = img.width();
    let height = img.height();
    let mut pixmap = Pixmap::new(width, height)?;
    pixmap.data_mut().copy_from_slice(img.as_raw());
    Some(pixmap)
}
const BGM_BYTES: &[u8] = include_bytes!("../assets/bgm.mp3");
const LOGO_BYTES: &[u8] = include_bytes!("../assets/logo.png");

struct App {
    window: Option<Arc<Window>>,
    context: Option<Context<Arc<Window>>>,
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
    is_fullscreen: bool,
    _audio_sink: rodio::MixerDeviceSink,
    _audio_player: Player,
    image_pixmap: Option<Pixmap>,
}
impl App {
    fn new() -> Self {
        let sink_handle = rodio::DeviceSinkBuilder::open_default_sink().expect("Error Code: 1");
        let cursor = Cursor::new(BGM_BYTES);
        let source = Decoder::new_mp3(cursor).expect("Error Code: 2");
        sink_handle.mixer().add(source.repeat_infinite());
        let player = rodio::Player::connect_new(&sink_handle.mixer());
        player.set_volume(0.5);
        player.play();
        let image_pixmap = load_image_as_pixmap(LOGO_BYTES);
        Self {
            window: None,
            context: None,
            surface: None,
            is_fullscreen: false,
            _audio_sink: sink_handle,
            _audio_player: player,
            image_pixmap,
        }
    }

    fn draw(&mut self) {
        let window = match self.window.as_ref() {
            Some(w) => w,
            None => return,
        };
        let surface = match self.surface.as_mut() {
            Some(s) => s,
            None => return,
        };
        let (width, height) = {
            let size = window.inner_size();
            (size.width, size.height)
        };

        if width == 0 || height == 0 {
            return;
        }

        surface
            .resize(
                std::num::NonZeroU32::new(width).unwrap(),
                std::num::NonZeroU32::new(height).unwrap(),
            )
            .unwrap();

        let mut canvas = Pixmap::new(width, height).unwrap();
        canvas.fill(Color::from_rgba8(30, 30, 50, 255));

        if let Some(img) = self.image_pixmap.as_ref() {
            let x = (width as i32 - img.width() as i32) / 2;
            let y = (height as i32 - img.height() as i32) / 2;
            canvas.draw_pixmap(
                x,
                y,
                img.as_ref(),
                &PixmapPaint::default(),
                Transform::identity(),
                None,
            );
        }
        let mut buffer = surface.buffer_mut().unwrap();
        let pixels = canvas.data();
        for (dst, src) in buffer.iter_mut().zip(pixels.chunks_exact(4)) {
            *dst = (src[0] as u32) << 16 | (src[1] as u32) << 8 | (src[2] as u32);
        }
        buffer.present().unwrap();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let attrs = WindowAttributes::default()
                .with_title("Pure Rust Cross-Platform")
                .with_inner_size(winit::dpi::LogicalSize::new(800, 600));

            let window = Arc::new(event_loop.create_window(attrs).unwrap());

            let context = Context::new(window.clone()).unwrap();
            let surface = Surface::new(&context, window.clone()).unwrap();

            self.window = Some(window);
            self.context = Some(context);
            self.surface = Some(surface);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                // self.audio_sink.stop();
                event_loop.exit();
            }
            WindowEvent::Resized(_) => {
                self.draw();
            }
            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        state,
                        logical_key,
                        physical_key,
                        ..
                    },
                ..
            } => {
                if state == ElementState::Pressed {
                    if physical_key
                        == winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyE)
                    {
                        self.is_fullscreen = !self.is_fullscreen;
                        let window = self.window.as_ref().unwrap();
                        if self.is_fullscreen {
                            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                        } else {
                            window.set_fullscreen(None);
                        }
                    }
                    if logical_key == winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape)
                    {
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                self.draw();
            }
            _ => (),
        }
    }
}

#[cfg(target_os = "android")]
fn init_logging() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Trace)
            .with_tag("[RenRs]"),
    );
}

#[cfg(not(target_os = "android"))]
fn init_logging() {
    simple_logger::SimpleLogger::new().init().unwrap();
}
#[cfg(not(target_os = "android"))]
pub fn desktop_main() {
    init_logging();
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
// ── Android entry point ──────────────────────────────────────────
#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;
#[cfg(target_os = "android")]
#[no_mangle]
pub fn android_main(app: AndroidApp) {
    init_logging();
    use winit::platform::android::EventLoopBuilderExtAndroid;
    let event_loop = EventLoop::builder().with_android_app(app).build().unwrap();
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}

// ── iOS entry point ─────────────────────────────────────────────
// Called from main.mm via FFI after UIApplication is set up.
#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn start_app() {
    init_logging();
    // On iOS, winit internally creates the UIWindow / UIViewController
    // from the UIApplication's key window.  No extra platform setup needed.
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
