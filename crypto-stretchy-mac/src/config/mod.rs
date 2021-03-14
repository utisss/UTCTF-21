use std::fmt::{
    self,
    Display,
    Formatter,
};

use crate::config::errors::ConfigBuilderError;

pub mod errors;

#[derive(Debug)]
pub struct ConfigBuilder<'a> {
    flag: Option<&'a str>,
    read_timeout: Option<u64>,
    minimum_key_size: Option<u8>,
    maximum_key_size: Option<u8>,
}

impl Default for ConfigBuilder<'_> {
    fn default() -> Self {
        Self {
            flag: None,
            read_timeout: None,
            minimum_key_size: None,
            maximum_key_size: None,
        }
    }
}

impl<'a> ConfigBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_flag(&mut self, flag: &'a str) -> &mut Self {
        self.flag = Some(flag);

        self
    }

    pub fn set_read_timeout(&mut self, read_timeout: u64) -> &mut Self {
        self.read_timeout = Some(read_timeout);

        self
    }

    pub fn set_minimum_key_size(&mut self, minimum_key_size: u8) -> &mut Self {
        self.minimum_key_size = Some(minimum_key_size);

        self
    }

    pub fn set_maximum_key_size(&mut self, maximum_key_size: u8) -> &mut Self {
        self.maximum_key_size = Some(maximum_key_size);

        self
    }

    pub fn finalize(&self) -> Result<Config<'a>, ConfigBuilderError> {
        let flag = self.flag.ok_or(ConfigBuilderError::MissingFlag)?;
        let read_timeout = self
            .read_timeout
            .ok_or(ConfigBuilderError::MissingReadTimeout)?;
        let minimum_key_size = self
            .minimum_key_size
            .ok_or(ConfigBuilderError::MissingMinimumKeySize)?;
        let maximum_key_size = self
            .maximum_key_size
            .ok_or(ConfigBuilderError::MissingMaximumKeySize)?;

        Ok(Config {
            flag,
            read_timeout,
            minimum_key_size,
            maximum_key_size,
        })
    }
}

#[derive(Debug)]
pub struct Config<'a> {
    flag: &'a str,
    read_timeout: u64,
    minimum_key_size: u8,
    maximum_key_size: u8,
}

impl Config<'_> {
    pub fn flag(&self) -> &str {
        &self.flag
    }

    pub fn read_timeout(&self) -> u64 {
        self.read_timeout
    }

    pub fn minimum_key_size(&self) -> u8 {
        self.minimum_key_size
    }

    pub fn maximum_key_size(&self) -> u8 {
        self.maximum_key_size
    }
}

impl Display for Config<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Configuration:\n\
            ==> Flag:             {}\n\
            ==> Read Timeout:     {}\n\
            ==> Minimum Key Size: {}\n\
            ==> Maximum Key Size: {}\n\
            ",
            self.flag,
            self.read_timeout,
            self.minimum_key_size,
            self.maximum_key_size,
        )
    }
}
