use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferOne, serde_as, OneOrMany};
use std::collections::HashMap;

mod bin;
#[cfg(test)]
mod test;

pub use bin::Bin;
use bin::{parse_bin, serialize_bin};

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Manifest {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    /// _comment is Deprecated. Use ## instead.
    #[serde(alias = "##")]
    #[serde(alias = "_comment")]
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub comment: Vec<String>,
    pub architecture: Option<ManifestArchitecture>,
    pub autoupdate: Option<Autoupdate>,
    #[serde(
        default,
        deserialize_with = "parse_bin",
        serialize_with = "serialize_bin"
    )]
    pub bin: Vec<Bin>,
    pub checkver: Option<Checkver>,
    /// Undocumented: Found at https://github.com/se35710/scoop-java/search?l=JSON&q=cookie
    pub cookie: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub depends: Vec<String>,
    pub description: Option<String>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub env_add_path: Vec<String>,
    pub env_set: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub extract_dir: Vec<String>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub extract_to: Vec<String>,
    pub hash: Option<Hash>,
    pub homepage: String,
    /// True if the installer InnoSetup based. Found in
    /// https://github.com/ScoopInstaller/Main/search?l=JSON&q=innosetup
    pub innosetup: Option<bool>,
    pub installer: Option<ManifestInstaller>,
    pub license: License,
    /// Deprecated
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub msi: Vec<String>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub notes: Vec<String>,
    #[serde(
        default,
        deserialize_with = "parse_bin",
        serialize_with = "serialize_bin"
    )]
    pub persist: Vec<Bin>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub post_install: Vec<String>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub post_uninstall: Vec<String>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub pre_install: Vec<String>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub pre_uninstall: Vec<String>,
    pub psmodule: Option<Psmodule>,
    pub shortcuts: Option<Vec<Vec<String>>>,
    pub suggest: Option<Suggest>,
    pub uninstaller: Option<Uninstaller>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub url: Vec<String>,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct ManifestArchitecture {
    #[serde(rename = "32bit")]
    pub the_32_bit: Option<ArchManifest>,
    #[serde(rename = "64bit")]
    pub the_64_bit: Option<ArchManifest>,
    pub arm64: Option<ArchManifest>,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct ArchManifest {
    #[serde(
        default,
        deserialize_with = "parse_bin",
        serialize_with = "serialize_bin"
    )]
    pub bin: Vec<Bin>,
    pub checkver: Option<Checkver>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub env_add_path: Vec<String>,
    pub env_set: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub extract_dir: Vec<String>,
    pub hash: Option<Hash>,
    pub installer: Option<ManifestInstaller>,
    /// Deprecated
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub msi: Vec<String>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub post_install: Vec<String>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub post_uninstall: Vec<String>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub pre_install: Vec<String>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub pre_uninstall: Vec<String>,
    pub shortcuts: Option<Vec<Vec<String>>>,
    pub uninstaller: Option<Uninstaller>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub url: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Checkver {
    CheckverClass(Box<CheckverClass>),
    Template(CheckverTemplate),
    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckverTemplate {
    Github,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
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
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub script: Vec<String>,
    pub sourceforge: Option<CheckVerSourceforge>,
    pub url: Option<String>,
    pub useragent: Option<String>,
    pub xpath: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckVerSourceforge {
    SourceforgeClass(SourceforgeDetail),
    String(String),
}

#[derive(Serialize, Deserialize)]
pub struct SourceforgeDetail {
    pub path: Option<String>,
    pub project: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Hash {
    String(String),
    StringArray(Vec<String>),
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct ManifestInstaller {
    /// Undocumented: only used in scoop-extras/oraclejdk* and scoop-extras/appengine-go
    #[serde(rename = "_comment")]
    pub comment: Option<String>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub args: Vec<String>,
    pub file: Option<String>,
    pub keep: Option<bool>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub script: Vec<String>,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Uninstaller {
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub args: Vec<String>,
    pub file: Option<String>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub script: Vec<String>,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Autoupdate {
    pub architecture: Option<AutoupdateArchitecture>,
    #[serde(
        default,
        deserialize_with = "parse_bin",
        serialize_with = "serialize_bin"
    )]
    pub bin: Vec<Bin>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub env_add_path: Vec<String>,
    pub env_set: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub extract_dir: Vec<String>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub hash: Option<Vec<HashExtraction>>,
    pub installer: Option<Installer>,
    pub license: Option<License>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub notes: Vec<String>,
    #[serde(
        default,
        deserialize_with = "parse_bin",
        serialize_with = "serialize_bin"
    )]
    pub persist: Vec<Bin>,
    pub psmodule: Option<Psmodule>,
    pub shortcuts: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub url: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AutoupdateArchitecture {
    #[serde(rename = "32bit")]
    pub the_32_bit: Option<AutoupdateArch>,
    #[serde(rename = "64bit")]
    pub the_64_bit: Option<AutoupdateArch>,
    pub arm64: Option<AutoupdateArch>,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct AutoupdateArch {
    #[serde(
        default,
        deserialize_with = "parse_bin",
        serialize_with = "serialize_bin"
    )]
    pub bin: Vec<Bin>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub env_add_path: Vec<String>,
    pub env_set: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub extract_dir: Vec<String>,
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferOne>>")]
    pub hash: Option<Vec<HashExtraction>>,
    pub installer: Option<Installer>,
    pub shortcuts: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub url: Vec<String>,
}

#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HashExtractionType {
    Md5,
    Sha1,
    Sha256,
    Sha512,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Installer {
    pub file: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum License {
    Details(DetailedLicense),
    String(String),
}

#[derive(Serialize, Deserialize)]
pub struct DetailedLicense {
    pub identifier: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Psmodule {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Suggest {
    Array(Suggested),
    Dict(HashMap<String, Suggested>),
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Suggested(#[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")] pub Vec<String>);
