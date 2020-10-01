mod utils;

use wasm_bindgen::prelude::*;
use js_sys::Array;
use serde::ser::{Serialize, Serializer, SerializeStruct};

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

#[wasm_bindgen]
pub fn find_text_segments(image_data: Vec<u8>, width: i32, height: i32) -> Array {
    let mut result: Vec<Segment> = Vec::new();

    let final_result = Array::new_with_length(result.len() as u32);
    for (index, tmp_segment) in result.iter().enumerate() {
        let json = JsValue::from_serde(tmp_segment)
            .expect("Could not turn segment into json");
        final_result.set(index as u32, json)
    }
    return final_result;
}
