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
    MissingRedactedFlag,
    MissingReadTimeout,
    MissingMinimumKeySize,
    MissingMaximumKeySize,
}

impl Error for ConfigBuilderError {}

impl Display for ConfigBuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingFlag => write!(f, "The flag is missing."),
            Self::MissingRedactedFlag =>
                write!(f, "The redacted_flag is missing."),
            Self::MissingReadTimeout =>
                write!(f, "The read_timout value is undefined."),
            Self::MissingMinimumKeySize =>
                write!(f, "The minimum_key_size value is undefined."),
            Self::MissingMaximumKeySize =>
                write!(f, "The maximum_key_size value is undefined."),
        }
    }
}
