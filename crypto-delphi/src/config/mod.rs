use std::fmt::{
    self,
    Display,
    Formatter,
};

use crate::config::errors::ConfigBuilderError;

pub mod errors;

#[derive(Debug)]
pub struct ConfigBuilder {
    flag: Option<String>,
    read_timeout: Option<u64>,
    game_timeout: Option<u64>,
    challenge_bytes: Option<usize>,
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self {
            flag: None,
            read_timeout: None,
            game_timeout: None,
            challenge_bytes: None,
        }
    }
}

impl<'a> ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_flag(&mut self, flag: String) -> &mut Self {
        self.flag = Some(flag);

        self
    }

    pub fn set_read_timeout(&mut self, read_timeout: u64) -> &mut Self {
        self.read_timeout = Some(read_timeout);

        self
    }

    pub fn set_game_timeout(&mut self, game_timeout: u64) -> &mut Self {
        self.game_timeout = Some(game_timeout);

        self
    }

    pub fn set_challenge_bytes(&mut self, challenge_bytes: usize) -> &mut Self {
        self.challenge_bytes = Some(challenge_bytes);

        self
    }

    pub fn finalize(&self) -> Result<Config, ConfigBuilderError> {
        let flag = self.flag.clone().ok_or(ConfigBuilderError::MissingFlag)?;
        let read_timeout = self
            .read_timeout
            .ok_or(ConfigBuilderError::MissingReadTimeout)?;
        let game_timeout = self
            .game_timeout
            .ok_or(ConfigBuilderError::MissingGameTimeout)?;
        let challenge_bytes = self
            .challenge_bytes
            .ok_or(ConfigBuilderError::MissingChallengeBytes)?;

        Ok(Config {
            flag,
            read_timeout,
            game_timeout,
            challenge_bytes,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    flag: String,
    read_timeout: u64,
    game_timeout: u64,
    challenge_bytes: usize,
}

impl Config {
    pub fn flag(&self) -> &str {
        &self.flag
    }

    pub fn read_timeout(&self) -> u64 {
        self.read_timeout
    }

    pub fn game_timeout(&self) -> u64 {
        self.game_timeout
    }

    pub fn challenge_bytes(&self) -> usize {
        self.challenge_bytes
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Configuration:\n\
            ==> Flag:            {}\n\
            ==> Read Timeout:    {}\n\
            ==> Game Timeout:    {}\n\
            ==> Challenge Bytes: {}\n\
            ",
            self.flag, self.read_timeout, self.game_timeout, self.challenge_bytes,
        )
    }
}
