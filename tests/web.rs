//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use js_sys::{JsString, JSON};
use tangibl_wasm::{parse, JsonTopCode};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn parses_a_start_token() -> Result<(), JsValue> {
    let topcodes = serde_wasm_bindgen::to_value(&vec![JsonTopCode {
        code: 61,
        unit: 8.0,
        orientation: 0.0,
        x: 0.0,
        y: 0.0,
    }])
    .unwrap();
    eprintln!("{:?}", topcodes);
    let result = parse(topcodes);
    assert_eq!(
        JsString::from("{\"name\":\"start\"}"),
        JSON::stringify(&result)?
    );
    Ok(())
}
