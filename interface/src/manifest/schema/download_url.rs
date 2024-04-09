use core::fmt;
use std::{
    convert::Infallible,
    fmt::{Display, Formatter},
    str::FromStr,
};

use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(DeserializeFromStr, SerializeDisplay, Debug, Clone)]
pub struct DownloadUrl {
    pub url: String,
    pub file_name: Option<String>,
}

impl FromStr for DownloadUrl {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((url, file_name)) = s.rsplit_once("#/") {
            Ok(DownloadUrl {
                url: url.to_string(),
                file_name: Some(file_name.to_string()),
            })
        } else {
            Ok(DownloadUrl {
                url: s.to_string(),
                file_name: None,
            })
        }
    }
}

impl Display for DownloadUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(file_name) = &self.file_name {
            write!(f, "{}/#{}", self.url, file_name)
        } else {
            write!(f, "{}", self.url)
        }
    }
}
