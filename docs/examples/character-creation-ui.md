# Character creation UI

This demo shows off a simple character creator, similar to what you'd find in Animal Crossing. It's a perfect example of how Godot WRY can handle UI and communicate between your web UI and your Godot scenes.

The idea is simple: use a 3D character model in Godot for the visual representation, and build the customization interface as a web app.

<iframe style="width:100%;aspect-ratio:16/9" src="https://www.youtube-nocookie.com/embed/cqKBvl5a1-o?si=PM7I2T4YPH4q6Fvv&autoplay=1&mute=1&loop=1&playlist=cqKBvl5a1-o" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

## Setting up the scene

The project structure is straightforward - a Godot scene with the character and a separate web application for the interface:

```
character_creator_ui_demo/
├── character_creator.gd      # Main Godot script
├── character_creator.tscn    # 3D scene with character
└── ui/                       # Web app
```

Each clothing item is a separate mesh. Switching items means hiding one mesh and showing another so we can organize the character parts to easily control them later:

```gdscript
@onready var body_material = $"Character/[...]/ACNHBody_001".get_active_material(0)
@onready var hair_meshes = {
  "short": $"Character/[...]/Hair 01",
  "long": $"Character/[...]/Hair_02"
}
# ...
```

## Setting up the WebView

This project uses the [WebView](/reference/webview) node provided by Godot WRY to overlay an webview over the game window so we can build our UI.

```gdscript
$WebView.full_window_size = true
$WebView.transparent = true
```

Setting `full_window_size = true` makes the webview ignore transforms and take the full viewport size, perfect for UIs.

## Building the web interface

This example uses **SvelteKit** for the web UI, but you can use any framework of your choice (React, Vue, vanilla JavaScript)... whatever you're comfortable with. The core concepts remain the same regardless of your choice.

The UI shows tabs for each customization categories. Each tab has an icon, a label, items (these could be different clothes or haircuts), and so on. Those are defined by the `Tab` type, following this structure:

```typescript
type Tab = {
  id: string;
  label: string;
  icon: Component;
  items?: (string | null)[];
  colors?: string[];
  selectedItem?: string | null;
  selectedColor?: string | null;
};
```

The `null` option represents "no item", useful for optional accessories or going shirtless. For example, this is how the "Shoes" tab looks like in code:

```typescript
{
  id: 'shoes',
  label: 'Shoes',
  icon: Sneaker,
  items: [null, 'shoes', 'rain_boots'],
  colors: ['#1D1616', '#F2F9FF', /* ... */],
  selectedItem: 'shoes',
  selectedColor: '#1D1616'
}
```

We can use this data to display the items in the UI and to be able to tell our Godot scene what items we want to wear.

## Communication between web and Godot

The communication layer is straightforward. From JavaScript, you can send messages using the global [`window.ipc`](/reference/javascript#ipc-postmessage) object.

Let's create a custom function that simplifies this for us by always sending a message that contains the `type` of the message and some data:

```typescript
export const postMessage = (type: string, body: object = {}) => {
  window.ipc.postMessage(
    JSON.stringify({
      type,
      ...body,
    })
  );
};
```

Then, when a user selects something, we can trigger message. For instance, if the user picks a different haircut, we post a `set_hair` message with the haircut mesh name as `item`:

```svelte
<button onclick={() => {
  activeTab.selectedItem = item;
  postMessage(`set_${activeTab.id}`, { item });
}}>
```

This would post a message like this:

```json
{
  "type": "set_hair",
  "item": "long"
}
```

On the Godot side, we can connect to the [`ipc_message`](/reference/webview#ipc-message) signal and handle the updates:

```gdscript
func _on_web_view_ipc_message(message):
  var data = JSON.parse_string(message)

  match data.type:
    "set_hair":
      # show selected hair, hide others
      for id in hair_meshes:
        var mesh = hair_meshes[id]
        mesh.visible = id == data.item

    "set_color_skin":
      # update skin color
      body_material.albedo_color = Color(data.color)

    # ...
```

> [!TIP]
> See how useful sending JSON strings is? This way we can always tell the message type and handle it accordingly. Any string format would work, but JSON provides a well-structured way of doing so.

## Running the demo

This demo is included with Godot WRY. Just [install Godot WRY](/guide/getting-started#installing) in your project, open `character_creator.tscn` and run the scene.

If you want to modify the web interface, you can rebuild it like so:

```bash
cd ui
npm install
npm run build
```

> [!TIP]
> The WebView node is set up to load the static build files. You can also run `npm run dev` and change the URL property to your localhost to see your changes in real time.

Complete source code: [`examples/character_creator_ui_demo`](https://github.com/doceazedo/godot_wry/tree/main/godot/addons/godot_wry/examples/character_creator_ui_demo)
