//! Small, synthesized UI cues based on Cuelume's live Web Audio recipes.
//!
//! The WAV assets keep playback deterministic while `rodio` supplies the
//! platform audio device layer on macOS, Linux, and Windows.

use std::io::Cursor;
use std::sync::atomic::{AtomicU32, Ordering};

static VOLUME_PERCENT: AtomicU32 = AtomicU32::new(100);

pub fn set_volume(percent: f32) {
    let percent = percent.clamp(0.0, 100.0);
    VOLUME_PERCENT.store(percent as u32, Ordering::Relaxed);
}

fn play_asset(bytes: &'static [u8], name: &'static str) {
    // Audio device setup and playback may touch platform services, so keep it
    // off the GPUI thread. The stream must stay alive until the sink finishes.
    std::thread::Builder::new()
        .name(format!("insulator-audio-{name}"))
        .spawn(move || {
            let Ok(stream) = rodio::OutputStreamBuilder::open_default_stream() else {
                return;
            };
            let Ok(source) = rodio::Decoder::try_from(Cursor::new(bytes)) else {
                return;
            };
            let sink = rodio::Sink::connect_new(stream.mixer());
            sink.set_volume(VOLUME_PERCENT.load(Ordering::Relaxed) as f32 / 100.0);
            sink.append(source);
            sink.sleep_until_end();
        })
        .ok();
}

pub fn play_success() {
    play_asset(include_bytes!("../assets/sounds/success.wav"), "success");
}

pub fn play_arrival() {
    play_asset(include_bytes!("../assets/sounds/arrival.wav"), "arrival");
}
