use anykit::math;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn sum_as_string(a: i32, b: i32) -> CString {
    // TODO: correct this impl
    // math::add(a, b)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
