use helpers::*;
use super::*;
use std::mem;

#[derive(Clone,Debug,PartialEq)]
pub enum Error {
    InitParaError,
    UnknownReason,
    MallocMemeError,
    InitExpected,
    UnsupportedData,
    Other(int),
}

impl From<int> for Error {
    fn from(value: int) -> Self {
        use ffi::CM_RETURN::*;
        match unsafe { mem::transmute(value) } {
            cmInitParaError => Error::InitParaError,
            cmUnknownReason => Error::UnknownReason,
            cmMallocMemeError => Error::MallocMemeError,
            cmInitExpected => Error::InitExpected,
            cmUnsupportedData => Error::UnsupportedData,
            n => Error::Other(n as int),
        }
    }
}
