use wasm_bindgen::prelude::*;
use tailwindcss_oxide::extractor::{Extractor, Extracted};

// See tailwindcss-oxide/src/main.rs 
fn run(input: &str) -> Vec<&str> {
    Extractor::new(input.as_bytes())
        .extract()
        .into_iter()
        .map(|x| match x {
            Extracted::Candidate(bytes) => unsafe { std::str::from_utf8_unchecked(bytes) },
            Extracted::CssVariable(bytes) => unsafe { std::str::from_utf8_unchecked(bytes) },
        })
        .collect::<Vec<_>>()
}

// WASM-compatible function for JavaScript
#[wasm_bindgen]
pub fn find_tw_candidates(input: &str) -> Vec<JsValue> {
    run(input).into_iter().map(JsValue::from).collect()
}

// PHP FFI-compatible function
#[no_mangle]
pub extern "C" fn find_tw_candidates_ffi(
    input: *const u8,
    input_len: usize,
) -> *mut *mut u8 {
    let input = unsafe { std::slice::from_raw_parts(input, input_len) };
    let input_str = unsafe { std::str::from_utf8_unchecked(input) };

    let results = run(input_str);

    // Create a vector of C-style strings (null-terminated)
    let mut c_strings: Vec<*mut u8> = results
        .into_iter()
        .map(|s| {
            let mut c_string = s.as_bytes().to_vec();
            c_string.push(0); // Null-terminate the string
            let ptr = c_string.as_mut_ptr();
            std::mem::forget(c_string); // Prevent Rust from deallocating
            ptr
        })
        .collect();

    // Push a null pointer as the final element to mark the end of the array
    c_strings.push(std::ptr::null_mut());

    let c_strings_ptr = c_strings.as_mut_ptr();
    std::mem::forget(c_strings); // Prevent Rust from deallocating
    c_strings_ptr
}

// Function to free memory allocated by `find_tw_candidates_ffi`
#[no_mangle]
pub extern "C" fn free_candidates(ptr: *mut *mut u8) {
    unsafe {
        if ptr.is_null() {
            return;
        }

        let mut current = ptr;
        while !(*current).is_null() {
            let _ = Box::from_raw(*current);
            current = current.add(1);
        }
        let _ = Box::from_raw(ptr);
    }
}
