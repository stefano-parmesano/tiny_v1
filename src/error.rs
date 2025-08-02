use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct TinyV1Error<'a>(pub &'a str);

impl<'a> Display for TinyV1Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'a> Error for TinyV1Error<'a> {}
