use derive_more::{Display, Error};
use std::{error::Error, fmt::Display};

#[derive(Debug, Display, Error)]
#[display("{name}: {source}")]
pub struct MyError<Name: Display, Source: Error> {
    pub name: Name,
    #[error(source)]
    pub source: Source,
}
