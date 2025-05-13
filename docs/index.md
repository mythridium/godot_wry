---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "Godot WRY"
  text: "Native webview extension for Godot 4"
  tagline: "Build GUIs and browsers with native system webviews in Godot"
  actions:
    - theme: brand
      text: Get started
      link: /guide/getting-started
    - theme: alt
      text: Godot Asset Library
      link: https://godotengine.org/asset-library/asset/3426
  image:
    src: /logo.svg
    alt: Godot WRY

features:
  - icon: 🍃
    title: Lightweight by design
    details: WRY uses the webview that is already available on the user's system, no extra dependencies.
  - icon: 🌎
    title: Websites and local files
    details: Load websites from an URL or serve local HTML, CSS and JS files using "res://".
  - icon: 🧩
    title: JavaScript interop
    details: Evaluate code from JavaScript and send messages to GDScript and vice-versa.
  - icon: 🚥
    title: Input events forwarding
    details: Mouse and keyboard inputs are forwarded to both the game and the webview.
---
