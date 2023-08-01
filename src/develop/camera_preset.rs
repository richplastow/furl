//! Used during development to view the Scene with alternate camera presets.

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug,PartialEq)]
pub enum CameraPreset {
    ChosenByScene,
    OrthographicFront,
    OrthographicLeft,
    OrthographicTop,
}
