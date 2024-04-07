use serde::{de, Deserialize, Deserializer, Serialize};
use std::{collections::HashMap, marker::PhantomData};

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    /// A comment.
    #[serde(rename = "##", deserialize_with = "string_or_seq_string")]
    pub empty: Vec<String>,
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    /// Deprecated. Use ## instead.
    #[serde(rename = "_comment", deserialize_with = "string_or_seq_string")]
    pub comment: Vec<String>,
    pub architecture: Option<ManifestArchitecture>,
    pub autoupdate: Option<Autoupdate>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub bin: Vec<String>,
    pub checkver: Option<Checkver>,
    /// Undocumented: Found at https://github.com/se35710/scoop-java/search?l=JSON&q=cookie
    pub cookie: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub depends: Vec<String>,
    pub description: Option<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub env_add_path: Vec<String>,
    pub env_set: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub extract_dir: Vec<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub extract_to: Vec<String>,
    pub hash: Option<Hash>,
    pub homepage: String,
    /// True if the installer InnoSetup based. Found in
    /// https://github.com/ScoopInstaller/Main/search?l=JSON&q=innosetup
    pub innosetup: Option<bool>,
    pub installer: Option<ManifestInstaller>,
    pub license: ManifestLicense,
    /// Deprecated
    #[serde(deserialize_with = "string_or_seq_string")]
    pub msi: Vec<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub notes: Vec<String>,
    pub persist: Option<StringOrArrayOfStringsOrAnArrayOfArrayOfStrings>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub post_install: Vec<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub post_uninstall: Vec<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub pre_install: Vec<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub pre_uninstall: Vec<String>,
    pub psmodule: Option<ManifestPsmodule>,
    pub shortcuts: Option<Vec<Vec<String>>>,
    pub suggest: Option<Suggest>,
    pub uninstaller: Option<Uninstaller>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub url: Vec<String>,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct ManifestArchitecture {
    #[serde(rename = "32bit")]
    pub the_32_bit: Option<The32BitClass>,
    #[serde(rename = "64bit")]
    pub the_64_bit: Option<The32BitClass>,
    pub arm64: Option<The32BitClass>,
}

#[derive(Serialize, Deserialize)]
pub struct The32BitClass {
    #[serde(deserialize_with = "string_or_seq_string")]
    pub bin: Vec<String>,
    pub checkver: Option<Checkver>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub env_add_path: Vec<String>,
    pub env_set: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub extract_dir: Vec<String>,
    pub hash: Option<Hash>,
    pub installer: Option<ManifestInstaller>,
    /// Deprecated
    #[serde(deserialize_with = "string_or_seq_string")]
    pub msi: Vec<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub post_install: Vec<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub post_uninstall: Vec<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub pre_install: Vec<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub pre_uninstall: Vec<String>,
    pub shortcuts: Option<Vec<Vec<String>>>,
    pub uninstaller: Option<Uninstaller>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub url: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrArrayOfStringsOrAnArrayOfArrayOfStrings {
    String(String),
    UnionArray(Vec<StringOrArrayOfStringsElement>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrArrayOfStringsElement {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Checkver {
    CheckverClass(Box<CheckverClass>),
    String(CheckverTemplate),
}

#[derive(Serialize, Deserialize)]
pub enum CheckverTemplate {
    Github,
}

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
    #[serde(deserialize_with = "string_or_seq_string")]
    pub script: Vec<String>,
    pub sourceforge: Option<SourceforgeUnion>,
    pub url: Option<String>,
    pub useragent: Option<String>,
    pub xpath: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SourceforgeUnion {
    SourceforgeClass(SourceforgeClass),
    String(String),
}

#[derive(Serialize, Deserialize)]
pub struct SourceforgeClass {
    pub path: Option<String>,
    pub project: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Hash {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Serialize, Deserialize)]
pub struct ManifestInstaller {
    /// Undocumented: only used in scoop-extras/oraclejdk* and scoop-extras/appengine-go
    #[serde(rename = "_comment")]
    pub comment: Option<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub args: Vec<String>,
    pub file: Option<String>,
    pub keep: Option<bool>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub script: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Uninstaller {
    #[serde(deserialize_with = "string_or_seq_string")]
    pub args: Vec<String>,
    pub file: Option<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub script: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Autoupdate {
    pub architecture: Option<AutoupdateArchitecture>,
    pub bin: Option<StringOrArrayOfStringsOrAnArrayOfArrayOfStrings>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub env_add_path: Vec<String>,
    pub env_set: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub extract_dir: Vec<String>,
    pub hash: Option<HashExtractionOrArrayOfHashExtractions>,
    pub installer: Option<AutoupdateInstaller>,
    pub license: Option<AutoupdateLicense>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub notes: Vec<String>,
    pub persist: Option<StringOrArrayOfStringsOrAnArrayOfArrayOfStrings>,
    pub psmodule: Option<AutoupdatePsmodule>,
    pub shortcuts: Option<Vec<Vec<String>>>,
    #[serde(deserialize_with = "string_or_seq_string")]
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

#[derive(Serialize, Deserialize)]
pub struct AutoupdateArch {
    pub bin: Option<StringOrArrayOfStringsOrAnArrayOfArrayOfStrings>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub env_add_path: Vec<String>,
    pub env_set: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub extract_dir: Vec<String>,
    pub hash: Option<HashExtractionOrArrayOfHashExtractions>,
    pub installer: Option<PurpleInstaller>,
    pub shortcuts: Option<Vec<Vec<String>>>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub url: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum HashExtractionOrArrayOfHashExtractions {
    HashExtraction(HashExtraction),
    HashExtractionArray(Vec<HashExtraction>),
}

#[derive(Serialize, Deserialize)]
pub struct HashExtraction {
    /// Same as 'regex'
    pub find: Option<String>,
    /// Same as 'jsonpath'
    pub jp: Option<String>,
    pub jsonpath: Option<String>,
    pub mode: Option<Mode>,
    pub regex: Option<String>,
    /// Deprecated, hash type is determined automatically
    #[serde(rename = "type")]
    pub hash_extraction_type: Option<Type>,
    pub url: Option<String>,
    pub xpath: Option<String>,
}

/// Deprecated, hash type is determined automatically
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Md5,
    Sha1,
    Sha256,
    Sha512,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
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
pub struct PurpleInstaller {
    pub file: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AutoupdateInstaller {
    pub file: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AutoupdateLicense {
    License(License),
    String(String),
}

#[derive(Serialize, Deserialize)]
pub struct License {
    pub identifier: String,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AutoupdatePsmodule {
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ManifestLicense {
    License(License),
    String(String),
}

#[derive(Serialize, Deserialize)]
pub struct ManifestPsmodule {
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Suggest {}

fn string_or_seq_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrVec(PhantomData<Vec<String>>);

    impl<'de> de::Visitor<'de> for StringOrVec {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![value.to_owned()])
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Vec::new())
        }

        fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(StringOrVec(PhantomData))
}
