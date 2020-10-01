//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use marvinj_rust::grayscale;

#[wasm_bindgen_test]
fn grayscale_test() {
    let width = 1;
    let height = 1;
    let mut input: Vec<u8> = Vec::new();
    for i in 0..(width * height * 4) {
        input.push((i * 20) as u8);
    }

    let mut result: Vec<u8> = Vec::new();
    result.push(0);
    result.push(11);
    result.push(4);
    result.push(60);

    grayscale(&mut input, width, height);

    assert_eq!(result.len(), input.len());
    for (i, _) in result.iter().enumerate() {
        assert_eq!(result[i as usize], input[i as usize]);
    }
}
