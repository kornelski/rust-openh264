#![feature(untagged_unions)]

#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
pub mod codec_api;
pub use codec_api as ffi;


use std::mem;
use std::default::Default;

impl Default for ffi::TagEncParamBase {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[test]
fn poke_it() {
    use ffi::*;

    let mut param: SEncParamBase = Default::default();
    param.iUsageType = EUsageType::CAMERA_VIDEO_NON_REAL_TIME;
    param.fMaxFrameRate = 60.;
}
