---
outline: [2, 3]
---

# WebView

The fundamental `Control` node to present a webview.

## Properties

| Property             | Type       | Description                                                                                               |
| -------------------- | ---------- | --------------------------------------------------------------------------------------------------------- |
| full_window_size     | bool       | Webview will always be the same size as the viewport.                                                     |
| url                  | String     | Initial URL to be loaded. This will override `html`.                                                      |
| html                 | String     | HTML string to be loaded. This will be ignored if `url` is provided.                                      |
| transparent          | bool       | Webview should be transparent.                                                                            |
| background_color     | Color      | **🚧 Not implemented.** Webview background color. This will be ignored if `transparent` is set to `true`. |
| devtools             | bool       | Enables web inspector. To open it, you can call `open_devtools()`, or right click the page and open it.   |
| headers              | Dictionary | **🚧 Not implemented.** Headers used when loading the requested URL.                                      |
| user_agent           | String     | Custom user agent header.                                                                                 |
| zoom_hotkeys         | bool       | Enables page zooming hotkeys.                                                                             |
| clipboard            | bool       | Enables clipboard access on **Linux** and **Windows**. Always enabled on macOS.                           |
| incognito            | bool       | Run the webview with incognito mode.                                                                      |
| focused_when_created | bool       | Webview will be focused when created.                                                                     |
| forward_input_events | bool       | Mouse and keyboard events captured by the webview will be propagated to the game.                         |

## Methods

> [!TIP]
> This implementation wraps WRY's [`WebView`](https://docs.rs/wry/latest/wry/struct.WebView.html) interface. Not all WRY features may be implemented but [contributions are always welcome](/contributing/how-to-contribute)!

### clear_all_browsing_data()

Clears all browsing data (such as cookies, cache, and local storage).

<a class="button" href="https://docs.rs/wry/latest/wry/struct.WebView.html#method.clear_all_browsing_data" target="_blank">WRY Documentation</a>

#### API

```py
func clear_all_browsing_data() -> void:
```

**Returns:** `void`

### eval(...)

Evaluate and run JavaScript code.

<a class="button" href="https://docs.rs/wry/latest/wry/struct.WebView.html#method.evaluate_script" target="_blank">WRY Documentation</a>

#### Example

```py
$WebView.eval("console.log(Math.PI)")
```

This will print `3.141592653589793` to the DevTools.

##### Retrieving result

