use std::{fs, io};
use std::path::Path;
use quick_xml::{de, se, DeError, SeError};
use serde::{Deserialize, Serialize};

pub mod test_utils;

#[derive(Debug)]
pub enum ReadXMLFileError {
    IOError(io::Error),
    DeError(DeError),
}

#[derive(Debug)]
pub enum WriteXMLFileError {
    IOError(io::Error),
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