use super::*;
use encoderptr::*;
use helpers::*;

pub struct ExtendedParams {
    pub inner: SEncParamExt,
    ptr: Option<Ptr>,
}

impl ExtendedParams {
    pub fn new() -> Result<Self, CMError> {
        let ptr = Ptr::new()?;

        let mut inner = Default::default();
        unsafe {
            try(cpp!((ptr).GetDefaultParams(&mut inner)))?;
        }

        Ok(ExtendedParams {
            inner: inner,
            ptr: Some(ptr),
        })
    }

    pub fn encoder(self) -> Result<SVCEncoder, CMError> {
        SVCEncoder::new_ext(self)
    }
}

impl IntoPtr for ExtendedParams {
    fn into_inner(mut self) -> (SEncParamExt, Ptr) {
        (self.inner, self.ptr.take().unwrap())
    }
}

#[test]
fn test_param() {
    let mut param = ExtendedParams::new().unwrap();
    param.inner.iUsageType = EUsageType::CAMERA_VIDEO_REAL_TIME;
    param.inner.fMaxFrameRate = 60.;
    param.inner.iPicWidth = 320;
    param.inner.iPicHeight = 200;
    param.inner.iTargetBitrate = 1000000;
    param.inner.bEnableDenoise = true;
    param.inner.iSpatialLayerNum = 1;

    param.inner.sSpatialLayers[0].iVideoWidth = 320;
    param.inner.sSpatialLayers[0].iVideoHeight = 200;
    param.inner.sSpatialLayers[0].fFrameRate = 30.;
    param.inner.sSpatialLayers[0].iSpatialBitrate = 100000;

    param.inner.sSpatialLayers[0].sSliceArgument.uiSliceMode = SliceModeEnum::SM_SINGLE_SLICE;
    // unfinished
}
