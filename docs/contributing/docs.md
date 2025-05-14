# Writing documentation

These are some basic instructions on how to write documentation for the Godot WRY project. These are suggestions rather than strict rules.

## Documentation structure

This documentation is powered by [VitePress](http://vitepress.dev/) and files are located in the `docs/` directory, the most relevant ones being:

```
docs/
├── .vitepress/      # VitePress files
│   └── config.mts   # VitePress configuration
│
├── public/          # Screenshots and other assets
│
├── about/           # Project information
├── contributing/    # Guides for contributors
├── guide/           # Tutorials
├── reference/       # API reference
│
└── index.md         # Homepage
```

## Markdown

All documentation is written in Markdown. If you are not familiar with it, the GitHub's [basic writing and formatting syntax](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax) guide is a great place to start.

## Screenshots

Screenshots, GIFs and videos are always great! And if you want to add one...

- Please make sure it's not too large. [Bulk Resize Photos](https://bulkresizephotos.com) is great for resizing and compressing images if you need to.
- Save the file in the **"docs/public/"** directory.
- If you save a file at **"docs/public/filename.png"**, you can reference it in Markdown like this: `![](/filename.png)`.

## Adding new documentation

Instructions for adding new content to the documentation:

### Where to add new content

If there's a page for what you want to add already, you might want to just create a new section on it.

If you need to create a new page, check the [structure](#documentation-structure) above and indentify where it should be added. You can also take a look at the docs sidebar and think where you think it should be.

### Create a new page

Create a new markdown file with a descriptive and short name. This is usually the page title in kebab-case. If the title is too long, you can use just the most important part of the time or some alternative word. Here are some examples:

- "Getting started" ⇒ **"/guide/getting-started.md"**
- "Writing documentation" ⇒ **"/contributing/docs.md"**
- "Building from source" ⇒ **"/contributing/compiling.md"**

You should also add you page to the sidebar. You can do that on the `themeConfig.sidebar` property inside the **"/docs/.vitepress/config.mts"** file.

### Style guidelines

Just add a title for your page and try to follow the style of other existing pages.

### Internal linking

It's very important and super useful to refer to other pages of the documentation so the reader always has somewhere to go if they need more information on a topic.

- If your page mentions another method, signal, guide, etc., **add a link**.
- If another page mentions the content related to your page, **add a link**.
- If another page would benefit from the information that you added, instead of repeating yourself, **add a link**.

## API reference

The API reference pages are maintained manually. They should maintain the exact same format as they currently have, and should always stay up to date to the latest changes.

## Building and testing docs

To preview documentation locally, you will need to install [Node.js](https://nodejs.org/en). Then, inside the **"/docs"** directory, install the dependencies:

```bash
npm install
```

Then start VitePress:

```bash
npm run docs:dev
```

The documentation will be available at http://localhost:5173 by default, and will hot reload your changes.

## Documentation changes

Before submitting any changes to the documentation:

1. Make sure anything you write is technical and easy to understand.
2. Check for spelling and grammar.
3. Test any commands or code examples you add.
4. Avoid making assumptions about the reader's knowledge. It's a good idea to add context, links and/or requirements before going into a topic.

[![Cartoon panel showing two characters: "Silicate chemistry is second nature to us geochemists, so it's easy to forget that the average person probably only knows the formulas for olivine and one or two feldspars.", "And quartz, of course.", "Of course.". Caption below the panel: "Even when they're trying to compensate for it, experts in anything wildly overestimate the average person's familiarity with their field."](https://imgs.xkcd.com/comics/average_familiarity.png)](https://xkcd.com/2501)
