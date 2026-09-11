//! Small, synthesized UI cues based on Cuelume's live Web Audio recipes.
//!
//! The WAV assets keep playback deterministic while `rodio` supplies the
//! platform audio device layer on macOS, Linux, and Windows.

use std::io::Cursor;

fn play_asset(bytes: &'static [u8], name: &'static str) {
    // Audio device setup and playback may touch platform services, so keep it
    // off the GPUI thread. The stream must stay alive until the sink finishes.
    std::thread::Builder::new()
        .name(format!("waku-audio-{name}"))
        .spawn(move || {
            let Ok(stream) = rodio::OutputStreamBuilder::open_default_stream() else {
                return;
            };
            let Ok(source) = rodio::Decoder::try_from(Cursor::new(bytes)) else {
                return;
            };
            let sink = rodio::Sink::connect_new(stream.mixer());
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
