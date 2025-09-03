pub mod duration;
pub mod date_time;
pub mod bool_as_int;

#[cfg(test)]
mod tests {
    use crate::with::bool_as_int::bool_as_int_format;
    use crate::with::date_time::date_time_format;
    use crate::with::date_time::date_time_option_format;
    use crate::with::duration::duration_format;
    use crate::with::duration::duration_option_format;
    use crate::xml::test_utils::cleanup_xml;
    use crate::xml::{FromXML, ToXML};
    use serde::{Deserialize, Serialize};
    use time::macros::datetime;
    use time::{Duration, PrimitiveDateTime};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestData {
        #[serde(rename = "@dateTime", with = "date_time_format")]
        date_time: PrimitiveDateTime,

        #[serde(rename = "@dateTimeOption", with = "date_time_option_format")]
        date_time_option: Option<PrimitiveDateTime>,

        #[serde(rename = "@duration", with = "duration_format")]
        duration: Duration,

        #[serde(rename = "@durationOption", with = "duration_option_format")]
        duration_option: Option<Duration>,

        #[serde(rename = "@boolAsInt", with = "bool_as_int_format")]
        bool_as_int: bool,
    }

    const EXPECTED_SERIALIZED_1: &'static str = r#"
        <TestData
            dateTime="1970-03-07 20:39:40"
            dateTimeOption="1970-03-07 20:39:40"
            duration="03:02:09"
            durationOption="03:02:09"
            boolAsInt="1"
        />
    "#;

    const EXPECTED_SERIALIZED_2: &'static str = r#"
        <TestData
            dateTime="1970-03-07 20:39:40"
            dateTimeOption=""
            duration="03:02:09"
            durationOption=""
            boolAsInt="0"
        />
    "#;

    fn expected_deserialized_1() -> TestData {
        TestData {
            date_time: datetime!(1970-03-07 20:39:40),
            date_time_option: Some(datetime!(1970-03-07 20:39:40)),
            duration: Duration::hours(3) + Duration::minutes(2) + Duration::seconds(9),
            duration_option: Some(Duration::hours(3) + Duration::minutes(2) + Duration::seconds(9)),
            bool_as_int: true,
        }
    }

    fn expected_deserialized_2() -> TestData {
        TestData {
            date_time: datetime!(1970-03-07 20:39:40),
            date_time_option: None,
            duration: Duration::hours(3) + Duration::minutes(2) + Duration::seconds(9),
            duration_option: None,
            bool_as_int: false,
        }
    }

    #[test]
    fn test_from_xml() {
        let test_data = TestData::from_xml(EXPECTED_SERIALIZED_1).unwrap();
        assert_eq!(test_data, expected_deserialized_1());

        let test_data = TestData::from_xml(EXPECTED_SERIALIZED_2).unwrap();
        assert_eq!(test_data, expected_deserialized_2());
    }

    #[test]
    fn test_to_xml() {
        let xml = expected_deserialized_1().to_xml(false).unwrap();
        assert_eq!(xml, cleanup_xml(EXPECTED_SERIALIZED_1.into()));

        let xml = expected_deserialized_2().to_xml(false).unwrap();
        assert_eq!(xml, cleanup_xml(EXPECTED_SERIALIZED_2.into()));
    }
}