use wasm_bindgen::prelude::*;
use tailwindcss_oxide::parser::*;

fn run(input: &str, loose: bool) -> Vec<&str> {
    Extractor::unique(
        input.as_bytes(),
        ExtractorOptions {
            preserve_spaces_in_arbitrary: loose,
        },
    )
    .into_iter()
    .map(|s| unsafe { std::str::from_utf8_unchecked(s) })
    .collect()
}

// Return array of strings from the the run function, which will be returned to JS
#[wasm_bindgen]
pub fn find_tw_candidates(input: &str, loose: bool) -> Vec<JsValue> {
    run(input, loose).into_iter().map(JsValue::from).collect()
}