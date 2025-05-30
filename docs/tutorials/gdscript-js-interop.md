# Interoperability

In this tutorial, you will build a game HUD using HTML and JavaScript that communicates with your Godot game. We'll create a player status display that shows health, score, and inventory, with the ability to use items directly from the web interface.

## Setting up the foundation

First, you need to create the basic WebView setup and establish communication. Let's start with a simple HTML structure for the HUD.

```gdscript
func _ready():
  $WebView.connect("ipc_message", self, "_on_ipc_message")
  setup_hud()

func setup_hud():
  $WebView.load_html("""
    <div id="hud">
      <h3>Player Status</h3>
      <div id="health">Health: <span id="health-value">100</span></div>
      <div id="score">Score: <span id="score-value">0</span></div>
    </div>
  """)
```

We are connecting to the [`ipc_message`](/reference/webview#ipc-message) signal right away so we can handle messages from the web interface. The initial HTML gives you a basic structure to build upon.

## Updating player health

Now we'll add the ability to update the health display when the player takes damage or heals. We'll send health data from Godot to the web interface:

```gdscript
var player_health: int = 100

func take_damage(amount: int):
  player_health -= amount
  update_health_display()

func update_health_display():
  var message = {
    "action": "update_health",
    "health": player_health
  }
  $WebView.post_message(JSON.stringify(message))
```

The web interface needs to listen for these health updates. Let's add some JavaScript to handle the incoming messages:

```gdscript
func setup_hud():
  $WebView.load_html("""
    <div id="hud">
      <h3>Player Status</h3>
      <div id="health">Health: <span id="health-value">100</span></div>
      <div id="score">Score: <span id="score-value">0</span></div>
    </div>
    <script>
      document.addEventListener("message", (event) => {
        const data = JSON.parse(event.detail);
        switch (data.action) {
          case "update_health":
            document.getElementById("health-value").textContent = data.health;
            break;
        }
      });
    </script>
  """)
```

When you call `take_damage(25)`, the health display automatically updates to show the new value. The [`post_message()`](/reference/webview#post-message) method sends the data, and the JavaScript event listener handles it.

## Adding score tracking

Let's now expand the system to handle score updates...

```gdscript
var player_score: int = 0

func add_score(points: int):
  player_score += points
  update_score_display()

func update_score_display():
  var message = {
    "action": "update_score",
    "score": player_score
  }
  $WebView.post_message(JSON.stringify(message))
```

The JavaScript needs to handle score updates too. So let's also extend the message listener:

```javascript
document.addEventListener("message", (event) => {
  const data = JSON.parse(event.detail);
  switch (data.action) {
    case "update_health":
      document.getElementById("health-value").textContent = data.health;
      break;
    case "update_score":
      document.getElementById("score-value").textContent = data.score;
      break;
  }
});
```

Now when you call `add_score(100)`, both the game state and the web display will update simultaneously.

> [!TIP]
> Using switch statements instead of if/else chains makes the code more readable and maintainable when handling multiple message types. It's also more performant and makes it easier to add new message handlers later.

## Building an inventory system

Let's add an inventory display that shows available items. We can create an inventory array and send it to the web interface.

```gdscript
var inventory: Array = [
  {"id": "health_potion", "label": "Health Potion", "count": 3},
  {"id": "magic_scroll", "label": "Magic Scroll", "count": 1},
  {"id": "gold_coin", "label": "Gold Coin", "count": 50}
]

func update_inventory_display():
  var message = {
    "action": "update_inventory",
    "items": inventory
  }
  $WebView.post_message(JSON.stringify(message))
```

Let's expand the HTML to include an inventory section with clickable items.

```html
<div id="hud">
  <h3>Player Status</h3>
  <div id="health">Health: <span id="health-value">100</span></div>
  <div id="score">Score: <span id="score-value">0</span></div>
  <div id="inventory">
    <h4>Inventory</h4>
    <div id="inventory-items"></div>
  </div>
</div>
```

The JavaScript needs to create clickable inventory items dynamically. So let's add this functionality to the message handler:

```javascript
switch (data.action) {
  case "update_health":
    document.getElementById("health-value").textContent = data.health;
    break;
  case "update_score":
    document.getElementById("score-value").textContent = data.score;
    break;
  case "update_inventory":
    const container = document.getElementById("inventory-items");
    container.innerHTML = "";
    data.items.forEach((item, index) => {
      const button = document.createElement("button");
      button.textContent = `${item.label} (${item.count})`;
      button.onclick = () => useItem(item.id);
      container.appendChild(button);
    });
    break;
}

function useItem(itemId) {
  ipc.postMessage(
    JSON.stringify({
      action: "use_item",
      item_id: itemId,
    })
  );
}
```

Now the inventory displays as clickable buttons. When clicked, they send a message back to Godot using [`ipc.postMessage()`](/reference/javascript#ipc-postmessage).

> [!TIP]
> Using separate `id` and `label` fields for inventory items provides better structure and flexibility. The `id` serves as a stable identifier for game logic, while the `label` can be localized or changed for display purposes without breaking functionality.

## Handling item usage

Back in Godot, we need to handle the item usage messages from the web interface. So let's implement the message handler:

```gdscript
func _on_ipc_message(message):
  var data = JSON.parse_string(message)
  match data.action:
    "use_item":
      use_item(data.item_id)

func use_item(item_id: String):
  for i in range(inventory.size()):
    if inventory[i].id == item_id and inventory[i].count > 0:
      inventory[i].count -= 1

      match item_id:
        "health_potion":
          player_health = min(player_health + 25, 100)
          update_health_display()
        "magic_scroll":
          add_score(500)

      update_inventory_display()
      break
```

When a player clicks "Health Potion" in the web interface, it sends a message to Godot, which processes the item usage, updates the game state, and refreshes both the health and inventory displays.

## Polishing the interface

Let's add some CSS styling to make the HUD look a bit nicer :)

```html
<style>
  #hud {
    font-family: sans-serif;
    background-color: rgba(0, 0, 0, 0.8);
    color: #ffffff;
    padding: 20px;
    border-radius: 10px;
  }

  button {
    margin: 5px;
    padding: 8px 12px;
    background-color: #4a90e2;
    color: #ffffff;
    border: none;
    border-radius: 5px;
    cursor: pointer;
  }

  button:hover {
    background-color: #357abd;
  }
</style>
```

## That's all folks!

The final result is a fully functional game HUD that updates in real-time. When you call `take_damage(30)`, the health display immediately reflects the change. When you click inventory items, they're consumed and the interface updates accordingly.

This bidirectional communication pattern scales well for complex interfaces. You can add new message types easily by extending both the Godot message handler and the JavaScript event listener.
