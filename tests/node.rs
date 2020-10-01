//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use marvinj_rust::grayscale;
use marvinj_rust::thresholding;

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

#[wasm_bindgen_test]
fn thresholding_test() {
    let threshold = 95;
    let width: i32 = 2;
    let height: i32 = 2;
    let mut input: Vec<u8> = Vec::new();
    for i in 0..(width*height*4) {
        input.push((50 + i * 5) as u8);
    }

    let mut result: Vec<u8> = Vec::new();
    result.push(0); // 50
    result.push(0); // 55
    result.push(0); // 60
    result.push(65); // 65

    result.push(0); // 70
    result.push(0); // 75
    result.push(0); // 80
    result.push(85); // 85

    result.push(0); // 90
    result.push(0); // 95
    result.push(0); // 100
    result.push(105); // 105

    result.push(255); // 110
    result.push(255); // 115
    result.push(255); // 120
    result.push(125); // 125

    thresholding(&mut input, width, height, threshold);

    assert_eq!(result.len(), input.len());

    assert_eq!(result[0], input[0]);
    assert_eq!(result[1], input[1]);
    assert_eq!(result[2], input[2]);
    assert_eq!(result[3], input[3]);
    assert_eq!(result[4], input[4]);
    assert_eq!(result[5], input[5]);
    assert_eq!(result[6], input[6]);
    assert_eq!(result[7], input[7]);
    assert_eq!(result[8], input[8]);
    assert_eq!(result[9], input[9]);
    assert_eq!(result[10], input[10]);
    assert_eq!(result[11], input[11]);
    assert_eq!(result[12], input[12]);
    assert_eq!(result[13], input[13]);
    assert_eq!(result[14], input[14]);
    assert_eq!(result[15], input[15]);
}