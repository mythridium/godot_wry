# Roadmap

We're currently working to get Godot WRY into a stable state.

## 🌟 Active

- **🐧 Linux support**  
  The Linux implementation is already in place, but needs to be tested and adjusted accordingly.
- **⛹️ Demos and examples**
- **🖱️ Forward mouse input events**

## ⏳ Planned

- **📲 Android/iOS support**  
  WRY already supports Android/iOS, so it should be possible for Godot WRY to implement this.
- **🌎 Browser support**  
  You don't need a webview to render UI with HTML on a browser, of course. But we can make it easier for Godot WRY users to also export their games to browsers.
- **🔗 Bind JS ⇔ GDScript variables**  
  We can make a [simple store contract](https://svelte.dev/docs/svelte/stores#Store-contract) that works with Svelte and RxJS (potentially [more](https://www.reddit.com/r/sveltejs/comments/158oft0/use_any_state_management_library_with_svelte/)!) available so developers can enjoy variables that are always updated on the UI.

## 🚀 Launched

- **⚡ Expose WRY methods to GDScript** (`0.0.4`)
- **⌨️ Forward keyboard input events** (`0.0.3`)
