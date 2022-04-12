use std::num::ParseFloatError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("CSV parsing error: {error:?}")]
    CsvError {
        #[from]
        error: csv::Error,
    },

    #[error("Header row not found in input data")]
    HeaderRowNotFound,

    // TODO: include row number?
    #[error("Missing field #{0}")]
    MissingField(u8),

    #[error("Parse float error: {error:?}")]
    ParseFloatError {
        #[from]
        error: ParseFloatError,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
