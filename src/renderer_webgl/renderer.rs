use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement,WebGlRenderingContext as GL,window};
use crate::error::{ERROR as E,error_to_string as e};
use super::rk_cold::{AngleInstancedArrays,RkCold};
use super::shader::ShaderProgram;


pub struct RendererWebGl {
    _canvas: HtmlCanvasElement, // maybe not needed?
    pub aspect_ratio: f32,
    pub canvas_extent_horizontal: f32,
    pub canvas_extent_vertical: f32,
    pub ext_instanced_arrays: AngleInstancedArrays,
    pub gl: GL,
    pub shaders: Vec<Box<dyn ShaderProgram>>,
}

impl RendererWebGl {
    pub fn new(
        canvas_extent_horizontal: f32,
        canvas_extent_vertical: f32,
        canvas_id: String,
    ) -> Self {
    
        // Initialise the canvas.
        let document = window().unwrap().document().unwrap();
        let canvas: HtmlCanvasElement =
            document.get_element_by_id(&canvas_id)
                .expect(e(E::R11820))
                .dyn_into::<HtmlCanvasElement>() // `dyn_into` is provided by JsCast
                .expect(e(E::R11833));

        // Run the ‘cold path’ — operations which only need to be executed once.
        let gl = RkCold::init_context(&canvas);
        RkCold::check_max_vertex_attributes(&gl, 16); // panic if it fails
        let ext_instanced_arrays = RkCold::init_extensions(&gl);
        RkCold::init_pipeline(&gl);

        Self {
            _canvas: canvas,
            aspect_ratio: canvas_extent_horizontal / canvas_extent_vertical,
            canvas_extent_horizontal,
            canvas_extent_vertical,
            ext_instanced_arrays,
            gl,
            shaders: vec![],
        }
    }

}
