//! Used during development to view the Scene with grids and axes.

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug,PartialEq)]
pub enum GuidesPreset {
    ChosenByScene,
    NoGuides,
    All10m,
    All1m,
    AxesOnly10m,
    AxesOnly1m,
    GridsOnly10m,
    GridsOnly1m,
}
