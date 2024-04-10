use std::path::PathBuf;

use once_cell::sync::Lazy;

pub static INSTALL_DIR: Lazy<PathBuf> = Lazy::new(|| {
    if let Ok(path) = std::env::var("SCOOP_RS_DIR") {
        PathBuf::from(path)
    } else {
        let mut path = dirs::home_dir().expect("Failed to get home directory");
        path.push("scoop");
        path
    }
});

pub static BUCKETS_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = INSTALL_DIR.clone();
    path.push("buckets");
    if !path.exists() {
        std::fs::create_dir_all(&path).expect("Failed to create cache directory");
    }
    path
});

pub static CACHE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = INSTALL_DIR.clone();
    path.push("cache");
    if !path.exists() {
        std::fs::create_dir_all(&path).expect("Failed to create cache directory");
    }
    path
});
