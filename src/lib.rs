
use gst::plugin_define;
use gst::glib;


plugin_define!(
    rsgstcustomaudio,
    env!("CARGO_PKG_DESCRIPTION"),
    plugin_init,
    concat!(env!("CARGO_PKG_VERSION"), "-", env!("COMMIT_ID")),
    "MIT/X11",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_REPOSITORY"),
    env!("BUILD_REL_DATE")
);

mod custom_audio;

fn plugin_init(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    custom_audio::register(plugin)?;
    Ok(())
}
