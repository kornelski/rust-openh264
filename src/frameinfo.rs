use super::*;

pub struct FrameInfo {
    pub inner: ffi::SFrameBSInfo,
}

impl FrameInfo {
    pub fn new() -> Self {
        FrameInfo {
            inner: Default::default()
        }
    }

    pub fn frame_size_bytes(&self) -> usize {
        self.inner.iFrameSizeInBytes as usize
    }

    pub fn timestamp_ms(&self) -> u64 {
        self.inner.uiTimeStamp as u64
    }
}
