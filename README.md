# bevy-linux-wallpaper

This project explores using the Bevy game engine as an animated wallpaper under on a linux system running X11.

## Crates

| **crate** | **Path** | **Description** |
|-----------|----------|-----------------|
| `mpv`     | `crates/mpv/`| attempt 1. renders bevy to a gif and displays said gif using MPV on the root window. |
| `bevy_winit` | `crates/bevy_winit/` | attempt 2. a fork of bevy_winit. spawns a new window and embeds is as a child of the root window. this give the best results. |
| `winit` | `crates/winit/` | attempt 3. a fork of winit designed to render directly to the root window. does not yet work. | 

see each crates readme for more info.
