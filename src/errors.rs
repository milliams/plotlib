use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Cannot save page to svg!")]
    FailedToSave(#[from] std::io::Error),
    #[error("Invalid {name:?} range: {lower} <= {upper}. Please specify the {name:?} range manually.")]
    InvalidRange {
        name: String,
        lower: f64,
        upper: f64
    },
}

pub type Result<T> = ::std::result::Result<T, Error>;
