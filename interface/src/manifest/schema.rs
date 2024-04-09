use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferOne, serde_as, OneOrMany};
use std::collections::HashMap;

pub mod arch;
mod bin;
mod download_url;
#[cfg(test)]
mod test;

pub use bin::Bin;
use bin::{parse_bin, serialize_bin};
pub use download_url::DownloadUrl;

use crate::bucket_app::BucketAppName;

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Manifest {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    /// _comment is Deprecated. Use ## instead.
    #[serde(alias = "##")]
    #[serde(alias = "_comment")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub comment: Option<Vec<String>>,
    pub architecture: Option<ManifestArchitecture>,
    pub autoupdate: Option<Autoupdate>,
    #[serde(
        deserialize_with = "parse_bin",
        serialize_with = "serialize_bin",
        default
    )]
    pub bin: Option<Vec<Bin>>,
    pub checkver: Option<Checkver>,
    /// Undocumented: Found at https://github.com/se35710/scoop-java/search?l=JSON&q=cookie
    pub cookie: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub depends: Option<Vec<BucketAppName>>,
    pub description: Option<String>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub env_add_path: Option<Vec<String>>,
    pub env_set: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub extract_dir: Option<Vec<String>>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub extract_to: Option<Vec<String>>,
    pub hash: Option<Hash>,
    pub homepage: String,
    /// True if the installer InnoSetup based. Found in
    /// https://github.com/ScoopInstaller/Main/search?l=JSON&q=innosetup
    pub innosetup: Option<bool>,
    pub installer: Option<ManifestInstaller>,
    pub license: License,
    /// Deprecated
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub msi: Option<Vec<String>>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub notes: Option<Vec<String>>,
    #[serde(
        deserialize_with = "parse_bin",
        serialize_with = "serialize_bin",
        default
    )]
    pub persist: Option<Vec<Bin>>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub post_install: Option<Vec<String>>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub post_uninstall: Option<Vec<String>>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub pre_install: Option<Vec<String>>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub pre_uninstall: Option<Vec<String>>,
    pub psmodule: Option<Psmodule>,
    pub shortcuts: Option<Vec<Vec<String>>>,
    pub suggest: Option<Suggest>,
    pub uninstaller: Option<Uninstaller>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub url: Option<Vec<DownloadUrl>>,
    pub version: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ManifestArchitecture {
    #[serde(rename = "32bit")]
    pub the_32_bit: Option<ArchManifest>,
    #[serde(rename = "64bit")]
    pub the_64_bit: Option<ArchManifest>,
    pub arm64: Option<ArchManifest>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ArchManifest {
    #[serde(
        deserialize_with = "parse_bin",
        serialize_with = "serialize_bin",
        default
    )]
    pub bin: Option<Vec<Bin>>,
    pub checkver: Option<Checkver>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub env_add_path: Option<Vec<String>>,
    pub env_set: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub extract_dir: Option<Vec<String>>,
    pub hash: Option<Hash>,
    pub installer: Option<ManifestInstaller>,
    /// Deprecated
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub msi: Option<Vec<String>>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub post_install: Option<Vec<String>>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub post_uninstall: Option<Vec<String>>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub pre_install: Option<Vec<String>>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub pre_uninstall: Option<Vec<String>>,
    pub shortcuts: Option<Vec<Vec<String>>>,
    pub uninstaller: Option<Uninstaller>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub url: Option<Vec<DownloadUrl>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Checkver {
    CheckverClass(Box<CheckverClass>),
    Template(CheckverTemplate),
    String(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CheckverTemplate {
    Github,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CheckverClass {
    pub github: Option<String>,
    /// Same as 'jsonpath'
    pub jp: Option<String>,
    pub jsonpath: Option<String>,
    /// Same as 'regex'
    pub re: Option<String>,
    pub regex: Option<String>,
    /// Allows rearrange the regexp matches
    pub replace: Option<String>,
    /// Reverse the order of regex matches
    pub reverse: Option<bool>,
    /// Custom PowerShell script to retrieve application version using more complex approach.
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub script: Option<Vec<String>>,
    pub sourceforge: Option<CheckVerSourceforge>,
    pub url: Option<String>,
    pub useragent: Option<String>,
    pub xpath: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum CheckVerSourceforge {
    SourceforgeClass(SourceforgeDetail),
    String(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SourceforgeDetail {
    pub path: Option<String>,
    pub project: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Hash {
    String(String),
    StringArray(Vec<String>),
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ManifestInstaller {
    /// Undocumented: only used in scoop-extras/oraclejdk* and scoop-extras/appengine-go
    #[serde(rename = "_comment")]
    pub comment: Option<String>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub args: Option<Vec<String>>,
    pub file: Option<String>,
    pub keep: Option<bool>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub script: Option<Vec<String>>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Uninstaller {
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub args: Option<Vec<String>>,
    pub file: Option<String>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub script: Option<Vec<String>>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Autoupdate {
    pub architecture: Option<AutoupdateArchitecture>,
    #[serde(
        deserialize_with = "parse_bin",
        serialize_with = "serialize_bin",
        default
    )]
    pub bin: Option<Vec<Bin>>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub env_add_path: Option<Vec<String>>,
    pub env_set: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub extract_dir: Option<Vec<String>>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub hash: Option<Vec<HashExtraction>>,
    pub installer: Option<Installer>,
    pub license: Option<License>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub notes: Option<Vec<String>>,
    #[serde(
        deserialize_with = "parse_bin",
        serialize_with = "serialize_bin",
        default
    )]
    pub persist: Option<Vec<Bin>>,
    pub psmodule: Option<Psmodule>,
    pub shortcuts: Option<Vec<Vec<String>>>,
    // Don't use DownloadUrl, because this is not complete url
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub url: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AutoupdateArchitecture {
    #[serde(rename = "32bit")]
    pub the_32_bit: Option<AutoupdateArch>,
    #[serde(rename = "64bit")]
    pub the_64_bit: Option<AutoupdateArch>,
    pub arm64: Option<AutoupdateArch>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AutoupdateArch {
    #[serde(
        deserialize_with = "parse_bin",
        serialize_with = "serialize_bin",
        default
    )]
    pub bin: Option<Vec<Bin>>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub env_add_path: Option<Vec<String>>,
    pub env_set: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub extract_dir: Option<Vec<String>>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub hash: Option<Vec<HashExtraction>>,
    pub installer: Option<Installer>,
    pub shortcuts: Option<Vec<Vec<String>>>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub url: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HashExtraction {
    /// "jp" is an alias for "jsonpath"
    #[serde(alias = "jp")]
    #[serde(alias = "jsonpath")]
    pub jsonpath: Option<String>,
    pub mode: Option<HashExtractionMode>,
    /// "find" is an alias for "regex
    #[serde(alias = "find")]
    #[serde(alias = "regex")]
    pub regex: Option<String>,
    /// Deprecated, hash type is determined automatically
    #[serde(rename = "type")]
    pub hash_extraction_type: Option<HashExtractionType>,
    pub url: Option<String>,
    pub xpath: Option<String>,
}

/// Deprecated, hash type is determined automatically
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum HashExtractionType {
    Md5,
    Sha1,
    Sha256,
    Sha512,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum HashExtractionMode {
    Download,
    Extract,
    Fosshub,
    Json,
    Metalink,
    Rdf,
    Sourceforge,
    Xpath,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Installer {
    pub file: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum License {
    Details(DetailedLicense),
    String(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DetailedLicense {
    pub identifier: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Psmodule {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Suggest {
    Array(Suggested),
    Dict(HashMap<String, Suggested>),
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Suggested(
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")] pub Option<Vec<BucketAppName>>,
);
