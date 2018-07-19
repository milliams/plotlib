use failure;
use std::result;

pub type Result<T> = result::Result<T, failure::Error>;
