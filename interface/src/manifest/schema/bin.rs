use serde::{de, ser::SerializeSeq as _, Deserialize, Deserializer, Serializer};

use crate::utils::get_stem;

#[derive(Debug, Clone)]
pub struct Bin {
    /// Shim target executable
    pub target: String,
    /// Shim name (without extension)
    pub name: String,
    pub args: Option<Vec<String>>,
}

// bin field can be one of these:
// - no field
// - string
// - vec that contains
//    - string
//    - or tuple of (target, name, args) represented as a vec
pub(super) fn parse_bin<'de, D>(deserializer: D) -> Result<Option<Vec<Bin>>, D::Error>
where
    D: Deserializer<'de>,
{
    struct BinOrBinArray;
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum BinStringOrDetails {
        String(String),
        Vec(Vec<String>),
    }

    impl<'de> de::Visitor<'de> for BinOrBinArray {
        type Value = Option<Vec<Bin>>;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_str<E>(self, target: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // binary name without extension
            let name = get_stem(target).0.to_string();
            Ok(Some(vec![Bin {
                target: target.to_string(),
                name,
                args: None,
            }]))
        }

        fn visit_seq<S>(self, mut visitor: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            let mut bins = Vec::new();
            while let Some(data) = visitor.next_element::<BinStringOrDetails>()? {
                match data {
                    BinStringOrDetails::String(str) => {
                        bins.push(Bin {
                            target: str.clone(),
                            name: get_stem(&str).0.to_string(),
                            args: None,
                        });
                    }
                    BinStringOrDetails::Vec(vec) => {
                        let (target, name, args) = if vec.len() == 2 {
                            (vec[0].clone(), vec[1].clone(), None)
                        } else if vec.len() >= 3 {
                            (vec[0].clone(), vec[1].clone(), Some(vec[2..].to_vec()))
                        } else {
                            return Err(de::Error::custom("invalid bin format"));
                        };
                        bins.push(Bin { target, name, args });
                    }
                }
            }
            Ok(Some(bins))
        }
    }

    deserializer.deserialize_any(BinOrBinArray)
}

pub(super) fn serialize_bin<S>(bin: &Option<Vec<Bin>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(bin) = bin {
        let mut seq = serializer.serialize_seq(Some(bin.len()))?;
        for b in bin {
            if let Some(args) = &b.args {
                let mut vec = vec![&b.target, &b.name];
                for arg in args {
                    vec.push(arg);
                }
                seq.serialize_element(&vec)?;
            } else {
                seq.serialize_element(&[&b.target, &b.name])?;
            }
        }
        seq.end()
    } else {
        serializer.serialize_none()
    }
}
