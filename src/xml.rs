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

// TODO: make it derivable
pub trait FromXML: for<'de> Deserialize<'de> {
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

// TODO: make it derivable
pub trait ToXML: Serialize {
    fn to_xml(&self) -> Result<String, SeError> {
        se::to_string(self)
    }

    fn to_xml_file_by_path<P: AsRef<Path>>(&self, path: P) -> Result<(), WriteXMLFileError> {
        let xml = self.to_xml()
            .map_err(|err| WriteXMLFileError::SeError(err))?;
        fs::write(path, xml).map_err(|err| WriteXMLFileError::IOError(err))?;
        Ok(())
    }
}