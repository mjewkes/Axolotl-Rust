use std::{fmt};
pub use std::error::Error as BaseError;

#[derive(Debug)]
pub enum CryptoErrorKind {
    Generic,
    Unknown,
}

#[derive(Debug)]
pub struct CryptoError {
    kind : CryptoErrorKind,
    info : &'static str,
}


impl BaseError for CryptoError {
    fn description(&self) -> &str {
        self.info
    }
}

impl CryptoError {
    pub fn new(kind: CryptoErrorKind, error: &'static str ) -> Self  {
        CryptoError { kind : kind, info: error}
    }
    pub fn kind(&self) -> &CryptoErrorKind {
        return &self.kind;
    } 
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CryptoError({:?}) : {} ", self.kind,self.description())
    }
}