# Test `wintab_lite` with `winit`

The annoying thing is that `winit` eats the actual native wintab window events. Luckliy wintab supports polling methods and keeps a nice timestamped event queue. Only needs access to the `hwnd` pointer. This is good news as it means it is likely-ish I can get this working in `bevy`, as long as the plugin lets me have the `hwnd` :P

> Note: at the time of writing this compiles and runs but outputs garbage
> packets. I am working to fix the alignment of the structs to be compatible
> with the original c library.