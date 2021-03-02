//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_bindgen::prelude::JsValue;
use js_hasher_rs::hasher;


wasm_bindgen_test_configure!(run_in_browser);


#[wasm_bindgen_test]
fn test_hasher_strings() {
    let js_value = JsValue::from_str("Testing");
    let hashed = hasher(js_value);

    assert_eq!(JsValue::from_f64(14360205181833445000f64), hashed)
}

#[wasm_bindgen_test]
fn test_hasher_strings2() {
    let js_value = JsValue::from_str("foo");
    let hashed = hasher(js_value);

    assert_eq!(JsValue::from_f64(4506850079084803000f64), hashed)
}

#[wasm_bindgen_test]
fn test_hasher_numbers() {
    let js_value = JsValue::from_f64(42f64);
    let hashed = hasher(js_value);

    assert_eq!(JsValue::from_f64(15130871412783075000f64), hashed)
}

#[wasm_bindgen_test]
fn test_hasher_bools() {
    let js_value = JsValue::from_bool(true);
    let hashed = hasher(js_value);

    assert_eq!(JsValue::from_f64(15130871412783075000f64), hashed)
}

