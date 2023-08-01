//! A kit of ‘cool path’ operations for the Renderer.

use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::{WebGlBuffer,WebGlRenderingContext as GL};

use crate::error::SLOWLY_GET_ERROR_COOL_PATH;

use super::{AttributeName,RendererWebGl,UniformName};
use super::shader::{
    ShaderBlueBox,
    ShaderFurlBasic,
    ShaderGuides,
    ShaderProgramName,
    ShaderRainbowCactus,
    ShaderRedBox
};


/// #### A kit of ‘cool path’ operations for the Renderer.
/// 
/// These are executed rarely — when a Scene initialises.
pub struct RkCool;

impl RkCool {

    ///
    pub fn store_signature_locations(
        r: &mut RendererWebGl,
        shader_index: usize,
    ) {
        // Tell WebGL to start using the shader under investigation.
        Self::use_shader(r, shader_index);

        // Get the Shader’s WebGlProgram and ShaderSignature.
        let shader = &mut r.shaders[shader_index];
        // let shader = r.get_shader_mut(shader_index);

        let program = shader.get_program();
        let signature = shader.get_signature();

        // Get each attribute’s location from WebGL.
        let mut index_to_location: Vec<u32> = vec![];
        for i in 0..signature.attribute_signatures.len() {
            let attribute_signature = &signature.attribute_signatures[i];
            let name_glsl = attribute_signature.name_glsl;

            // Ask WebGL what location it has given the attribute.
            let gl_location = r.gl.get_attrib_location(&program, name_glsl);
            if SLOWLY_GET_ERROR_COOL_PATH {
                let e = r.gl.get_error(); // @TODO does this actually pick up errors?
                if e != 0 { panic!("store...() get_attrib_location() {} {}", name_glsl, e) }
            }
            if gl_location < 0 { panic!("store...() attribute name '{}' not recognised", name_glsl) }
            index_to_location.push(gl_location as u32);
        }

        // Store each attribute’s location in the proper AttributeSignature.
        // @TOD combine these two loops into one... which means solve a Rust mutability puzzle
        for i in 0..index_to_location.len() {
            let location = index_to_location[i];
            shader.set_attribute_location(i, location);
            // crate::info(&format!("shaders[{}].signature.attribute_signatures[{}].location = {}", shader_index, i, location));
        }
    }

    ///
    pub fn _confirm_signature_locations(
        r: &RendererWebGl,
        shader_index: usize,
    ) {
        // Tell WebGL to start using the shader under investigation.
        Self::use_shader(r, shader_index);

        // Get the Shader’s WebGlProgram and ShaderSignature.
        let shader = &r.shaders[shader_index];
        let program = shader.get_program();
        let signature = shader.get_signature();

        // Confirm each AttributeSignature.
        for i in 0..signature.attribute_signatures.len() {
            let attribute_signature = &signature.attribute_signatures[i];
            let location = attribute_signature.location;
            let name_glsl = attribute_signature.name_glsl;

            // // This is an error in our Rust code — the `location` field should
            // // simply match the Vec<> index, so:
            // // vec![ { location: 0, ... }, { location: 1, ... }, ...etc ]
            // if location as u32 != attribute_signature.location {
            //     panic!("easy-to-fix location mismatch [{}] {} {} != {}",
            //         shader_index, name_glsl, location, attribute_signature.location);
            // }

            // Ask WebGL what location it has given the attribute.
            let gl_location = r.gl.get_attrib_location(&program, name_glsl);
            if SLOWLY_GET_ERROR_COOL_PATH {
                let e = r.gl.get_error(); // @TODO does this actually pick up errors?
                if e != 0 { panic!("confirm...() get_attrib_location() {} {}", name_glsl, e) }
            }
            if gl_location < 0 { panic!("confirm...() attribute name '{}' not recognised", name_glsl) }

            // During development, this is probably a mismatch between the GLSL
            // in the .vect, and the Rust in the AttributeSignature.
            // If it occurs in production, it may be because some graphics card
            // out there in the wild does not allocate locations predictably.
            if location != gl_location as u32 {
                panic!("confirm...() serious location mismatch shaders[{}] \"{}\" is at {}, not {}",
                    shader_index, name_glsl, gl_location, location);
            }

            crate::info(&format!("[{}] {} {}", shader_index, name_glsl, location));
        }
    }


