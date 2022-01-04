//! A boilerplate internal module for a private API

/// Returns the result of adding two integers.
/// ```
/// use anykit::math;
/// 
/// assert_eq!(math::add(1,0), 1);
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smoke() {
        assert_eq!(add(1, 1), 2);
    }
}