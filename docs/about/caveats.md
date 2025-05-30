# Caveats

Here are the main limitations you should know about before using Godot WRY.

## Webview always renders on top

The webview is rendered directly in the OS window as a native UI element, always appearing on top of your game content. You can't render it on 3D meshes or have game elements appear over it. Think of it as a desktop application overlay rather than an in-world UI element.

## Different browser engines across platforms

Since each platform uses its native webview (WebView2, WebKit, WebKitGTK), your web content may behave differently across Windows, macOS, and Linux. Test on all target platforms, especially when using newer web features.

## No automatic dependency checks

The extension doesn't verify or install required dependencies like WebKitGTK on Linux. You're responsible for ensuring users have the necessary libraries installed and handling missing dependencies gracefully.

We are open for handling this on Godot WRY's side, so contributions are welcomed.
