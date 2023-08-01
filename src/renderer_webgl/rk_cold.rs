//! A kit of ‘cold path’ operations for the Renderer.

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{HtmlCanvasElement,WebGlRenderingContext as GL};
use crate::error::{ERROR as E,error_to_string as e,SLOWLY_GET_ERROR_COLD_PATH};


// Declare the type of the `ANGLE_instanced_arrays` WebGL extension.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = ANGLE_instanced_arrays)]
    pub type AngleInstancedArrays;

    // developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays/drawArraysInstancedANGLE
    #[wasm_bindgen(method)]
    pub fn drawArraysInstancedANGLE(
        this: &AngleInstancedArrays,
        mode: u32, // eg WebGLRenderingContext::TRIANGLES
        first: u32,
        count: u32,
        primcount: u32,
    );

    // developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays/drawElementsInstancedANGLE
    #[wasm_bindgen(method)]
    pub fn drawElementsInstancedANGLE(
        this: &AngleInstancedArrays,
        mode: u32, // eg WebGLRenderingContext::TRIANGLES
        count: i32,
        type_: u32, // eg WebGLRenderingContext::UNSIGNED_SHORT
        offset: i32, // in bytes, so must be a multiple of the size of the given type
        primcount: u32,
    );

    // developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays/vertexAttribDivisorANGLE
    #[wasm_bindgen(method)]
    pub fn vertexAttribDivisorANGLE(
        this: &AngleInstancedArrays,
        index: u32,
        divisor: u32,
    );
}


/// #### A kit of ‘cold path’ operations for the Renderer.
/// 
/// These only need to be executed once — as the app initialises.
pub struct RkCold;

impl RkCold {


    /// Initialises the WebGL context.
    pub fn init_context(
        canvas: &HtmlCanvasElement,
    ) -> GL {
        let gl: GL =
            canvas
                .get_context("webgl")
                .unwrap()
                .unwrap()
                .dyn_into::<GL>() // `dyn_into` is provided by JsCast
                .unwrap();

        // Ask WebGL for its most recent error.
        if SLOWLY_GET_ERROR_COLD_PATH {
            let e = gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("initializing {}", e) }
        }

        // The WebGL context has been successfully initialized.
        gl
    }


    /// Make sure the WebGL context can handle enough vertex attributes.
    pub fn check_max_vertex_attributes(
        gl: &GL,
        needed_max: u16,
    ) {
        let actual_max =
            gl.get_parameter(GL::MAX_VERTEX_ATTRIBS)
                .unwrap()
                .as_f64()
                .expect(e(E::R22860)) as u16;

        if SLOWLY_GET_ERROR_COLD_PATH {
            let e = gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("get_parameter() {}", e) }
        }

        if actual_max < needed_max {
            panic!("{}: {} < {}", e(E::R22863), actual_max, needed_max);
        }
    }


    /// Initialises the `ANGLE_instanced_arrays` WebGL extension
    pub fn init_extensions(
        gl: &GL,
    ) -> AngleInstancedArrays {

        // Check that the extension is present on the device. @TODO
        // info(&format!("{:?}",gl.get_supported_extensions()));

        let ext_instanced_arrays =
            gl.get_extension("ANGLE_instanced_arrays")
                .unwrap()
                .expect(e(E::R22870))
                .unchecked_into::<AngleInstancedArrays>();

        if SLOWLY_GET_ERROR_COLD_PATH {
            let e = gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("get_extension() {}", e) }
        }
    
        // let property = JsValue::from_str("drawArraysInstancedANGLE");
        // let value = js_sys::Reflect::get(&ext_instanced_arrays, &property);
        // console::log_1(&ext_instanced_arrays);
        // console::log_1(&property);
        // console::log_1(&value.unwrap());

        // The `ANGLE_instanced_arrays` extension has been successfully initialized.
        ext_instanced_arrays
    }


    /// Initialise the graphics pipeline.
    pub fn init_pipeline(
        gl: &GL,
    ) {

        // @TODO investigate what viewport() does
        // gl.viewport(200, 200, 500, 500);
        // gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);
        // gl.viewport(0, 0, canvas_extent_horizontal, canvas_extent_vertical);
    
        // // Allow semitransparent fills — this has a performance hit though!
        // // Also, define how blending should work. 
        // gl.enable(GL::BLEND);
        // gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);

        // Tell WebGL to test the depth when drawing, so if a triangle is behind
        // another triangle it won't be drawn.
        gl.enable(GL::DEPTH_TEST);
        if SLOWLY_GET_ERROR_COLD_PATH {
            let e = gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("enable(GL::DEPTH_TEST) {}", e) }
        }
        gl.depth_func(GL::LEQUAL);
        if SLOWLY_GET_ERROR_COLD_PATH {
            let e = gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("depth_func() {}", e) }
        }
        
        // Pipeline Stage 1: Color Clearing
        // Define what colour should be used when the canvas is cleared.
        // Also, set the clear-depth to its maximum, which will clear everything.
        // developer.mozilla.org/en-US/docs/Web/API/WebGL_API/By_example/Clearing_with_colors
        gl.clear_color(0.1, 0.05, 0.15, 1.0); // rgba
        if SLOWLY_GET_ERROR_COLD_PATH {
            let e = gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("clear_color() {}", e) }
        }
        gl.clear_depth(1.0);
        if SLOWLY_GET_ERROR_COLD_PATH {
            let e = gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("clear_depth() {}", e) }
        }

        // Pipeline Stage 2: Scissoring
        // Enable scissoring, and define the position and size of its area.
        // gl.enable(GL::SCISSOR_TEST);
        // gl.scissor(40, 20,
        //     (canvas_extent_horizontal - 70.0) as i32, // 30px right-border
        //     (canvas_extent_vertical - 30.0) as i32); // 10px tp[-border

        // Pipeline Stage 3: Colour Masking
        // Switch off the blue channel, just for fun!
        // developer.mozilla.org/en-US/docs/Web/API/WebGL_API/By_example/Color_masking
        // gl.color_mask(true, true, false, true);

    }
}
