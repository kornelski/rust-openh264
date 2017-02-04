use helpers::*;
use super::*;
use std::mem;

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
