use helpers::*;
use super::*;
use std::marker::PhantomData;
use std::ptr;

pub struct Picture<'data> {
    pub inner: ffi::SSourcePicture,
    _datas: PhantomData<&'data [u8]>,
}

impl<'data> Picture<'data> {
    pub fn new(width: usize, heigth: usize, color_format: EVideoFormatType, timestamp_ms: u64, planes: &[(usize, &'data [u8])]) -> Option<Self> {
        let mut strides = [0; 4];
        let mut pointers = [ptr::null_mut(); 4];

        if planes.len() == 0 || planes.len() > strides.len() {
            return None;
        }

        for (i, &(stride, data)) in planes.iter().enumerate() {
            strides[i] = stride as int;
            pointers[i] = data as *const _ as *mut _;
        }

        Some(Picture {
            inner: ffi::SSourcePicture {
                iColorFormat: color_format as int,
                iStride: strides,
                pData: pointers,
                iPicWidth: width as int,
                iPicHeight: heigth as int,
                uiTimeStamp: timestamp_ms as i64,
            },
            _datas: PhantomData,
        })
    }
}
