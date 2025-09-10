use quick_xml::se::Serializer;
use quick_xml::{de, DeError, SeError};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, io};
use thiserror::Error;

pub mod test_utils;

#[derive(Error, Debug)]
pub enum ReadXMLFileError {
    #[error("{0}")]
    IOError(io::Error),

    #[error("Couldn't deserialize: {0}")]
    DeError(DeError),
}

#[derive(Error, Debug)]
pub enum WriteXMLFileError {
    #[error("{0}")]
    IOError(io::Error),

    #[error("Couldn't serialize: {0}")]
    SeError(SeError),
}

pub trait FromXML: for<'de> Deserialize<'de> {
    fn from_xml(xml: &str) -> Result<Self, DeError>;

    fn from_xml_file_by_path<P: AsRef<Path>>(path: P) -> Result<Self, ReadXMLFileError>;
}

impl<T> FromXML for T
where
    T: for<'de> Deserialize<'de>,
{
    fn from_xml(xml: &str) -> Result<Self, DeError> {
        de::from_str(xml)
    }

    fn from_xml_file_by_path<P: AsRef<Path>>(path: P) -> Result<Self, ReadXMLFileError> {
        let xml = fs::read_to_string(path).map_err(|err| ReadXMLFileError::IOError(err))?;
        Ok(
            Self::from_xml(&xml)
                .map_err(|err| ReadXMLFileError::DeError(err))?
        )
    }
}

pub trait ToXML: Serialize {
    fn to_xml(&self, declaration_header: bool) -> Result<String, SeError>;

    fn to_xml_file_by_path<P: AsRef<Path>>(&self, path: P, declaration_header: bool) -> Result<(), WriteXMLFileError>;
}

impl<T> ToXML for T
where
    T: Serialize,
{
    fn to_xml(&self, declaration_header: bool) -> Result<String, SeError> {
        let mut serialized_content = String::new();
        let mut ser = Serializer::new(&mut serialized_content);
        ser.indent(' ', 2);
        self.serialize(ser)?;
        // let serialized_content = se::to_string(self)?;
        Ok(if declaration_header {
            let mut serialized = String::from(concat!(r#"<?xml version="1.0" encoding="UTF-8"?>"#, "\n"));
            serialized.push_str(&serialized_content);
            serialized
        } else {
            serialized_content
        })
    }

    fn to_xml_file_by_path<P: AsRef<Path>>(&self, path: P, declaration_header: bool) -> Result<(), WriteXMLFileError> {
        let xml = self.to_xml(declaration_header)
            .map_err(|err| WriteXMLFileError::SeError(err))?;
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent).map_err(|err| WriteXMLFileError::IOError(err))?;
        }
        fs::write(path, xml).map_err(|err| WriteXMLFileError::IOError(err))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xml::test_utils::cleanup_xml;
    use tempfile::tempdir;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestData {
        #[serde(rename = "@attr")]
        attr: String,

        #[serde(rename = "TestDataChild")]
        child: TestDataChild,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestDataChild {
        #[serde(rename = "@attr")]
        attr: String,
    }

    const EXPECTED_SERIALIZED: &'static str = r#"
        <TestData attr="value">
            <TestDataChild attr="child value"/>
        </TestData>
    "#;

    const EXPECTED_SERIALIZED_WITH_DECLARATION_HEADER: &'static str = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <TestData attr="value">
            <TestDataChild attr="child value"/>
        </TestData>
    "#;

    fn expected_deserialized() -> TestData {
        TestData {
            attr: "value".into(),
            child: TestDataChild {
                attr: "child value".into(),
            }
        }
    }

    #[test]
    fn test_from_xml() {
        let test_data = TestData::from_xml(EXPECTED_SERIALIZED).unwrap();
        assert_eq!(test_data, expected_deserialized());
    }

    #[test]
    fn test_from_xml_with_declaration_header() {
        let test_data = TestData::from_xml(EXPECTED_SERIALIZED_WITH_DECLARATION_HEADER).unwrap();
        assert_eq!(test_data, expected_deserialized());
    }

    #[test]
    fn test_from_xml_file_by_path() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("some_dir/xml_input_file.xml");
        fs::create_dir_all(file_path.parent().unwrap()).unwrap();
        fs::write(&file_path, EXPECTED_SERIALIZED).unwrap();

        let test_data = TestData::from_xml_file_by_path(&file_path).unwrap();

        assert_eq!(test_data, expected_deserialized());
    }

    #[test]
    fn test_to_xml() {
        let xml = expected_deserialized().to_xml(false).unwrap();
        assert_eq!(cleanup_xml(xml), cleanup_xml(EXPECTED_SERIALIZED.into()));
    }

    #[test]
    fn test_to_xml_with_declaration_header() {
        let xml = expected_deserialized().to_xml(true).unwrap();
        assert_eq!(cleanup_xml(xml), cleanup_xml(EXPECTED_SERIALIZED_WITH_DECLARATION_HEADER.into()));
    }

    #[test]
    fn test_to_xml_file_by_path() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("some_dir/xml_output_file.xml");

        expected_deserialized().to_xml_file_by_path(&file_path, false).unwrap();

        let xml = fs::read_to_string(&file_path).unwrap();
        assert_eq!(cleanup_xml(xml), cleanup_xml(EXPECTED_SERIALIZED.into()));
    }
}