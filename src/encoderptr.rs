use helpers::*;
use super::*;
use std::ptr;

pub struct Ptr(pub *mut ffi::ISVCEncoder);

impl Ptr {
    pub fn new() -> Result<Self, CMError> {
        let mut enc = ptr::null_mut();
        unsafe {
            try(ffi::WelsCreateSVCEncoder(&mut enc))?;
        }

        Ok(Ptr(enc))
    }
}

pub trait IntoPtr {
    fn into_inner(self) -> (SEncParamExt, Ptr);
}

impl Drop for Ptr {
    fn drop(&mut self) {
        unsafe {
            ffi::WelsDestroySVCEncoder(self.0);
        }
    }
}
