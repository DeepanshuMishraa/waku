//! Desktop ownership of the Insulator daemon process.

use std::path::PathBuf;

use anyhow::{Context as _, bail};

pub fn start_process() -> anyhow::Result<insulator_client::DaemonSupervisor> {
    let address = std::env::var(insulator_client::DAEMON_ADDRESS_ENV)
        .ok()
        .filter(|value| !value.trim().is_empty());
    let token = std::env::var(insulator_client::DAEMON_TOKEN_ENV)
        .ok()
        .filter(|value| !value.is_empty());
    match (address, token) {
        (Some(address), Some(token)) => {
            return insulator_client::DaemonSupervisor::connect(address.trim(), token);
        }
        (Some(_), None) => bail!(
            "{} is set but {} is missing",
            insulator_client::DAEMON_ADDRESS_ENV,
            insulator_client::DAEMON_TOKEN_ENV
        ),
        (None, Some(_)) => bail!(
            "{} is set but {} is missing",
            insulator_client::DAEMON_TOKEN_ENV,
            insulator_client::DAEMON_ADDRESS_ENV
        ),
        (None, None) => {}
    }
    let app_settings = insulator_client::persistence::load_or_create_app_settings()
        .context("could not load desktop daemon settings")?;
    insulator_client::DaemonSupervisor::spawn_configured(
        &daemon_executable_path()?,
        cfg!(debug_assertions),
        app_settings.daemon_exposure,
    )
}

/// Resolve the local host name once during app construction. Settings can
/// then show a useful LAN URL without touching the OS from a render frame.
pub fn local_hostname() -> Option<String> {
    #[cfg(unix)]
    {
        let mut buffer = [0_u8; 256];
        let result = unsafe { libc::gethostname(buffer.as_mut_ptr().cast(), buffer.len()) };
        if result == 0 {
            let length = buffer
                .iter()
                .position(|byte| *byte == 0)
                .unwrap_or(buffer.len());
            let hostname = String::from_utf8_lossy(&buffer[..length]).trim().to_owned();
            if !hostname.is_empty() {
                return Some(hostname);
            }
        }
    }
    // `COMPUTERNAME` is the Windows equivalent and is always set; `HOSTNAME`
    // covers the shells that export it.
    ["COMPUTERNAME", "HOSTNAME"]
        .into_iter()
        .filter_map(|name| std::env::var(name).ok())
        .map(|hostname| hostname.trim().to_owned())
        .find(|hostname| !hostname.is_empty())
}

fn daemon_executable_path() -> anyhow::Result<PathBuf> {
    if let Some(path) = std::env::var_os("INSULATOR_DAEMON_PATH")
        .or_else(|| std::env::var_os("INSULATOR_DAEMON_PATH"))
        .filter(|path| !path.is_empty())
    {
        return Ok(path.into());
    }
    let candidates = [
        format!("insulator-debug-daemon{}", std::env::consts::EXE_SUFFIX),
        format!("insulator-daemon{}", std::env::consts::EXE_SUFFIX),
        format!("insulator-daemon{}", std::env::consts::EXE_SUFFIX),
    ];
    let current = std::env::current_exe().context("could not locate the app executable")?;

    // Development keeps the daemon beside Cargo's debug artifacts rather than
    // inside Insulator Debug.app. The supervisor watches this file and swaps only
    // the daemon when the development watcher relinks it.
    #[cfg(debug_assertions)]
    if let Some(debug_directory) = current
        .ancestors()
        .find(|candidate| candidate.file_name().is_some_and(|name| name == "debug"))
    {
        for candidate in &candidates {
            let external = debug_directory.join(candidate);
            if external.is_file() {
                return Ok(external);
            }
        }
    }

    if let Some(parent) = current.parent() {
        for candidate in &candidates {
            let sibling = parent.join(candidate);
            if sibling.is_file() {
                return Ok(sibling);
            }
        }
    }

    let default_sibling = current
        .parent()
        .map(|directory| directory.join(&candidates[0]))
        .unwrap_or_else(|| PathBuf::from(&candidates[0]));

    #[cfg(debug_assertions)]
    bail!(
        "Insulator daemon was not found in Cargo's debug directory or next to the app executable: {}",
        default_sibling.display(),
    );
    #[cfg(not(debug_assertions))]
    bail!(
        "Insulator daemon is missing next to the app executable: {}",
        default_sibling.display(),
    )
}
