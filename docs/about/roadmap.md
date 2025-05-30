---
layout: page
title: Roadmap
---

<style>
.roadmap-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 48px 24px;
}

.roadmap-header {
  text-align: center;
  margin-bottom: 64px;
}

.roadmap-header a {
  color: var(--vp-c-brand-1);
  text-decoration: underline;
}

.roadmap-title {
  font-size: 48px;
  font-weight: 800;
  background-clip: text;
  margin: 0 0 24px 0;
}

.roadmap-subtitle {
  font-size: 20px;
  color: var(--vp-c-text-2);
  max-width: 600px;
  margin: 0 auto;
  line-height: 1.6;
}

@media (max-width: 768px) {
  .roadmap-container {
    padding: 32px 16px;
  }
  
  .roadmap-title {
    font-size: 36px;
  }
  
  .roadmap-subtitle {
    font-size: 18px;
  }
}
</style>

<div class="roadmap-container">
  <header class="roadmap-header">
    <h1 class="roadmap-title">🗺️ Roadmap</h1>
    <p class="roadmap-subtitle">
      We're actively working to get Godot WRY into a stable state. Have a feature request? <a href="https://github.com/doceazedo/godot_wry/issues/new?template=feature_request.md" target="_blank">Open an issue</a>.
    </p>
  </header>

<RoadmapSection 
    icon="🌟" 
    title="Active" 
    description="Currently in development. These features are being actively worked on.">
<RoadmapCard
      icon="🐧"
      title="Linux support"
      description="Partial Linux support is already in place, but still needs to support Wayland and transparency."
      issue-link="https://github.com/doceazedo/godot_wry/issues/17"
    />
</RoadmapSection>

<RoadmapSection 
    icon="⏳" 
    title="Planned" 
    description="Features we're planning to implement No set timeline. Contributions are welcomed.">
<RoadmapCard
      icon="📲"
      title="Android/iOS support"
      description="WRY already supports Android/iOS, so it should be possible for Godot WRY to implement this."
    />
<RoadmapCard
      icon="🌎"
      title="Browser support"
      description="You don't need a webview to render UI with HTML on a browser, of course. But we can make it easier for Godot WRY users to also export their games to browsers."
    />
<RoadmapCard
      icon="🔗"
      title="Bind JS ↔ GDScript variables"
      description="We can make a simple store contract that works with Svelte and RxJS (potentially more!) available so developers can enjoy variables that are always updated on the UI."
    />
</RoadmapSection>

<RoadmapSection 
    icon="🚀" 
    title="Launched" 
    description="Features that have been implemented and are alrady available in released versions.">
<RoadmapCard
      icon="🖱️"
      title="Forward mouse input events"
      description="Mouse events are now properly forwarded from the webview to GDScript, enabling interactive UI components."
      version="v0.0.6"
    />
<RoadmapCard
      icon="⛹️"
      title="Demos and examples"
      description="An example project is now available to help developers get a preview of how Godot WRY works."
      version="v0.0.4"
    />
<RoadmapCard
      icon="⚡"
      title="Expose WRY methods to GDScript"
      description="Core WRY functionality is now accessible through GDScript, providing the foundation for webview integration."
      version="v0.0.4"
    />
<RoadmapCard
      icon="⌨️"
      title="Forward keyboard input events"
      description="Keyboard events are properly forwarded from the webview to GDScript for handling user input."
      version="v0.0.3"
    />
</RoadmapSection>

</div>
