use wasm_bindgen::prelude::wasm_bindgen;

// For a discussion of this technique, see https://youtu.be/p7DtoeuDT5Y?t=1279
#[wasm_bindgen]
extern "C" {

    /// A developer utility for logging errors to the browser console.
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);

    /// A developer utility for logging info messages to the browser console.
    #[wasm_bindgen(js_namespace = console)]
    pub fn info(s: &str);

    /// A developer utility for logging general messages to the browser console.
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

}

// Alternatively:
// pub fn log(message: String) {
//     console::log_1(&message.into());
// }
//
// And in Cargo.toml:
// features = [ 
//   ...
//   'console',
//   ...
// ]
//
