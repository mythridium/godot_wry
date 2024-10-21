use std::num::{NonZero, NonZeroIsize};
use std::ptr::NonNull;
use godot::classes::display_server::HandleType;
use godot::classes::DisplayServer;
use raw_window_handle::{AppKitWindowHandle, HandleError, HasWindowHandle, RawWindowHandle, Win32WindowHandle, WindowHandle};

pub struct GodotWindow;

impl HasWindowHandle for GodotWindow {
    #[cfg(target_os = "windows")]
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let display_server = DisplayServer::singleton();
        let window_handle = display_server.window_get_native_handle(HandleType::WINDOW_HANDLE);
        let non_zero_window_handle = NonZero::new(window_handle).expect("WindowHandle creation failed");
        unsafe {
            Ok(WindowHandle::borrow_raw(
                RawWindowHandle::Win32(Win32WindowHandle::new({
                    NonZeroIsize::try_from(non_zero_window_handle).expect("Invalid window_handle")
                }))
            ))
        }
    }

    #[cfg(target_os = "macos")]
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let display_server = DisplayServer::singleton();
        let window_handle = display_server.window_get_native_handle(HandleType::WINDOW_VIEW);
        unsafe {
            Ok(WindowHandle::borrow_raw(
                RawWindowHandle::AppKit(AppKitWindowHandle::new({
                    let ptr: *mut c_void = transmute(window_handle);
                    NonNull::new(ptr).expect("Id<T> should never be null")
                }))
            ))
        }
    }
}
