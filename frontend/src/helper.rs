/*!
    Useful helper functions
*/

/// Converts a String to a &'static str.
/// 
/// ## Disclaimer
/// This uses `Box::leak()`. Proceed with caution.
pub unsafe fn convert_string_to_static_str(string: String) -> &'static str {
    // `Box::leak()` can memory leak if not used approipriately by caller
    unsafe {
        let b = Box::leak(Box::new(string));
        b.as_str()
    }
}
