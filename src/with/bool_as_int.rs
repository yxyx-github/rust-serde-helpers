pub mod bool_as_int_format {
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(b: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(if *b { "1" } else { "0" })
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "1" => Ok(true),
            "0" => Ok(false),
            other => Err(de::Error::custom(format!("Invalid boolean value: {other}"))),
        }
    }
}