mod utils;

use wasm_bindgen::prelude::*;
use js_sys::Array;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::cmp;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Segment {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub width: i32,
    pub height: i32,
    pub area: i32,
}

impl Serialize for Segment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Segment", 7)?;
        state.serialize_field("x1", &self.x1)?;
        state.serialize_field("y1", &self.y1)?;
        state.serialize_field("x2", &self.x2)?;
        state.serialize_field("y2", &self.y2)?;
        state.serialize_field("width", &self.width)?;
        state.serialize_field("height", &self.height)?;
        state.serialize_field("area", &self.area)?;
        state.end()
    }
 
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

fn get_index(x: &i32, y: &i32, width: i32, part: i32) -> usize {
    return (((y * width) + x) * 4 + part) as usize;
}

pub fn grayscale(image: &mut Vec<u8>, width: i32, height: i32) {
    for y in 0..height {
        for x in 0..width {
            let r = image[get_index(&x, &y, width, 0)] as f32;
            let g = image[get_index(&x, &y, width, 1)] as f32;
            let b = image[get_index(&x, &y, width, 2)] as f32;

            let r = (r * 0.3) as u8;
            let g = (g * 0.59) as u8;
            let b = (b * 0.11) as u8;

            image[get_index(&x, &y, width, 0)] = r;
            image[get_index(&x, &y, width, 1)] = g;
            image[get_index(&x, &y, width, 2)] = b;
        }
    }
}

pub fn thresholding(image: &mut Vec<u8>, width: i32, height: i32, threshold: u8) {
    grayscale(image, width, height);

    for y in 0..height {
        for x in 0..width {
            let gray = image[get_index(&x, &y, width, 0)] as u8;
            let alpha = image[get_index(&x, &y, width, 3)] as u8;

            let value = 255 * (gray < threshold) as u8;
            image[get_index(&x, &y, width, 0)] = value;
            image[get_index(&x, &y, width, 1)] = value;
            image[get_index(&x, &y, width, 2)] = value;
            image[get_index(&x, &y, width, 3)] = alpha;
        }
    }
}

fn find_text(image: Vec<u8>, height: i32, width: i32, max_white_space: i32, max_font_line_width: i32, min_text_width: i32) -> Vec<Segment> {
    let mut result: Vec<Segment> = Vec::new();

    let mut segments: Vec<Vec<[i32;4]>> = Vec::new();
    let mut raw_segments: Vec<Vec<[i32;4]>> = Vec::new();
    for _ in 0..height {
        segments.push(Vec::new());
        raw_segments.push(Vec::new());
    }

    let mut pattern_start_x = -1;
    let mut pattern_length = 0;
    let mut white_pixels = 0;
    let mut black_pixels = 0;

    for y in 0..height {
        for x in 0..width {
            let color = image[get_index(&x, &y, width, 0)] as u8;

            if color == 255 && pattern_start_x != -1 {
                white_pixels += 1;
                black_pixels = 0;
            } else if color == 0 {
                black_pixels += 1;
                
                if pattern_start_x == -1 {
                    pattern_start_x = x;
                }

                white_pixels = 0;
            }

            if white_pixels > max_white_space || black_pixels > max_font_line_width || x == width - 1 {
                if pattern_length >= min_text_width {
                    segments[y as usize].push([pattern_start_x, y, pattern_start_x + pattern_length, y]);
                }

                white_pixels = 0;
                black_pixels = 0;
                pattern_length = 0;
                pattern_start_x = -1;
            }

            if pattern_start_x != -1 {
                pattern_start_x += 1;
            }
        }
    }

    for y in 0..height-2 {
        let list_y: &Vec<[i32; 4]> = &segments[y as usize];

        for raw_w in 0..2 {
            let w = y + raw_w;

            let list_w: &Vec<[i32; 4]> = &segments[w as usize];

            let mut i = 0;
            while i < list_y.len() {
                let s_a = &list_y[i as usize];

                let mut j = 0;
                while j < list_w.len() {
                    let s_b = &list_w[j as usize];

                    if 
                        (s_a[0] <= s_b[0] && s_a[2] >= s_b[2]) ||
                        (s_a[0] >= s_b[0] && s_a[0] <= s_b[2]) ||
                        (s_a[2] >= s_b[0] && s_a[2] <= s_b[2])
                    {
                        let result = [
                            cmp::min(s_a[0], s_b[0]),
                            s_a[1],
                            cmp::min(s_a[2], s_b[2]),
                            s_b[3],
                        ];

                        raw_segments[y as usize].push(result);
                        break;
                    }

                    j += 1;
                }
                
                i += 1;
            }
        }
    }

    for y in 0..height {
        let list: &Vec<[i32;4]> = &raw_segments[y as usize];

        for seg in list {
            result.push(Segment{
                x1: seg[0],
                y1: seg[1],
                x2: seg[2],
                y2: seg[3],
                width: seg[2] - seg[0],
                height: seg[3] - seg[1],
                area: (seg[2] - seg[0]) * (seg[3] - seg[1]),
            });
        }
    }

    return result;
}

#[wasm_bindgen]
pub fn find_text_segments(
    mut image_data: Vec<u8>,
    width: i32,
    height: i32,
    max_white_space: i32,
    max_font_line_width: i32,
    min_text_width: i32,
    gray_scale_threshold: u8
) -> Array {
    thresholding(&mut image_data, width, height, gray_scale_threshold);

    let result = find_text(image_data, height, width, max_white_space, max_font_line_width, min_text_width);

    let final_result = Array::new_with_length(result.len() as u32);
    for (index, tmp_segment) in result.iter().enumerate() {
        let json = JsValue::from_serde(tmp_segment)
            .expect("Could not turn segment into json");
        final_result.set(index as u32, json)
    }
    return final_result;
}
