use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct World {
    pub width: usize
}

#[wasm_bindgen]
impl World {
    pub fn new() -> {
        World {
            width: 8
        }
    }
}

