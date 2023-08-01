use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug)]
pub enum SceneContainerName {
    BlueRedBoxes,
    Empty,
    AloneFurl,
    RainbowCactus,
}
