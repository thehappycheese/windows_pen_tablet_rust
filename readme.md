# Rust Pen Tablet / Pen Display Input Capture

This workspace contains several crates (plus some extras, see further below)
with different attempts to capture input from my Huion Kamvas Pro 24 Pen
Display. Note all crates compile and run, but the first few in this list do not
currently retrieve pen pressure information for me.

1. `test_windows_pointer_api` (Does not work)
   - attempts to read
     [`WM_POINTERUPDATE` events](https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-pointerupdate)
     (part of the
     [Pointer Input Messages and Notifications](https://learn.microsoft.com/en-us/windows/win32/inputmsg/messages-and-notifications-portal)
     API) directly using the `windows` crate
   - The pointer input api of `Win32` (`Win32_UI_Input_Pointer` feature) simply
     does not do what it says in the docs; events are reported as an emulated
     mouse without pressure information, and any window that calls
     `EnableMouseInPointer` will experience frequent spikes in input delay, or
     short freezes in input making it unusable. (See this
     [reddit post](https://www.reddit.com/r/huion/comments/1bwjl7c/tablet_freezing_midway_drawing/))
     I think this is due to a lack of interest from Microsoft, but I also cannot
     confirm that the issue is not with Huion's driver. Microsoft is my prime
     suspect because I have seen this issues reported by users of all different
     tablet manufacturers.
2. `test_wintab_with_bindgen` (Does not work)
   - attempts to use the [`octotablet`](https://github.com/Fuzzyzilla/octotablet) crate to access pen events, but in a
     much more abstracted and idiomatic way. `winit` provides a much easier mechanisim to create a window.
   - `octotablet` currently only provides access via the
     [`RealTimeStylus` api](https://learn.microsoft.com/en-us/windows/win32/tablet/realtimestylus-reference)
     which is documented as "Legacy User Interaction Features - ... Windows 7
     and Earlier". However the author has indicated that they are [open to a PR](https://github.com/Fuzzyzilla/octotablet/issues/6#issuecomment-2046173357) to include wintab support.
3. `test_wintab_with_bindgen` (Works! Hooray?)
   - uses the wintab interface maintained by Wacom, using `bindgen` and `libloading`
   - `clang` must be installed and the `LIBCLANG_PATH` environment variable must
    be set
   - I have distributed the C headers listed below with this repo because I
     found them
     [here](https://github.com/Wacom-Developer/wacom-device-kit-windows/tree/881d8e8303e858e53584e70235fe32e3c9ef06f2/Wintab%20Pressure%20Test/SampleCode/Wintab)
     under an
     [MIT licence](https://github.com/Wacom-Developer/wacom-device-kit-windows/blob/881d8e8303e858e53584e70235fe32e3c9ef06f2/Wintab%20Pressure%20Test/SampleCode/MIT-license.txt)
     however please note that the headers themselves contain a
     [copyright notice](https://github.com/Wacom-Developer/wacom-device-kit-windows/blob/881d8e8303e858e53584e70235fe32e3c9ef06f2/Wintab%20Pressure%20Test/SampleCode/Wintab/WINTAB.H#L4C1-L10C81) ¯\\_(ツ)_/¯.
     - `MSGPACK.H`
     - `PKTDEF.H`
     - `WINTAB.H`
   - I don't know what the actual overall wintab license is. I cant find source
     code for the `wintab32.dll`? I suppose that makes it a proprietary thing?
   - My code does not close the context properly, so beware potential issues. I
     had a LOT of trouble getting it to that point.
4. `wintab_lite` (Work in Progress - But Will work!)
   - A types crate with some C-FFI compatible structs which are much
   nicer and better documented than what you get from bindgen.
5. `wintab_winit` (Work in Progress - But Will work!)
   - Tests the types defined in `wintab_lite` by trying to use them with `winit`


## Extra Packages

- `test_print_type_sizes` contains python notebooks and experiments used to
  understand `struct` memory layout using the experimental compiler flag
  `rustc -Z print-type-sizes`