You can call JavaScript code (including asynchronous) and listen to the [`ipc_message`](#ipc-message) signal to retrieve the result:

```py
func _on_button_pressed() -> void:
	$WebView.eval("
		const resp = await fetch('https://httpbin.org/ip');
		const data = await resp.json();
		ipc.postMessage(JSON.stringify({
			type: "my-ip",
			data
		}));
	")

func _on_web_view_ipc_message(message: String) -> void:
	var data = JSON.parse_string(message)
	if data.type == "my-ip":
		print("Your IP address is: %s" % data.origin)
```

We include an extra `type` field in the message payload to explicitly identify the kind of message being sent. This allows us to distinguish between different messages and handle them accordingly.

#### API

```py
func eval(js: String) -> void:
```

| Parameter | Type   | Description                      |
| --------- | ------ | -------------------------------- |
| js        | String | JavaScript code to be evaluated. |

**Returns:** `void`

### focus()

Tries moving focus to the webview, making it the active element that will receive keyboard and mouse input events.

<a class="button" href="https://docs.rs/wry/latest/wry/struct.WebView.html#method.focus" target="_blank">WRY Documentation</a>

#### API

```py
func focus() -> void:
```

**Returns:** `void`

### focus_parent()

Tries moving focus away from the webview back to the parent window.

<a class="button" href="https://docs.rs/wry/latest/wry/struct.WebView.html#method.focus_parent" target="_blank">WRY Documentation</a>

#### API

```py
func focus_parent() -> void:
```

**Returns:** `void`

### is_devtools_open()

Returns if the developer tools window is currently open.

<a class="button" href="https://docs.rs/wry/latest/wry/struct.WebView.html#method.is_devtools_open" target="_blank">WRY Documentation</a>

#### API

```py
func is_devtools_open() -> bool:
```

**Returns:** `bool`

### load_html(...)

Load HTML content into the webview.

<a class="button" href="https://docs.rs/wry/latest/wry/struct.WebView.html#method.load_html" target="_blank">WRY Documentation</a>

#### API

```py
func load_html(html: String) -> void:
```

| Parameter | Type   | Description               |
| --------- | ------ | ------------------------- |
| html      | String | The HTML content to load. |

**Returns:** `void`

### load_url(...)

Navigate to the specified URL.

<a class="button" href="https://docs.rs/wry/latest/wry/struct.WebView.html#method.load_url" target="_blank">WRY Documentation</a>

#### API

```py
func load_url(url: String) -> void:
```

| Parameter | Type   | Description                     |
| --------- | ------ | ------------------------------- |
| url       | String | The URL to load in the webview. |

**Returns:** `void`

### open_devtools()

Open the webview's web inspector (usually called DevTools). Only works if the `devtools` property is enabled.

<a class="button" href="https://docs.rs/wry/latest/wry/struct.WebView.html#method.open_devtools" target="_blank">WRY Documentation</a>

#### API

```py
func open_devtools() -> void:
```

**Returns:** `void`

### close_devtools()

Closes the webview's web inspector (usually called DevTools) if it's open.

<a class="button" href="https://docs.rs/wry/latest/wry/struct.WebView.html#method.close_devtools" target="_blank">WRY Documentation</a>

#### API

```py
func close_devtools() -> void:
```

**Returns:** `void`

### print()

Opens a dialog to print the current webview content.

<a class="button" href="https://docs.rs/wry/latest/wry/struct.WebView.html#method.print" target="_blank">WRY Documentation</a>

#### API

```py
func print() -> void:
```

**Returns:** `void`

### set_visible(...)

Shows or hides the webview.

<a class="button" href="https://docs.rs/wry/latest/wry/struct.WebView.html#method.set_visible" target="_blank">WRY Documentation</a>

#### API

```py
func set_visible(visible: bool) -> void:
```

| Parameter | Type | Description                            |
| --------- | ---- | -------------------------------------- |
| visible   | bool | Whether the webview should be visible. |

**Returns:** `void`

#### API

```py
func url() -> String:
```

**Returns:** `String`

### zoom(...)

Changes the zoom level of the page.

<a class="button" href="https://docs.rs/wry/latest/wry/struct.WebView.html#method.zoom" target="_blank">WRY Documentation</a>

#### Example

```py
# Set zoom to 150%
$WebView.zoom(1.5)

# Reset to default zoom
$WebView.zoom(1.0)
```

#### API

```py
func zoom(factor: float) -> void:
```

| Parameter | Type  | Description                       |
| --------- | ----- | --------------------------------- |
| factor    | float | The zoom factor (1.0 is default). |

**Returns:** `void`

## Signals

### ipc_message(...)

Emitted when JavaScript code in the WebView sends a message using `ipc.postMessage()`. This allows communication between JavaScript and your Godot game.

#### Example

```py
func _ready() -> void:
	$WebView.connect("ipc_message", self, "_on_ipc_message")
	$WebView.load_html('
		<button onclick="sendMessage()">Send message</button>
		<script>
			function sendMessage() {
				ipc.postMessage(JSON.stringify({
					type: "button_clicked",
					timestamp: new Date.now()
				}));
			}
		</script>
	')

func _on_ipc_message(message: String) -> void:
	var data = JSON.parse_string(message)
	if data.type == "button_clicked":
		print("Button was clicked at: %s" % data.timestamp)
```

#### API

```py
signal ipc_message(message: String)
```

| Parameter | Type   | Description                                     |
| --------- | ------ | ----------------------------------------------- |
| message   | String | The message sent from the WebView's JavaScript. |
