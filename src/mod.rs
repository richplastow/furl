use wasm_bindgen::prelude::wasm_bindgen;

mod app;
pub use app::App;

mod error;
mod node;
mod renderer_webgl;
mod scene;
mod shape;


// You can write `crate::info("hullo")` anywhere in code.
mod develop;
pub use develop::{error,info,log};


#[wasm_bindgen]
pub fn greet() -> String {
  return "Hello!".to_string();
}
