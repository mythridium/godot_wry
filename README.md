<h1 align="center">WebView Rendering Library for Godot</h1>

<p align="center">
  <a href="https://github.com/tauri-apps/wry">WRY</a> is a cross-platform webview rendering library. This extension allows you to use the native webview in Godot to build browsers and GUIs with HTML, CSS and JavaScript.
</p>

<p align="center">
  <a href="https://doceazedo.com">
    <img src="assets/screenshot-7.png">
  </a>
</p>

## 📥 Download

You can download the extension directly on the [Godot Asset Library](https://godotengine.org/asset-library/asset/3426) or by navigating to the "AssetLib" tab in the editor and searching for "wry".

Alternatively, you can go to the [Releases](https://github.com/doceazedo/godot_wry/releases) page, download the latest ZIP (_not_ the source code) and import it manually into your project.

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

## 🧰 Getting started

After installing the extension, you will now be able to see the **WebView** node inside `Node → CanvasItem → Control` when creating a new node. You can edit it's properties and layout as you wish.

| ![](assets/create-new-node.png)                       | ![](assets/inspector.png)                      |
| ----------------------------------------------------- | ---------------------------------------------- |
| <p align="center"><i>"Create new node" window</i></p> | <p align="center"><i>WebView inspector</i></p> |

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
