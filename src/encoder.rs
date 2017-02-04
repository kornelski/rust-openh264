use super::*;
use helpers::*;
use picture::*;
use err;

pub struct SVCEncoder {
    ptr: *mut ffi::ISVCEncoder,
}

impl SVCEncoder {
    pub fn new(params: &SEncParamBase) -> Result<Self, err::Error> {
        let mut ptr = ptr::null_mut();

        unsafe {
            try(ffi::WelsCreateSVCEncoder(&mut ptr))?;
            try(cpp!(ptr.Initialize(params)))?;
        };

        Ok(SVCEncoder {
            ptr: ptr,
        })
    }

    pub fn encode_frame(&mut self, picture: &Picture, info: &mut SFrameBSInfo) -> Result<(), err::Error> {
        unsafe {
            try(cpp!(self.ptr=>EncodeFrame(&picture.inner, info)))
        }
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
            let ptr = self.ptr;
            cpp!(ptr.SetOption(option, value as *const _ as *mut c_void)) == 0
        }
    }
}

#[test]
fn test() {
    let mut param = SEncParamBase::default();
    param.iUsageType = EUsageType::CAMERA_VIDEO_REAL_TIME;
    param.fMaxFrameRate = 60.;
    param.iPicWidth = 320;
    param.iPicHeight = 200;
    param.iTargetBitrate = 1000000;

    let mut enc = SVCEncoder::new(&param).expect("create encoder");

    enc.set_trace_level(1);
    enc.set_data_format(EVideoFormatType::videoFormatI420);

    let width = 320;
    let height = 200;

    let luma = vec![128u8; width*height];
    let cbcr = vec![128u8; (width/2)*(height/2)];

    let mut info = Default::default();
    let pic = Picture::new(width, height, EVideoFormatType::videoFormatI420, 0, &[
        (width, &luma),
        (width/2, &cbcr),
        (width/2, &cbcr),
    ]).expect("create pic");

    enc.encode_frame(&pic, &mut info).expect("encode frame");

//       ASSERT_TRUE (rv == cmResultSuccess);
//       if (info.eFrameType != videoFrameTypeSkip && cbk != NULL) {
//        //output bitstream
//       }
//    }
//
//   Step5:teardown encoder
//   @code
//    if (encoder_) {
//        encoder_->Uninitialize();
//        WelsDestroySVCEncoder (encoder_);
//    }
//   @endcode*/
}
