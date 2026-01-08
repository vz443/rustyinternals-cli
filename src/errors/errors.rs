use std::array::TryFromSliceError;

pub enum ParseError {
    IncorrectSliceLength,
}

impl From<TryFromSliceError> for ParseError {
    fn from(_: TryFromSliceError) -> Self {
        ParseError::IncorrectSliceLength
    }
}

pub enum PeError { // add more as error handling goes on. 
    DosHeaderParseError,
    FileHeaderParseError,
}