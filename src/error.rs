use helpers::*;
use super::*;
use std::mem;
use std::fmt;
use std::error;

#[derive(Clone,Debug,PartialEq)]
pub enum CMError {
    InitParaError,
    UnknownReason,
    MallocMemeError,
    InitExpected,
    UnsupportedData,
    Other(int),
}

impl From<int> for CMError {
    fn from(value: int) -> Self {
        use ffi::CM_RETURN::*;
        match unsafe { mem::transmute(value) } {
            cmInitParaError => CMError::InitParaError,
            cmUnknownReason => CMError::UnknownReason,
            cmMallocMemeError => CMError::MallocMemeError,
            cmInitExpected => CMError::InitExpected,
            cmUnsupportedData => CMError::UnsupportedData,
            n => CMError::Other(n as int),
        }
    }
}

impl fmt::Display for CMError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, formatter)
    }
}

impl error::Error for CMError {
    fn description(&self) -> &str {
        match *self {
            CMError::InitParaError => "InitParaError",
            CMError::UnknownReason => "UnknownReason",
            CMError::MallocMemeError => "MallocMemeError",
            CMError::InitExpected => "InitExpected",
            CMError::UnsupportedData => "UnsupportedData",
            CMError::Other(_) => "Other",
        }
    }
}
