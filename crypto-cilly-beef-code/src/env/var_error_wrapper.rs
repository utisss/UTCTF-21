use std::{
    env::VarError,
    error::Error,
    fmt::{
        self,
        Display,
        Formatter,
    },
};

#[derive(Debug)]
pub struct VarErrorWrapper<'a> {
    name: &'a str,
    err: VarError,
}

impl Error for VarErrorWrapper<'_> {}

impl Display for VarErrorWrapper<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Accessing {} returned {}", self.name, self.err)
    }
}

impl<'a> From<(VarError, &'a str)> for VarErrorWrapper<'a> {
    fn from((err, name): (VarError, &'a str)) -> Self {
        VarErrorWrapper { err, name }
    }
}
