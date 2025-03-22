# Bevy_winit

A fork of bevy_winit. Spawns a new window and embeds is as a child of the root window.

**NOTE:** programs made with this library *must* be launched before any other program that needs to draw infront of them. (ex. [glava](https://github.com/jarcode-foss/glava), [polybar](https://github.com/polybar/polybar), etc)

## Pros Vs Cons

Pros:

- Clean and smooth animation when compared to the MPV version.
- Itegrates tightly with bevy and bevys standard infrastructures. Therefor had grate support for thridparty bevy plugins.
- allows for multiple cameras.
- compatible with polybar.

Cons:

- partially compatible with the built in bar from [Qtile](https://qtile.org). (requires a config reload after the program gets launched to view the bar.)
