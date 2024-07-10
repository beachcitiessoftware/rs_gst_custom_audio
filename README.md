# Custom Audio Plugin for Gstreamer in Rust

Build a debug build with:

```bash
cargo build
```

### Build a release build with:

```bash
cargo build -r
```

### Test with `audiotestsrc`:
```bash
GST_DEBUG=customaudio:4 RUST_BACKTRACE=full GST_PLUGIN_PATH=./target/debug/ gst-launch-1.0 audiotestsrc ! customaudio ! autoaudiosink
```

```bash
GST_DEBUG=customaudio:4 RUST_BACKTRACE=full GST_PLUGIN_PATH=./target/debug/ gst-launch-1.0 audiotestsrc ! audioconvert ! audioresample ! audio/x-raw,format=S16LE,channels=1,rate=44100 ! customaudio ! decodebin ! audioconvert ! audioresample ! autoaudiosink
```

### Test with files:

```bash
GST_DEBUG=customaudio:4 RUST_BACKTRACE=full GST_PLUGIN_PATH=./target/debug/ gst-launch-1.0 filesrc location=./audio/testing.wav ! decodebin ! audioconvert ! audioresample ! audio/x-raw,format=S16LE,channels=1,rate=44100 ! customaudio ! audioconvert ! audioresample ! wavenc ! queue2 ! filesink location=./audio/converted.wav
```

```bash
GST_DEBUG=customaudio:4 RUST_BACKTRACE=full GST_PLUGIN_PATH=./target/debug/ gst-launch-1.0 filesrc location=./audio/testing.mp3 ! decodebin ! audioconvert ! audioresample ! audio/x-raw,format=S16LE,channels=1,rate=44100 ! customaudio ! audioconvert ! audioresample ! lamemp3enc ! queue2 ! filesink location=./audio/converted.mp3
```

```bash
GST_DEBUG=customaudio:4 RUST_BACKTRACE=full GST_PLUGIN_PATH=./target/debug/ gst-launch-1.0 filesrc location=./audio/testing.flac ! decodebin ! audioconvert ! audioresample ! audio/x-raw,format=S16LE,channels=1,rate=44100 ! customaudio ! audioconvert ! audioresample ! flacenc ! queue2 ! filesink location=./audio/converted.flac
```
