# bevy-linux-wallpaper

This project explores using the Bevy game engine as an animated wallpaper under on a linux system running X11.

## Crates

| **crate** | **Path** | **Description** |
|-----------|----------|-----------------|
| `mpv`     | `crates/mpv/`| attempt 1. renders bevy to a gif and displays said gif using [mpv](https://mpv.io/) on the root window. |
| `bevy_winit` | `crates/bevy_winit/` | attempt 2. a fork of bevy_winit. spawns a new window and embeds is as a child of the root window. this give the best results. |
| `winit` | `crates/winit/` | attempt 3. a fork of winit designed to render directly to the root window. does not yet work. | 

see each crates readme for more info.

## Setup

```toml
[dependencies]
bevy_wallpaper = { git="https://github.com/calacuda/bevy-linux-wallpaper", branch="main" }
```

## Usage 

```rust
let mut wp_plug = WallpaperPlugin::<WakeUp>::default();
wp_plug.run_on_any_thread = true;

App::new()
    .add_plugins((
        DefaultPlugins
            .set(LogPlugin {
                level: Level::INFO,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoVsync,
                    name: Some("wallpaper".into()),
                    window_level: WindowLevel::AlwaysOnBottom,
                    mode: WindowMode::Windowed,
                    skip_taskbar: false,
                    titlebar_shown: false,
                    resolution: WindowResolution::new(1920., 1080.),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .disable::<PipelinedRenderingPlugin>()
            .disable::<AccessibilityPlugin>()
            .disable::<AudioPlugin>()
            .disable::<WinitPlugin>(),
        WireframePlugin,
        wp_plug, 
    ))
```

## Versioning

| Bevy Version | bevy_wallpaper |
|-|-|
| `0.15` | `0.1` |
