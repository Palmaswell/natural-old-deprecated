mod math;
pub use crate::math::Vector;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let foo = Vector { x: 2.0, y: 3.0 };
    alert(&format!("Hola a todos?????, {}", name));
}
