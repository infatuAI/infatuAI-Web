# infatuAI Web

Rust bindings for Chromium integrated with Bevy the data-driven game engine via GStreamer (chroma key compositing). With 3d mode and comic book mode. Tested on Linux and Mac. With aspirations to add Windows support in the future.

## Linux x86_64

```
CEF_PATH=/path/to/cef_binary_88.1.4+g5a6ab23+chromium-88.0.4324.96_linux64 cargo build
```

## Mac x86_64

```
CEF_PATH=/path/to/cef_binary_88.2.8+ge484012+chromium-88.0.4324.150_macosx64 cargo build
```

## Tests (Linux x86_64)
```
cd cef-sys
CEF_PATH=/path/to/cef_binary_88.1.4+g5a6ab23+chromium-88.0.4324.96_linux64 cargo test

...

test bindgen_test_layout_timespec ... ok
test bindgen_test_layout_tm ... ok
test bindgen_test_layout__cef_screen_info_t ... ok
test bindgen_test_layout__cef_domnode_t ... ok

test result: ok. 143 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
