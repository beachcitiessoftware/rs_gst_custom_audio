use std::env;
use std::sync::Mutex;
use std::fmt;
use hex;
use gst::glib;
use gst::prelude::*;
use gst_video::subclass::prelude::*;
use dotenv::dotenv;
use gst_audio::subclass::prelude::AudioFilterImpl;
use gst_base::subclass::BaseTransformMode;
use once_cell::sync::Lazy;
use reqwest::blocking;
use reqwest::blocking::multipart;
use tokio::runtime::Runtime;
use tracing::{span, info, warn, error, debug, trace, Level};
use tracing_subscriber;

/// The default trace level
const TRACE_LEVEL: Level = Level::INFO;

/// DEBUG category for GStreamer
static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "customaudio",
        gst::DebugColorFlags::empty(),
        Some("Rust Custom Audio Filter"),
    )
});

/// CustomAudio plugin for GStreamer
pub struct CustomAudio {
    processed_count: Mutex<i32>,
    process_audio_endpoint: String,
}

impl Default for CustomAudio {
    fn default() -> Self {
        tracing_subscriber::fmt()
            .with_max_level(TRACE_LEVEL)
            .init();
        dotenv().ok();

        let process_audio_endpoint = if let Ok(endpoint) = env::var("PROCESS_AUDIO_ENDPOINT") {
            gst::info!(
                    CAT,
                    "Audio samples will be sent to: {}",
                    endpoint
                );
            endpoint
        } else {
            warn!("Failed to read end-point from .env file.\n\n");
            "http://127.0.0.1:8000/process".to_string()
        };

        CustomAudio {
            processed_count: Mutex::new(0),
            process_audio_endpoint,
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for CustomAudio {
    const NAME: &'static str = "GstCustomAudio";
    type Type = super::CustomAudio;
    type ParentType = gst_audio::AudioFilter;
}


impl ObjectImpl for CustomAudio {}
impl GstObjectImpl for CustomAudio {}
impl ElementImpl for CustomAudio {
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
            gst::subclass::ElementMetadata::new(
                "Custom Audio Filter",
                "Filter/Effect/Converter/Audio",
                "Executes arbitrary function on audio.",
                "Michael Honaker <mike@beachcitiessoft.com>",
            )
        });
        Some(&*ELEMENT_METADATA)
    }

    fn pad_templates() -> &'static [gst::PadTemplate] {
        static PAD_TEMPLATES: Lazy<Vec<gst::PadTemplate>> = Lazy::new(|| {
            let caps = gst::Caps::new_any();

            let src_pad_template = gst::PadTemplate::new(
                "src",
                gst::PadDirection::Src,
                gst::PadPresence::Always,
                &caps,
            ).unwrap();

            let sink_pad_template = gst::PadTemplate::new(
                "sink",
                gst::PadDirection::Sink,
                gst::PadPresence::Always,
                &caps,
            ).unwrap();

            vec![src_pad_template, sink_pad_template]
        });

        PAD_TEMPLATES.as_ref()
    }
}
impl BaseTransformImpl for CustomAudio {
    const MODE: BaseTransformMode = BaseTransformMode::AlwaysInPlace;
    const PASSTHROUGH_ON_SAME_CAPS: bool = false;
    const TRANSFORM_IP_ON_PASSTHROUGH: bool = false;

    /// This function processes the audio buffer in place.
    ///
    /// # Arguments
    ///
    /// * `buf` - A mutable reference to the audio buffer.
    ///
    /// # Returns
    ///
    /// * `Result<gst::FlowSuccess, gst::FlowError>` - The result of the transformation.
    fn transform_ip(&self, buf: &mut gst::BufferRef) -> Result<gst::FlowSuccess, gst::FlowError> {
        let sent_data = buf.map_readable().unwrap().as_slice().to_vec();
        let endpoint = self.process_audio_endpoint.clone();
        let processed_count = self.processed_count.lock().unwrap().clone();
        let client = blocking::Client::new();

        // Submit the audio to the REST-API
        let part = multipart::Part::bytes(sent_data.clone())
            .file_name("audio.raw")
            .mime_str("application/octet-stream")
            .unwrap();
        let form = multipart::Form::new().part("file", part);

        // Call the REST API to with the audio data to be processed.
        match client.post(&endpoint).multipart(form).send() {
            Ok(resp) => {
                let received_data: Box<[u8]> = if resp.status().is_success() {
                    resp.bytes().unwrap().to_vec().into_boxed_slice()
                } else {
                    sent_data.to_vec().into_boxed_slice()
                };

                let mut count: i32 = processed_count.clone();
                count += 1;
                gst::trace!(
                    CAT,
                    "Processed {} audio samples",
                    count
                );
                trace!("\n Sent data: {}\n Rcvd data: {}\n",
                    hex::encode(&sent_data),
                    hex::encode(&received_data));
                let mut map = buf.map_writable().unwrap();
                map.as_mut_slice().copy_from_slice(&received_data);
                Ok(gst::FlowSuccess::Ok)
            }
            Err(err) => {
                error!("Failed to get a valid response: {}", err);
                gst::error!(
                    CAT,
                    "Failed to get a valid response: {}",
                    err
                );
                Err(gst::FlowError::Error)
            }
        }
    }
}

impl AudioFilterImpl for CustomAudio {}
