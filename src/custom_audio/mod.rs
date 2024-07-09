use gst::glib;
use gst::prelude::*;

mod imp;

glib::wrapper! {
    pub struct CustomAudio(ObjectSubclass<imp::CustomAudio>) @extends gst_base::BaseTransform, gst::Element, gst::Object;
}

// Registers the type for our element, and then registers in GStreamer under
// the name "customaudio" for being able to instantiate it via e.g.
// gst::ElementFactory::make(), or
// gst-launch-1.0 audiotestsrc ! decodebin ! audioconvert ! audioresample ! customaudio ! autoaudiosink
pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "customaudio",
        gst::Rank::NONE,
        CustomAudio::static_type(),
    )
}
