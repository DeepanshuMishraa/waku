//! Small, synthesized UI cues based on Cuelume's live Web Audio recipes.

#[cfg(target_os = "macos")]
fn play_asset(bytes: &'static [u8], filename: &'static str) {
    use std::path::PathBuf;
    use std::sync::OnceLock;

    static SUCCESS_PATH: OnceLock<Option<PathBuf>> = OnceLock::new();
    static ARRIVAL_PATH: OnceLock<Option<PathBuf>> = OnceLock::new();
    let slot = match filename {
        "success.wav" => &SUCCESS_PATH,
        "arrival.wav" => &ARRIVAL_PATH,
        _ => return,
    };
    let Some(path) = slot
        .get_or_init(|| {
            let path = std::env::temp_dir().join(format!("waku-{filename}"));
            (std::fs::write(&path, bytes).is_ok()).then_some(path)
        })
        .clone()
    else {
        return;
    };

    // Completion and drop handlers must not wait on the audio process or block
    // the GPUI thread.
    std::thread::spawn(move || {
        let _ = std::process::Command::new("/usr/bin/afplay")
            .arg(path)
            .spawn();
    });
}

#[cfg(target_os = "macos")]
pub fn play_success() {
    play_asset(
        include_bytes!("../assets/sounds/success.wav"),
        "success.wav",
    );
}

#[cfg(target_os = "macos")]
pub fn play_arrival() {
    play_asset(
        include_bytes!("../assets/sounds/arrival.wav"),
        "arrival.wav",
    );
}

#[cfg(not(target_os = "macos"))]
pub fn play_success() {}

#[cfg(not(target_os = "macos"))]
pub fn play_arrival() {}
