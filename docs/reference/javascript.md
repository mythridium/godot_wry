# JavaScript

API available on the web content rendered by the `WebView`.

## Methods

### ipc.postMessage(...)

Sends a message from the JavaScript to Godot. The message will be received as a [`ipc_message`](/reference/webview#ipc-message) signal in Godot.

> [!TIP]
> Using JSON strings are convenient for structured data and handling multiple messages.

#### Example

Sending a simple message:

```js
ipc.postMessage("Hello from JavaScript!");
```

Sending JSON:

```js
ipc.postMessage(
  JSON.stringify({
    type: "user_action",
    action: "login",
    username: "player1",
  })
);
```

#### API

```ts
function window.ipc.postMessage(message: string);
```

| Parameter | Type   | Description                      |
| --------- | ------ | -------------------------------- |
| message   | String | The message to be sent to Godot. |

**Returns:** `void`

## Events

### message

Triggered when Godot sends a message to the web content using [`post_message()`](/reference/webview#post_message).

#### Example

Handling a simple message:

```js
document.addEventListener("message", (event) => {
  console.log("Received message from Godot:", event.detail);
});
```

Handling structured messages:

```js
document.addEventListener("message", (event) => {
  const data = JSON.parse(event.detail);
  switch (data.action) {
    case "update_health":
      // TODO: updateHealthBar(data.health);
      break;
    case "show_notification":
      // TODO: showNotification(data.message, data.type);
      break;
  }
});
```

#### API

```ts
type GodotMessageCallback = (event: { detail: string }) => void;

document.addEventListener("message", callback);
```

| Parameter | Type   | Description                      |
| --------- | ------ | -------------------------------- |
| detail    | String | The message received from Godot. |

**Returns:** `void`
