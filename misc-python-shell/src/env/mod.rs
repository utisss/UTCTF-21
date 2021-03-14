use std::{
    env,
    env::VarError,
    error::Error,
    str::FromStr,
};

pub use crate::env::environment_parse_error::EnvironmentParseError;

mod environment_parse_error;
mod var_error_wrapper;

pub fn get_var<T>(
    var_name: &str, default: Option<T>,
) -> Result<T, EnvironmentParseError<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Error,
{
    match env::var(var_name) {
        Ok(val) => match val.parse() {
            Ok(val) => Ok(val),
            Err(err) => Err(EnvironmentParseError::ParseError(err)),
        },
        Err(err) => match err {
            VarError::NotPresent => match default {
                Some(default) => Ok(default),
                None => Err((err, var_name).into()),
            },
            VarError::NotUnicode(_) => Err((err, var_name).into()),
        },
    }
}
