# What is Godot WRY?

WRY is a cross-platform library for rendering webviews. This extension integrates WRY into Godot, allowing you to embed web content directly within your Godot game/application.

Unlike frameworks like CEF (Chromium Embedded Framework), which require bundling a full browser engine with your project, WRY uses the built-in webview provided by the user's operating system. This reduces your game size and lowers memory and CPU usage at runtime.

## Supported platforms

| Platform                        | Support                                                                 | Web engine                 |
| ------------------------------- | ----------------------------------------------------------------------- | -------------------------- |
| **Windows (10, 11)**            | ✅ Supported                                                            | WebView2 (Chromium)        |
| **Mac (Intel, Apple Sillicon)** | ✅ Supported                                                            | WebKit                     |
| **Linux**                       | 🚧 [Work in progress](https://github.com/doceazedo/godot_wry/issues/17) | WebKitGTK                  |
| **Android**                     | ⏳ Planned                                                              | Android WebView (Chromium) |
| **iOS**                         | ⏳ Planned                                                              | WebKit                     |
| **Browser/HTML5**               | ⏳ Planned                                                              | —                          |

### Linux

[WebKitGTK](https://webkitgtk.org) is required for WRY to function on Linux. The package name may differ based on the operating system and Linux distribution.

### Android/iOS

WRY itself already has [mobile support](https://github.com/tauri-apps/wry/blob/dev/MOBILE.md). Contributions to add Android/iOS support in this extension are welcome!

## Use cases

- **UI rendering**  
  You can use any JavaScript framework to build UIs (e.g., inventories, leaderboards, shops). It enables to iterate fast and to integrate with existing web toolchains.
- **Authentication**  
  Embed authentication flows (e.g., an existing login page or a third-party OAuth2 page) using a webview to handle login and registration.
- **In-game news**  
  Serve real-time news, patch notes, or event banners via remotely hosted web content, without the need for client updates.
- **Payment**  
  Load payment pages (e.g., Stripe, PayPal) within the game, similar to how games such as Fortnite and Valorant do.

## Performance

Godot's native UI system is inherently more performant than rendering UI through a webview.

However, when comparing WRY to other embedded webview frameworks (e.g., Chromium), WRY typically results in smaller binary sizes since it doesn't bundle any runtime dependencies.

Runtime performance is also usually improved as the game doesn't have the cost of initializing and managing a separate browser engine instance.
