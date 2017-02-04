#![feature(untagged_unions)]

#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
mod codec_api;
pub use codec_api::*;

use std::mem;
use std::default::Default;

impl Default for TagEncParamBase { fn default() -> Self { unsafe { mem::zeroed() } } }
impl Default for SSourcePicture { fn default() -> Self { unsafe { mem::zeroed() } } }
impl Default for SFrameBSInfo { fn default() -> Self { unsafe { mem::zeroed() } } }

#[test]
fn poke_it() {
    let mut param: SEncParamBase = Default::default();
    param.iUsageType = EUsageType::CAMERA_VIDEO_NON_REAL_TIME;
    param.fMaxFrameRate = 60.;
}
