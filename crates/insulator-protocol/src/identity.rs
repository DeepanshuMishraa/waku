//! Shared application identity used by the daemon and desktop client.

#[cfg(debug_assertions)]
pub const APP_NAME: &str = "Insulator Debug";
#[cfg(not(debug_assertions))]
pub const APP_NAME: &str = "Insulator";

#[cfg(debug_assertions)]
pub const APP_ID: &str = "sh.insulator.dev";
#[cfg(not(debug_assertions))]
pub const APP_ID: &str = "sh.insulator";

#[cfg(debug_assertions)]
pub const DATA_DIRECTORY_NAME: &str = "Insulator Debug";
#[cfg(not(debug_assertions))]
pub const DATA_DIRECTORY_NAME: &str = "Insulator";
