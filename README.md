# Custom Audio Plugin for Gstreamer in Rust

Build with:

```bash
cargo build
```

Test with:
```bash
RUST_BACKTRACE=full GST_PLUGIN_PATH=./target/debug/ gst-launch-1.0 \
audiotestsrc ! customaudio ! autoaudiosink
```
