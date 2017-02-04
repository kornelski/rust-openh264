 #![allow(unused_imports)]
 #![allow(dead_code)]
 #![allow(unused_variables)]

extern crate openh264_sys as ffi;

#[macro_use]
mod helpers;
mod encoderptr;

mod encoder;
pub use encoder::*;

mod error;
pub use error::*;

mod picture;
pub use picture::*;

mod frameinfo;
pub use frameinfo::*;

mod extendedparams;
pub use extendedparams::*;

pub fn version() -> OpenH264Version {
    unsafe {
        ffi::WelsGetCodecVersion()
    }
}

pub use ffi::SEncParamBase;
pub use ffi::EUsageType;
pub use ffi::EVideoFormatType;

pub use ffi::EVideoFrameType;
pub use ffi::EProfileIdc;
pub use ffi::ELevelIdc;
pub use ffi::SliceModeEnum;
pub use ffi::EVideoFormatSPS;
pub use ffi::EColorPrimaries;
pub use ffi::ETransferCharacteristics;
pub use ffi::EColorMatrix;
pub use ffi::ESampleAspectRatio;
pub use ffi::EParameterSetStrategy;
pub use ffi::OpenH264Version;
pub use ffi::PDecodingParam;
pub use ffi::PEncParamBase;
pub use ffi::PFrameBSInfo;
pub use ffi::PLayerBSInfo;
pub use ffi::PParserBsInfo;
pub use ffi::PRateThresholds;
pub use ffi::PSliceInfo;
pub use ffi::PVuiSarInfo;
pub use ffi::SBitrateInfo;
pub use ffi::SBufferInfo;
pub use ffi::SDecoderCapability;
pub use ffi::SDecoderStatistics;
pub use ffi::SDecodingParam;
pub use ffi::SDeliveryStatus;
pub use ffi::SDumpLayer;
pub use ffi::SEncoderStatistics;
pub use ffi::SEncParamExt;
pub use ffi::SLayerBSInfo;
pub use ffi::SLevelInfo;
pub use ffi::SliceInfo;
pub use ffi::SLTRConfig;
pub use ffi::SLTRMarkingFeedback;
pub use ffi::SLTRRecoverRequest;
pub use ffi::SParserBsInfo;
pub use ffi::SProfileInfo;
pub use ffi::SRateThresholds;
pub use ffi::SSliceArgument;
pub use ffi::SSpatialLayerConfig;
pub use ffi::SSysMEMBuffer;
pub use ffi::SVideoProperty;
pub use ffi::SVuiSarInfo;

#[test]
fn version_test() {
    let v = version();
    assert_eq!(1, v.uMajor);
}
