use std::{
    error::Error,
    fmt::{
        self,
        Display,
        Formatter,
    },
};

#[derive(Debug)]
pub enum ConfigBuilderError {
    MissingFlag,
    MissingReadTimeout,
    MissingGameTimeout,
    MissingChallengeBytes,
}

impl Error for ConfigBuilderError {}

impl Display for ConfigBuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingFlag => write!(f, "The flag is missing."),
            Self::MissingReadTimeout =>
                write!(f, "The read_timout value is undefined."),
            Self::MissingGameTimeout =>
                write!(f, "The game_timeout value is undefined."),
            Self::MissingChallengeBytes =>
                write!(f, "The challenge_bytes value is undefined."),
        }
    }
}
