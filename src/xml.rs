use std::{fs, io};
use std::path::Path;
use quick_xml::{de, se, DeError, SeError};
use serde::{Deserialize, Serialize};
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
        if declaration_header {
            let mut serialized = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
            serialized.push_str(&se::to_string(self)?);
            Ok(serialized)
        } else {
            se::to_string(self)
        }
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
    use tempfile::tempdir;
    use crate::xml::test_utils::cleanup_xml;
    use super::*;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestData {
        #[serde(rename = "@attr")]
        attr: String,
    }

    const EXPECTED_SERIALIZED: &'static str = r#"<TestData attr="value"/>"#;

    fn expected_deserialized() -> TestData {
        TestData {
            attr: "value".into(),
        }
    }

    #[test]
    fn test_from_xml() {
        let test_data = TestData::from_xml(EXPECTED_SERIALIZED).unwrap();
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
        assert_eq!(xml, cleanup_xml(EXPECTED_SERIALIZED.into()));
    }

    #[test]
    fn test_to_xml_file_by_path() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("some_dir/xml_output_file.xml");

        expected_deserialized().to_xml_file_by_path(&file_path, false).unwrap();

        let xml = fs::read_to_string(&file_path).unwrap();
        assert_eq!(xml, cleanup_xml(EXPECTED_SERIALIZED.into()));
    }
}