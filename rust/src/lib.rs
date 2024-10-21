mod godot_window;

use godot::prelude::*;
use godot::classes::{Control, IControl, IDisplayServer, ISprite2D, Sprite2D};
use wry::{RGBA, WebViewBuilder, Rect, WebViewAttributes};
use wry::dpi::{LogicalPosition, LogicalSize};
use wry::http::{HeaderMap, Request};
use crate::godot_window::GodotWindow;

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
#[class(base=Control)]
struct WebView {
    base: Base<Control>,
    webview: Option<wry::WebView>,
    #[export]
    full_window_size: bool,
    #[export]
    url: GString,
    #[export]
    html: GString,
    #[export]
    transparent: bool,
    #[export]
    background_color: Color,
    #[export]
    devtools: bool,
    #[export]
    headers: Dictionary,
    #[export]
    user_agent: GString,
    #[export]
    zoom_hotkeys: bool,
    #[export]
    clipboard: bool,
    #[export]
    incognito: bool,
    #[export]
    focused: bool,
}

#[godot_api]
impl IControl for WebView {
    fn init(base: Base<Control>) -> Self {
        godot_print!("Hello, webview renderer!");
        Self {
            base,
            webview: None,
            full_window_size: true,
            url: "https://github.com/doceazedo/godot_wry".into(),
            html: "".into(),
            transparent: false,
            background_color: Color::from_rgb(1.0, 1.0, 1.0),
            devtools: true,
            headers: Dictionary::new(),
            user_agent: "".into(),
            zoom_hotkeys: false,
            clipboard: true,
            incognito: false,
            focused: true,
        }
    }

    fn ready(&mut self) {
        let window = GodotWindow;
        let base = self.base().clone();
        let webview_builder = WebViewBuilder::with_attributes(WebViewAttributes {
            url: if self.html.is_empty() { Some(String::from(&self.url)) } else { None },
            html: if self.url.is_empty() { Some(String::from(&self.html)) } else { None },
            transparent: self.transparent,
            devtools: self.devtools,
            // headers: Some(HeaderMap::try_from(self.headers.iter_shared().typed::<GString, Variant>()).unwrap_or_default()),
            user_agent: Some(String::from(&self.user_agent)),
            zoom_hotkeys_enabled: self.zoom_hotkeys,
            clipboard: self.clipboard,
            incognito: self.incognito,
            focused: self.focused,
            bounds: if !self.full_window_size {
                let rect = self.base().get_global_rect();
                Option::from(Rect {
                    position: LogicalPosition::new(rect.position.x, rect.position.y).into(),
                    size: LogicalSize::new(rect.size.x, rect.size.y).into(),
                })
            } else { None },
            ..Default::default()
        }).with_ipc_handler(move |req: Request<String>| {
            let body = req.body().as_str();
            base.clone().emit_signal("ipc_message".into(), &[body.to_variant()]);
        });

        if !self.url.is_empty() && !self.html.is_empty() {
            godot_error!("You have entered both a URL and HTML code. You may only enter one at a time.")
        }

        let webview = if self.full_window_size {
            webview_builder.build(&window).unwrap()
        } else {
            webview_builder.build_as_child(&window).unwrap()
        };

        self.webview.replace(webview);
    }
}

#[godot_api]
impl WebView {
    #[signal]
    fn ipc_message(message: GString);

    #[func]
    fn post_message(&self, message: GString) {
        if let Some(webview) = &self.webview {
            let message = str::replace(&*String::from(message), "'", "\\'");
            let script = str::replace("document.dispatchEvent(new CustomEvent('message', { detail: '{}' }))", "{}", &*message);
            let _ = webview.evaluate_script(&*script);
        }
    }
}
