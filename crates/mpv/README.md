# MPV

tells bevy to render to a gif and uses [mpv](https://mpv.io/) to display it as the wallpaper.

to run this; first build the project in release mode (`cargo build -r`) then edit the `./bg.sh` shell script to your liking.

## Pros Vs Cons

Pros:

- compatible with the built in bar from [Qtile](https://qtile.org).
- compatible with polybar.

Cons:

- choppy movement.
- approx. 1-5 second delay between render and display.
- only supports one camera.

## Dependencies

- `xwinwrap` (from the xorg utils)
- [`mpv`](https://mpv.io/)
