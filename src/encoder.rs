use super::*;
use helpers::*;
use std::ptr;
use encoderptr::*;
use layers::*;

pub struct SVCEncoder {
    ptr: Ptr,
}

pub trait IntoEncoder {
    fn encoder(self) -> Result<SVCEncoder, CMError>;
}

impl IntoEncoder for SEncParamBase {
    fn encoder(self) -> Result<SVCEncoder, CMError> {
        SVCEncoder::new(&self)
    }
}

impl SVCEncoder {
    pub fn new(params: &SEncParamBase) -> Result<Self, CMError> {
        let ptr = Ptr::new()?;

        unsafe { try(cpp!((ptr).Initialize(params))) }?;

        Ok(SVCEncoder{
            ptr: ptr,
        })
    }

    pub fn new_ext(params: ExtendedParams) -> Result<Self, CMError> {
        let (mut inner, ptr) = params.into_inner();

        unsafe { try(cpp!((ptr).InitializeExt(&mut inner))) }?;

        Ok(SVCEncoder {
            ptr: ptr,
        })
    }

    pub fn encode_frame(&mut self, picture: &Picture) -> Result<FrameInfo, CMError> {
        let mut info = FrameInfo::new();
        unsafe {
            try(cpp!(self.EncodeFrame(&picture.inner, &mut info.inner)))?
        }
        Ok(info)
    }

    pub fn set_data_format(&mut self, value: EVideoFormatType) -> bool {
        self.set_option(ffi::ENCODER_OPTION_DATAFORMAT, &value)
    }

    pub fn set_trace_level(&mut self, value: u32) -> bool {
        self.set_option(ffi::ENCODER_OPTION_TRACE_LEVEL, &value)
    }

    #[inline]
    fn set_option<T>(&mut self, option: ffi::ENCODER_OPTION, value: &T) -> bool {
        unsafe {
            cpp!(self.SetOption(option, value as *const _ as *mut c_void)) == 0
        }
    }
}

impl Drop for SVCEncoder {
    fn drop(&mut self) {
        unsafe {
            cpp!(self.Uninitialize());
        }
    }
}

#[test]
fn test_encode() {
    let mut param = SEncParamBase::default();
    param.iUsageType = EUsageType::CAMERA_VIDEO_REAL_TIME;
    param.fMaxFrameRate = 60.;
    param.iPicWidth = 320;
    param.iPicHeight = 200;
    param.iTargetBitrate = 1000000;

    let mut enc = param.encoder().expect("create encoder");

    enc.set_trace_level(1);
    enc.set_data_format(EVideoFormatType::videoFormatI420);

    let width = 320;
    let height = 200;

    let luma = vec![128u8; width*height];
    let cbcr = vec![128u8; (width/2)*(height/2)];

    let ts = 1234;
    let pic = Picture::new(width, height, EVideoFormatType::videoFormatI420, ts, &[
        (width, &luma),
        (width/2, &cbcr),
        (width/2, &cbcr),
    ]).expect("create pic");

    let info = enc.encode_frame(&pic).expect("encode frame");

    assert_eq!(ts, info.timestamp_ms());
    assert!(info.frame_size_bytes() > 0);
    assert!(info.layers().count() > 0);
    for layer in info.layers() {
        assert_eq!(layer.nal_byte_size(), layer.buf().len());
    }
}
