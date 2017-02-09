use ffi;
use super::*;
use helpers::*;
use std::iter;
use std::slice;

#[derive(Clone,Debug)]
pub struct LayerInfo<'a> {
    inner: &'a ffi::SLayerBSInfo,
}

impl<'a> LayerInfo<'a> {
    pub fn new(info: &'a ffi::SLayerBSInfo) -> Self {
        LayerInfo {
            inner: info,
        }
    }

    pub fn temporal_id(&self) -> u8 {
        self.inner.uiTemporalId
    }
    pub fn spatial_id(&self) -> u8 {
        self.inner.uiSpatialId
    }
    pub fn quality_id(&self) -> u8 {
        self.inner.uiQualityId
    }
    pub fn frame_type(&self) -> EVideoFrameType {
        self.inner.eFrameType
    }
    pub fn layer_type(&self) -> u8 {
        self.inner.uiLayerType
    }
    /// refer to D.2.11 Sub-sequence information SEI message semantics
    pub fn subseq_id(&self) -> ::std::os::raw::c_int {
        self.inner.iSubSeqId
    }
    /// count number of NAL coded already
    pub fn nal_count(&self) -> ::std::os::raw::c_int {
        self.inner.iNalCount
    }
    /// length of NAL size in byte from 0 to iNalCount-1
    pub fn nal_byte_size(&self) -> usize {
        unsafe {*self.inner.pNalLengthInByte as usize}
    }
    /// buffer of bitstream contained
    pub fn buf(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.inner.pBsBuf, self.nal_byte_size())
        }
    }

}

pub struct LayersIter<'a> {
    layers: iter::Map<slice::Iter<'a, SLayerBSInfo>, fn(&'a ffi::SLayerBSInfo) -> LayerInfo<'a>>,
}

impl<'a> LayersIter<'a> {
    pub fn new(layers: &'a [SLayerBSInfo]) -> Self {
        LayersIter {
            layers: layers.iter().map(LayerInfo::new),
        }
    }
}

impl<'a> Iterator for LayersIter<'a> {
    type Item = LayerInfo<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.layers.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.layers.size_hint()
    }

    fn count(self) -> usize {
        self.layers.count()
    }
}