    ///
    pub fn create_buffer_f32(
        r: &RendererWebGl,
        values: Vec<f32>,
    ) -> WebGlBuffer {

        // Convert `values` (a Rust vector) into a JavaScript `Float32Array`.
        // A WASM buffer is a different beast to the WebGL buffer created below.
        // __IMPORTANT NOTE:__ `/ 4` because items in `Vec<f32>` are four bytes.
        let wasm_pointer = values.as_ptr() as u32 / 4; // <-- don’t get 4 wrong!
        let wasm_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>() // `dyn_into` is provided by JsCast
            .unwrap()
            .buffer();
        let wasm_array = js_sys::Float32Array::new(&wasm_buffer)
            .subarray(
                wasm_pointer,
                wasm_pointer + values.len() as u32,
            );

        // Tell WebGL to initialise a WebGLBuffer object, which represents a
        // block of VRAM, physically close to the GPU. Get a pointer to it.
        let webgl_buffer = r.gl.create_buffer()
            .expect("failed to create attribute buffer");
        if SLOWLY_GET_ERROR_COOL_PATH {
            let e = r.gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("create_buffer() {}", e) }
        }

        // WebGL is a state machine, which has an internal ‘current buffer’
        // pointer. Tell it to switch to using the WebGL buffer we just created.
        r.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&webgl_buffer));
        if SLOWLY_GET_ERROR_COOL_PATH {
            let e = r.gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("bind_buffer() {}", e) }
        }
    
        // Write the `Float32Array` into the WebGL buffer we just created.
        r.gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &wasm_array,
            GL::STATIC_DRAW, // because `wasm_array` will remain the same for many renders
        );
        if SLOWLY_GET_ERROR_COOL_PATH {
            let e = r.gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("buffer_data_with_array_buffer_view() {}", e) }
        }

        // The pointer to the new WebGLBuffer object will be used every time an
        // attribute needs some of its values.
        webgl_buffer
    }


    /// Uses `GL::ELEMENT_ARRAY_BUFFER` instead of `GL::ARRAY_BUFFER`.
    pub fn create_index_buffer_u16(
        r: &RendererWebGl,
        values: Vec<u16>,
    ) -> WebGlBuffer {

        // Convert `values` (a Rust vector) into a JavaScript `Uint16Array`.
        // A WASM buffer is a different beast to the WebGL buffer created below.
        // __IMPORTANT NOTE:__ `/ 2` because items in `Vec<f16>` are two bytes.
        let wasm_pointer = values.as_ptr() as u32 / 2; // <-- don’t get 2 wrong!
        let wasm_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>() // `dyn_into` is provided by JsCast
            .unwrap()
            .buffer();
        let wasm_array = js_sys::Uint16Array::new(&wasm_buffer)
            .subarray(
                wasm_pointer,
                wasm_pointer + values.len() as u32,
            );

        // Tell WebGL to initialise a WebGLBuffer object, which represents a
        // block of VRAM, physically close to the GPU. Get a pointer to it.
        let webgl_buffer = r.gl.create_buffer()
            .expect("failed to create attribute buffer");
        if SLOWLY_GET_ERROR_COOL_PATH {
            let e = r.gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("create_buffer() {}", e) }
        }

        // WebGL is a state machine, which has an internal ‘current buffer’
        // pointer. Tell it to switch to using the WebGL buffer we just created.
        r.gl.bind_buffer(
            GL::ELEMENT_ARRAY_BUFFER,
            Some(&webgl_buffer),
        );
        if SLOWLY_GET_ERROR_COOL_PATH {
            let e = r.gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("bind_buffer() {}", e) }
        }

        // Write the `Uint16Array` into the WebGL buffer we just created.
        r.gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &wasm_array,
            GL::STATIC_DRAW, // because `wasm_array` will remain the same for many renders
        );
        if SLOWLY_GET_ERROR_COOL_PATH {
            let e = r.gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("buffer_data_with_array_buffer_view() {}", e) }
        }

        // The pointer to the new WebGLBuffer object will be used every time an
        // attribute needs some of its values.
        webgl_buffer
    }


    /// WebGL attributes are disabled by default, so enable the attribute here.
    /// Remember to call `use_shader()` before calling this function.
    /// @TODO make this fn run ops other than just enableVertexAttribArray()
    pub fn enable_attribute(
        r: &RendererWebGl,
        attribute_name: AttributeName,
    ) {
        // We’ve made sure that a Scene’s attribute locations are identical for
        // all the vertex shaders it uses. So `[0]` is as good as any index.
        let location = r.shaders[0].get_attribute_location(attribute_name);

        r.gl.enable_vertex_attrib_array(location);
        if SLOWLY_GET_ERROR_COOL_PATH {
            let e = r.gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("enable_vertex_attrib_array() {}", e) }
        }
    }


    /// @TODO describe
    pub fn use_attribute(
        r: &RendererWebGl,
        attribute_name: AttributeName,
        size: i32,
    ) {
        // We’ve made sure that a Scene’s attribute locations are identical for
        // all the vertex shaders it uses. So `[0]` is as good as any index.
        let location = r.shaders[0].get_attribute_location(attribute_name);

        // void gl.vertexAttribPointer(index, size, type, normalized, stride, offset)
        r.gl.vertex_attrib_pointer_with_i32(
            location, // index of the attribute to be modified
            size, // number of values per attribute — must be 1, 2, 3, or 4
            GL::FLOAT, // type — must all be BYTE, SHORT, UNSIGNED_BYTE|SHORT, FLOAT
            false, // normalized — should integers be clamped when cast to float?
            0, // stride — 0 means the attributes are ‘tightly packed’.
            0, // offset — ???? must be a multiple of the byte length of `type`
        );
        if SLOWLY_GET_ERROR_COOL_PATH {
            let e = r.gl.get_error(); // eg 1281 "Channel count `size` must be within [1,4]."
            if e != 0 { panic!("vertex_attrib_pointer_with_i32() {}", e) }
        }
    }


    /// @TODO describe
    pub fn set_repeat_gap(
        r: &RendererWebGl,
        attribute_name: AttributeName,
        divisor: u32,
    ) {
        // We’ve made sure that a Scene’s attribute locations are identical for
        // all the vertex shaders it uses. So `[0]` is as good as any index.
        let location = r.shaders[0].get_attribute_location(attribute_name);

        r.ext_instanced_arrays.vertexAttribDivisorANGLE(location, divisor);
        if SLOWLY_GET_ERROR_COOL_PATH {
            let e = r.gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("vertexAttribDivisorANGLE() {}", e) }
        }
    }


    /// Inits a new Shader, and returns its index in the `renderer.shaders` vector.
    /// Under the hood, `link_program()` tells WebGL to start using the new shader.
    pub fn add_shader(
        r: &mut RendererWebGl,
        shader_kind: ShaderProgramName,
    ) -> usize {
        let gl = &r.gl;

        &r.shaders.push(
            match shader_kind {
                ShaderProgramName::BlueBox =>
                    Box::new(ShaderBlueBox::new(&gl)),
                ShaderProgramName::FurlBasic =>
                    Box::new(ShaderFurlBasic::new(&gl)),
                ShaderProgramName::Guides =>
                    Box::new(ShaderGuides::new(&gl)),
                ShaderProgramName::RainbowCactus =>
                    Box::new(ShaderRainbowCactus::new(&gl)),
                ShaderProgramName::RedBox =>
                    Box::new(ShaderRedBox::new(&gl)),
            }
        );

        // Tell the caller the index of the new shader, for future reference.
        r.shaders.len() - 1
    }


    /// Tells WebGL to stop using its current shader program, and start using a
    /// different one.  
    /// NOTE: There is an RkWarm equivalent of this function.
    /// 
    /// @TODO maybe do nothing if we know that shader is already active?
    pub fn use_shader(
        r: &RendererWebGl,
        shader_index: usize,
    ) {
        r.shaders[shader_index].use_program(&r.gl);
        if SLOWLY_GET_ERROR_COOL_PATH {
            let e = r.gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("use_program() {}", e) }
        }
    }


    /// @TODO describe
    pub fn set_uniform_f32(
        r: &RendererWebGl,
        shader_index: usize,
        uniform_name: UniformName,
        value: f32,
    ) {
        let location = r.shaders[shader_index].get_uniform_location(uniform_name);

        r.gl.uniform1f(Some(&location), value);
        if SLOWLY_GET_ERROR_COOL_PATH {
            let e = r.gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("uniform1f() {}", e) }
        }
    }

}
