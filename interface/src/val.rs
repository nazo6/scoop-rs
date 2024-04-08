use std::path::PathBuf;

use once_cell::sync::Lazy;

pub static INSTALL_PATH: Lazy<PathBuf> = Lazy::new(|| {
    if let Ok(path) = std::env::var("SCOOP_RS_DIR") {
        PathBuf::from(path)
    } else {
        let mut path = dirs::home_dir().expect("Failed to get home directory");
        path.push("scoop-rs");
        path
    }
});
