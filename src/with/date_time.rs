const DATE_TIME_FORMAT: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";

pub mod date_time_format {
    use crate::with::date_time::DATE_TIME_FORMAT;
    use serde::{de, ser, Deserialize, Deserializer, Serializer};
    use time::{format_description, PrimitiveDateTime};

    pub fn serialize<S>(pdt: &PrimitiveDateTime, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let format = format_description::parse(DATE_TIME_FORMAT).map_err(ser::Error::custom)?;
        let formatted = pdt.format(&format).map_err(ser::Error::custom)?;
        serializer.serialize_str(&formatted).map_err(ser::Error::custom)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<PrimitiveDateTime, D::Error> where D: Deserializer<'de> {
        let format = format_description::parse(DATE_TIME_FORMAT).map_err(de::Error::custom)?;
        let s = String::deserialize(deserializer)?;
        PrimitiveDateTime::parse(&s, &format).map_err(de::Error::custom)
    }
}

pub mod date_time_option_format {
    use crate::with::date_time::DATE_TIME_FORMAT;
    use serde::{de, ser, Deserialize, Deserializer, Serializer};
    use time::{format_description, PrimitiveDateTime};

    pub fn serialize<S>(pdt: &Option<PrimitiveDateTime>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match pdt {
            None => serializer.serialize_str("").map_err(ser::Error::custom),
            Some(pdt) => {
                let format = format_description::parse(DATE_TIME_FORMAT).map_err(ser::Error::custom)?;
                let formatted = pdt.format(&format).map_err(ser::Error::custom)?;
                serializer.serialize_str(&formatted).map_err(ser::Error::custom)
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<PrimitiveDateTime>, D::Error> where D: Deserializer<'de> {
        let value = String::deserialize(deserializer)?;
        if value == "" {
            return Ok(None);
        }
        let format = format_description::parse(DATE_TIME_FORMAT).map_err(de::Error::custom)?;
        Ok(Some(PrimitiveDateTime::parse(&value, &format).map_err(de::Error::custom)?))
    }
}