use serde::{de, Deserialize, Deserializer, Serialize};
use std::{collections::HashMap, marker::PhantomData};

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    /// A comment.
    #[serde(rename = "##", deserialize_with = "string_or_seq_string")]
    empty: Vec<String>,
    #[serde(rename = "$schema")]
    schema: Option<String>,
    /// Deprecated. Use ## instead.
    #[serde(rename = "_comment", deserialize_with = "string_or_seq_string")]
    comment: Vec<String>,
    architecture: Option<ManifestArchitecture>,
    autoupdate: Option<Autoupdate>,
    #[serde(deserialize_with = "string_or_seq_string")]
    bin: Vec<String>,
    checkver: Option<Checkver>,
    /// Undocumented: Found at https://github.com/se35710/scoop-java/search?l=JSON&q=cookie
    cookie: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(deserialize_with = "string_or_seq_string")]
    depends: Vec<String>,
    description: Option<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    env_add_path: Vec<String>,
    env_set: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(deserialize_with = "string_or_seq_string")]
    extract_dir: Vec<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    extract_to: Vec<String>,
    hash: Option<Hash>,
    homepage: String,
    /// True if the installer InnoSetup based. Found in
    /// https://github.com/ScoopInstaller/Main/search?l=JSON&q=innosetup
    innosetup: Option<bool>,
    installer: Option<ManifestInstaller>,
    license: ManifestLicense,
    /// Deprecated
    #[serde(deserialize_with = "string_or_seq_string")]
    msi: Vec<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    notes: Vec<String>,
    persist: Option<StringOrArrayOfStringsOrAnArrayOfArrayOfStrings>,
    #[serde(deserialize_with = "string_or_seq_string")]
    post_install: Vec<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    post_uninstall: Vec<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pre_install: Vec<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pre_uninstall: Vec<String>,
    psmodule: Option<ManifestPsmodule>,
    shortcuts: Option<Vec<Vec<String>>>,
    suggest: Option<Suggest>,
    uninstaller: Option<Uninstaller>,
    #[serde(deserialize_with = "string_or_seq_string")]
    url: Vec<String>,
    version: String,
}

#[derive(Serialize, Deserialize)]
pub struct ManifestArchitecture {
    #[serde(rename = "32bit")]
    the_32_bit: Option<The32BitClass>,
    #[serde(rename = "64bit")]
    the_64_bit: Option<The32BitClass>,
    arm64: Option<The32BitClass>,
}

#[derive(Serialize, Deserialize)]
pub struct The32BitClass {
    #[serde(deserialize_with = "string_or_seq_string")]
    bin: Vec<String>,
    checkver: Option<Checkver>,
    #[serde(deserialize_with = "string_or_seq_string")]
    env_add_path: Vec<String>,
    env_set: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(deserialize_with = "string_or_seq_string")]
    extract_dir: Vec<String>,
    hash: Option<Hash>,
    installer: Option<ManifestInstaller>,
    /// Deprecated
    #[serde(deserialize_with = "string_or_seq_string")]
    msi: Vec<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    post_install: Vec<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    post_uninstall: Vec<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pre_install: Vec<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pre_uninstall: Vec<String>,
    shortcuts: Option<Vec<Vec<String>>>,
    uninstaller: Option<Uninstaller>,
    #[serde(deserialize_with = "string_or_seq_string")]
    url: Vec<String>,
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
    github: Option<String>,
    /// Same as 'jsonpath'
    jp: Option<String>,
    jsonpath: Option<String>,
    /// Same as 'regex'
    re: Option<String>,
    regex: Option<String>,
    /// Allows rearrange the regexp matches
    replace: Option<String>,
    /// Reverse the order of regex matches
    reverse: Option<bool>,
    /// Custom PowerShell script to retrieve application version using more complex approach.
    #[serde(deserialize_with = "string_or_seq_string")]
    script: Vec<String>,
    sourceforge: Option<SourceforgeUnion>,
    url: Option<String>,
    useragent: Option<String>,
    xpath: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SourceforgeUnion {
    SourceforgeClass(SourceforgeClass),
    String(String),
}

#[derive(Serialize, Deserialize)]
pub struct SourceforgeClass {
    path: Option<String>,
    project: Option<String>,
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
    comment: Option<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    args: Vec<String>,
    file: Option<String>,
    keep: Option<bool>,
    #[serde(deserialize_with = "string_or_seq_string")]
    script: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Uninstaller {
    #[serde(deserialize_with = "string_or_seq_string")]
    args: Vec<String>,
    file: Option<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    script: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Autoupdate {
    architecture: Option<AutoupdateArchitecture>,
    bin: Option<StringOrArrayOfStringsOrAnArrayOfArrayOfStrings>,
    #[serde(deserialize_with = "string_or_seq_string")]
    env_add_path: Vec<String>,
    env_set: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(deserialize_with = "string_or_seq_string")]
    extract_dir: Vec<String>,
    hash: Option<HashExtractionOrArrayOfHashExtractions>,
    installer: Option<AutoupdateInstaller>,
    license: Option<AutoupdateLicense>,
    #[serde(deserialize_with = "string_or_seq_string")]
    notes: Vec<String>,
    persist: Option<StringOrArrayOfStringsOrAnArrayOfArrayOfStrings>,
    psmodule: Option<AutoupdatePsmodule>,
    shortcuts: Option<Vec<Vec<String>>>,
    #[serde(deserialize_with = "string_or_seq_string")]
    url: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AutoupdateArchitecture {
    #[serde(rename = "32bit")]
    the_32_bit: Option<AutoupdateArch>,
    #[serde(rename = "64bit")]
    the_64_bit: Option<AutoupdateArch>,
    arm64: Option<AutoupdateArch>,
}

#[derive(Serialize, Deserialize)]
pub struct AutoupdateArch {
    bin: Option<StringOrArrayOfStringsOrAnArrayOfArrayOfStrings>,
    #[serde(deserialize_with = "string_or_seq_string")]
    env_add_path: Vec<String>,
    env_set: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(deserialize_with = "string_or_seq_string")]
    extract_dir: Vec<String>,
    hash: Option<HashExtractionOrArrayOfHashExtractions>,
    installer: Option<PurpleInstaller>,
    shortcuts: Option<Vec<Vec<String>>>,
    #[serde(deserialize_with = "string_or_seq_string")]
    url: Vec<String>,
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
    find: Option<String>,
    /// Same as 'jsonpath'
    jp: Option<String>,
    jsonpath: Option<String>,
    mode: Option<Mode>,
    regex: Option<String>,
    /// Deprecated, hash type is determined automatically
    #[serde(rename = "type")]
    hash_extraction_type: Option<Type>,
    url: Option<String>,
    xpath: Option<String>,
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
    file: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AutoupdateInstaller {
    file: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AutoupdateLicense {
    License(License),
    String(String),
}

#[derive(Serialize, Deserialize)]
pub struct License {
    identifier: String,
    url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AutoupdatePsmodule {
    name: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ManifestLicense {
    License(License),
    String(String),
}

#[derive(Serialize, Deserialize)]
pub struct ManifestPsmodule {
    name: Option<String>,
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
