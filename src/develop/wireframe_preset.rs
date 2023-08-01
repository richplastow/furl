//! Used during development to view Scene meshes as dots or lines.

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug,PartialEq)]
pub enum WireframePreset {
    ChosenByScene,
    Dots,
    Lines,
    Solid,
}
