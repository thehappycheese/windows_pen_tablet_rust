# Rust Pen Tablet / Pen Display Input Capture

This workspace contains three binary crates (plus some extras, see further
below) with different attempts to capture input from my Huion Kamvas Pro 24 Pen
Display:

1. `windows_direct` (Does not work)
   - attempts to read
     [`WM_POINTERUPDATE` events](https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-pointerupdate)
     (part of the
     [Pointer Input Messages and Notifications](https://learn.microsoft.com/en-us/windows/win32/inputmsg/messages-and-notifications-portal)
     API) directly using the `windows` crate
2. `winit_and_octotab` (Does not work)
   - attempts to use the [`octotablet`](https://github.com/Fuzzyzilla/octotablet) crate to access pen events, but in a
     much more abstracted and idiomatic way. `winit` provides a much easier mechanisim to create a window.
   - `octotablet` only provides access via the
     [`RealTimeStylus` api](https://learn.microsoft.com/en-us/windows/win32/tablet/realtimestylus-reference)
     which is documented as "Legacy User Interaction Features - ... Windows 7
     and Earlier".
3. `wintab_test` (Works! Hooray?)
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
   - It doen't seem like a great situation from an open source perspective, which
     is possibly why no rust crate has yet created a wrapper? (e.g. `octotablet`
     has explicitly marked wintab support as "not planned")
   - Therefore it is also not my goal to publish a wrapper at this time.
   - My code does not close the context properly, so beware potential issues. I
     had a LOT of trouble getting it to that point.

Sadly the first two methods are just broken on windows 11 at the time of
writing. The only usable option is `wintab`.

The pointer input api of `Win32` (`Win32_UI_Input_Pointer` feature) simply does
not do what it says in the docs; events are reported as an emulated mouse
without pressure information, and any window that calls `EnableMouseInPointer`
will experience frequent spikes in input delay, or short freezes in input making
it unusable. (See this [reddit post](https://www.reddit.com/r/huion/comments/1bwjl7c/tablet_freezing_midway_drawing/))

I think this is due to a lack of interest from Microsoft, but I also cannot
confirm that the issue is not with Huion's driver. Microsoft is my prime suspect
because I have seen this issues reported by users of all different tablet
manufacturers.


## Extra Packages

- `test_print_type_sizes` contains python notebooks and experiments used to
  understand `struct` memory layout using the experimental compiler flag
  `rustc -Z print-type-sizes`