# Example  `wintab_lite` with `winit` and `libloading`

IMPORTANT NOTES AND LIMITATIONS:

- This demo worked for me with my hardware, it does not strictly follow best practices
- I found that the default log context object was configured differently from
  what the documentation said on my system, so your milage may vary.

Other Notes:

- `winit` eats the actual native `wintab` events (e.g. `WT_PACKET`).
  - Luckliy `wintab` supports polling methods and keeps a nice timestamped event
    queue. Only needs access to the `hwnd` pointer. This is good news as it
    means it is likely-ish I can get this working in `bevy`, as long as the
    plugin lets me have the `hwnd` :P
- My tablet only reports button 1; the pen tip button. Other buttons are
  reported as keystrokes???! I think winit has misinterpreted something there.
  they should be mouse events. Perhaps I broke something in the event loop?

Usage

- `cargo run`
- Press `c` on the keyboard to clear the view.
- Only wintab input will cause anything to be drawn. Mouse won't do anything.

![screenshot](./readme_extras/screenshot.png)