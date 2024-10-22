<h1 align="center">WebView Rendering Library for Godot</h1>

<p align="center">
  <a href="https://github.com/tauri-apps/wry">WRY</a> is a cross-platform webview rendering library. This extension allows you to use the native webview in Godot to build browsers and GUIs with HTML, CSS and JavaScript.
</p>

<p align="center">
  <a href="https://doceazedo.com">
    <img src="assets/screenshot.gif">
  </a>
</p>

## Download

> [!WARNING]  
> This extension is in active development and may be released soon.

## Getting started

After installing the extension, usage is as simple as adding a **WebView** node to your scene.

## Supported platforms

| Platform    | Support             | Web engine                 |
| ----------- | ------------------- | -------------------------- |
| **Windows** | ✅ Supported        | WebView 2 (Chromium)       |
| **Mac**     | ✅ Supported        | WebKit                     |
| **Linux**   | 🔄 Work in progress | WebKitGTK                  |
| **Android** | 🚧 Planned          | Android WebView (Chromium) |
| **iOS**     | 🚧 Planned          | WebKit                     |

### Windows

Transparency is not supported on Windows 7, and WebView2 support is limited to version 109 for Windows 7 and 8/8.1.

### Linux

WRY requires [WebKitGTK](https://webkitgtk.org). So you need to make sure it is properly installed beforehand.

### Android/iOS

WRY already has [mobile support](https://github.com/tauri-apps/wry?tab=readme-ov-file#android--ios). Contributions to implement Android and iOS are very welcome!

## Interop between WebView and Godot

Godot and the WebView can exchange messages with each other. This is useful for updating the UI data, or triggering game actions when interacting with the UI.

> 💡 **Example:** you can send a `play` message on a HTML button click, then Godot can listen for that message and start the game.

Sending messages from Godot to the WebView:

```py
$WebView.post_message("Hello from Godot!")
```

Sending messages from JavaScript to Godot:

```js
window.ipc.postMessage("Hello from JavaScript!");
```

Receiving messages in Godot using the `ipc_message` signal:

```py
func _on_web_view_ipc_message(message):
	print("Just got a message from the webview: %s" % message)
```

Receiving messages in JavaScript using an event listener:

```js
document.addEventListener("message", (event) => {
  console.log("Just got a message from Godot:");
  console.log(event.detail);
});
```

## Caveats

Since WRY utilizes the system's native webview for HTML rendering, the results may vary across different platforms, similar to how a website might appear or behave differently in Chrome versus Safari.

It's important to note that WRY renders the webview directly within the window. This prevents rendering on 3D meshes and customizing the rendering process.

If these limitations are significant for your use case, consider alternatives like [gdcef](https://github.com/Lecrapouille/gdcef) or [godot-webview](https://godotwebview.com/).

## TO-DO

> [!WARNING]  
> This extension is in active development, broken stuff is expected and contributions are appreciated!

- Load URLs from Godot project files via `res://`
- React to property changes (such as changing the URL, size and visibility)
- Implement JS evaluation
- Fix transparency when `full_window_size` is enabled
- Platform support

## License

The Godot WRY extension is license under MIT. WRY is licensed under Apache-2.0/MIT.
