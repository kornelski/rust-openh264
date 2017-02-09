use super::*;
use layers::*;

pub struct FrameInfo {
    pub inner: ffi::SFrameBSInfo,
}

impl FrameInfo {
    pub fn new() -> Self {
        FrameInfo {
            inner: Default::default()
        }
    }

    pub fn layers(&self) -> LayersIter {
        LayersIter::new(&self.inner.sLayerInfo[..self.inner.iLayerNum as usize])
    }

    pub fn frame_type(&self) -> EVideoFrameType {
        self.inner.eFrameType
    }

    pub fn frame_size_bytes(&self) -> usize {
        self.inner.iFrameSizeInBytes as usize
    }

    pub fn timestamp_ms(&self) -> u64 {
        self.inner.uiTimeStamp as u64
    }
}
