pub use ::std::os::raw::c_int as int;
pub use ::std::os::raw::c_void;

use error::CMError;

macro_rules! cpp {
    ( ( $object_ptr:expr ) . $method_name:ident( $( $params:expr ),* ) ) => {{
        let raw_vtable = $object_ptr.0;
        assert!(!raw_vtable.is_null());
        ((**raw_vtable).$method_name.unwrap())(raw_vtable, $( $params ),*)
    }};
    ( $obj:ident.$met:ident( $( $params:expr ),* ) ) => {
        cpp!( ($obj . ptr) . $met ( $( $params ),* ))
    };
}

#[must_use]
pub fn try(res: int) -> Result<(), CMError> {
    if res == 0 {Ok(())} else {Err(res.into())}
}
