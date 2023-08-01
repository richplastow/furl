//! Used during development to override the automatic level-of-detail choice.

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug,PartialEq)]
pub enum LodPreset {
    ChosenByScene,
    /// All meshes in the Scene are set to their minimum level-of-detail.
    All0,
    /// All meshes are set to a low (but not minimum) level-of-detail.
    All1,
}
