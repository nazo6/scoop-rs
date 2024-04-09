use serde::{de, ser::SerializeSeq as _, Deserialize, Deserializer, Serializer};

#[derive(Debug, Clone)]
pub struct Persist {
    /// Persist target executable
    pub target: String,
    /// Persist name (without extension)
    pub name: Option<String>,
}

// bin field can be one of these:
// - no field
// - string
// - vec that contains
//    - string
//    - or tuple of (target, name, args) represented as a vec
pub(super) fn parse_persist<'de, D>(deserializer: D) -> Result<Option<Vec<Persist>>, D::Error>
where
    D: Deserializer<'de>,
{
    struct PersistOrPersistArray;
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum PersistStringOrDetails {
        String(String),
        Vec(Vec<String>),
    }

    impl<'de> de::Visitor<'de> for PersistOrPersistArray {
        type Value = Option<Vec<Persist>>;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_str<E>(self, target: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(vec![Persist {
                target: target.to_string(),
                name: None,
            }]))
        }

        fn visit_seq<S>(self, mut visitor: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            let mut bins = Vec::new();
            while let Some(data) = visitor.next_element::<PersistStringOrDetails>()? {
                match data {
                    PersistStringOrDetails::String(str) => {
                        bins.push(Persist {
                            target: str.clone(),
                            name: None,
                        });
                    }
                    PersistStringOrDetails::Vec(vec) => {
                        if vec.len() == 2 {
                            bins.push(Persist {
                                target: vec[0].clone(),
                                name: Some(vec[1].clone()),
                            });
                        } else {
                            return Err(serde::de::Error::custom("invalid persist format"));
                        }
                    }
                }
            }
            Ok(Some(bins))
        }
    }

    deserializer.deserialize_any(PersistOrPersistArray)
}

pub(super) fn serialize_persist<S>(
    bin: &Option<Vec<Persist>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(bin) = bin {
        let mut seq = serializer.serialize_seq(Some(bin.len()))?;
        for b in bin {
            if let Some(name) = &b.name {
                seq.serialize_element(&[&b.target, name])?;
            } else {
                seq.serialize_element(&b.target)?;
            }
        }
        seq.end()
    } else {
        serializer.serialize_none()
    }
}
