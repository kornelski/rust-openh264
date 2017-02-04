pub use ::std::os::raw::c_int as int;
pub use ::std::os::raw::c_void;

use err;

macro_rules! cpp {
    ( $object_ptr:expr => $method_name:ident( $( $params:expr ),* ) ) => {{
        let ptr = $object_ptr;
        assert!(!ptr.is_null());
        ((**ptr).$method_name.unwrap())(ptr, $( $params ),*)
    }};
    ( $obj:ident.$met:ident( $( $params:expr ),* ) ) => {
        cpp!( $obj => $met ( $( $params ),* ))
    };
}

#[must_use]
pub fn try(res: int) -> Result<(), err::Error> {
    if res == 0 {Ok(())} else {Err(res.into())}
}
