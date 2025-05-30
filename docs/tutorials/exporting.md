# Exporting your project

When building a game with Godot WRY, you'll often have HTML, CSS, and JavaScript files that need to be included in your exported project. By default, Godot only exports recognized resource files, so we need to configure it to include our web assets.

## Including static web files

Open your project's export settings by going to **Project → Export**. Select your export preset and navigate to the **Resources** tab.

In the **Filters to export non-resource files/folders** field, you need to add patterns for the files you want to include, for instance:

```
*.html, *.css, *.js, build/*
```

You can use wildcards (`*`) to match multiple files or specify entire directories.

## Common pattern for web projects

If you are using a web framework that builds static files, just make sure to include your **"build"** folder in your export filters. The directory name and location will depend on your framework, of course.

Considering you have a [Vite](https://vite.dev/) app at "ui/my-cool-hud" you can export all your build files like so:

```
ui/my-cool-hud/build/**/**.*
```
