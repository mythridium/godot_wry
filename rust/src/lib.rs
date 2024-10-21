use std::ffi::c_void;
use std::mem::{transmute};
use std::ptr::NonNull;
use godot::prelude::*;
use godot::classes::{DisplayServer, IDisplayServer, ISprite2D, Sprite2D};
use godot::classes::display_server::HandleType;
use raw_window_handle::{AppKitWindowHandle, HasRawWindowHandle, RawWindowHandle};
use wry::raw_window_handle::{HandleError, HasWindowHandle, WindowHandle};
use wry::{RGBA, WebViewBuilder, Rect};
use objc2_foundation::is_main_thread;
use wry::dpi::{LogicalPosition, LogicalSize};

struct GodotWRY;

#[gdextension]
unsafe impl ExtensionLibrary for GodotWRY {}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    angular_speed: f64,
    base: Base<Sprite2D>
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        // In GDScript, this would be:
        // rotation += angular_speed * delta

        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);
        // The 'rotate' method requires a f32,
        // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32
    }
}

#[derive(GodotClass)]
#[class(base=Node)]
struct WebView {
    base: Base<Node>,
    webview: Option<wry::WebView>,
}

#[godot_api]
impl INode for WebView {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Hello, webview renderer!");
        Self {
            base,
            webview: None,
        }
    }

    fn ready(&mut self) {
        godot_print!("webview is ready");
        let window = GodotWindow;
        let webview = WebViewBuilder::new()
            .with_url("https://github.com/doceazedo")
            .with_bounds(Rect {
                position: LogicalPosition::new(300, 100).into(),
                size: LogicalSize::new(640, 360).into(),
            })
            .with_transparent(true)
            .with_background_color(RGBA::from((0, 255, 0, 255)))
            .with_devtools(true)
            .build_as_child(&window)
            .unwrap();
        self.webview.replace(webview);
    }
}

struct GodotWindow;

impl HasWindowHandle for GodotWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let display_server = DisplayServer::singleton();
        let window_handle = display_server.window_get_native_handle(HandleType::WINDOW_VIEW);
        unsafe {
            Ok(WindowHandle::borrow_raw(
                RawWindowHandle::AppKit(AppKitWindowHandle::new({
                    assert!(is_main_thread(), "can only access AppKit handles on the main thread");
                    let ptr: *mut c_void = transmute(window_handle);
                    NonNull::new(ptr).expect("Id<T> should never be null")
                }))
            ))
        }
    }
}
