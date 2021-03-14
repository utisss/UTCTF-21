use std::{
    env::VarError,
    error::Error,
    fmt::{
        self,
        Debug,
        Display,
        Formatter,
    },
    str::FromStr,
};

use super::var_error_wrapper::VarErrorWrapper;

pub enum EnvironmentParseError<'a, T>
where
    T: FromStr,
    <T as FromStr>::Err: Error,
{
    ParseError(<T as FromStr>::Err),
    VarError(VarErrorWrapper<'a>),
}

impl<T> Error for EnvironmentParseError<'_, T>
where
    T: FromStr,
    <T as FromStr>::Err: Error,
{
}

impl<T> Debug for EnvironmentParseError<'_, T>
where
    T: FromStr,
    <T as FromStr>::Err: Error,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(err) => write!(f, "{:?}", err),
            Self::VarError(err) => write!(f, "{:?}", err),
        }
    }
}

impl<T> Display for EnvironmentParseError<'_, T>
where
    T: FromStr,
    <T as FromStr>::Err: Error,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(err) => write!(f, "{}", err),
            Self::VarError(err) => write!(f, "{}", err),
        }
    }
}

impl<'a, T> From<(VarError, &'a str)> for EnvironmentParseError<'a, T>
where
    T: FromStr,
    <T as FromStr>::Err: Error,
{
    fn from((err, name): (VarError, &'a str)) -> Self {
        let temp: VarErrorWrapper = (err, name).into();
        temp.into()
    }
}

impl<'a, T> From<VarErrorWrapper<'a>> for EnvironmentParseError<'a, T>
where
    T: FromStr,
    <T as FromStr>::Err: Error,
{
    fn from(err: VarErrorWrapper<'a>) -> Self {
        EnvironmentParseError::VarError(err)
    }
}
