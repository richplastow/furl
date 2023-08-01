//! A kit of ‘warm path’ operations for the Renderer.

use web_sys::WebGlRenderingContext as GL;
use crate::error::SLOWLY_GET_ERROR_WARM_PATH;
use crate::shape::Point3;
use super::{RendererWebGl,UniformName};

pub struct RkWarm;


/// #### A kit of ‘warm path’ operations for the Renderer.
/// 
/// These are executed on every render, so ideally 60 times per second.  
/// It’s called ‘warm’ not ‘hot’, because there are potentially hotter paths:
///   1. Code in a vertex shader
///   2. The interior of a loop in a vertex shader
///   3. Code in a fragment shader
///   4. The interior of a loop in a fragment shader
impl RkWarm {


    /// Deletes everything drawn on the canvas. Typically called at the start of
    /// each render phase.
    pub fn clear(
        r: &RendererWebGl,
    ) {
        r.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        if SLOWLY_GET_ERROR_WARM_PATH {
            let e = r.gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("clear() {}", e) }
        }
    }


    /// Tells WebGL to stop using its current shader program, and start using a
    /// different one.  
    /// NOTE: There is an RkCool equivalent of this function.
    /// 
    /// @TODO maybe do nothing if we know that shader is already active?
    pub fn use_shader(
        r: &RendererWebGl,
        index: usize,
    ) {
        r.shaders[index].use_program(&r.gl);
        if SLOWLY_GET_ERROR_WARM_PATH {
            let e = r.gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("use_program() {}", e) }
        }
    }


    /// @TODO describe
    pub fn draw(
        r: &RendererWebGl,
        mode: u32, // eg WebGLRenderingContext::POINTS or ::TRIANGLES
        first: i32,
        count: i32,
    ) {
        r.gl.draw_arrays(mode, first, count);
        if SLOWLY_GET_ERROR_WARM_PATH {
            let e = r.gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("draw_arrays() {}", e) }
        }
    }


    /// @TODO describe
    pub fn draw_elements(
        r: &RendererWebGl,
        mode: u32, // eg WebGLRenderingContext::POINTS or ::TRIANGLES
        offset: i32, // in bytes, so must be a multiple of the size of the given type
        count: i32,
        type_: u32, // eg WebGLRenderingContext::UNSIGNED_SHORT
    ) {
        r.gl.draw_elements_with_i32(mode, count, type_, offset);
        if SLOWLY_GET_ERROR_WARM_PATH {
            let e = r.gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("draw_elements_with_i32() {}", e) }
        }
    }


    /// @TODO describe
    pub fn repeat(
        r: &RendererWebGl,
        mode: u32, // eg WebGLRenderingContext::POINTS or ::TRIANGLES
        first: u32,
        count: u32,
        primcount: u32,
    ) {
        r.ext_instanced_arrays.drawArraysInstancedANGLE(
            mode,
            first, // starting index in the array of vector points
            count, // number of vertices per instance
            primcount, // number of instances
        );
        if SLOWLY_GET_ERROR_WARM_PATH {
            let e = r.gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("drawArraysInstancedANGLE() {}", e) }
        }
    }


    /// @TODO describe
    pub fn draw_instances(
        r: &RendererWebGl,
        mode: u32, // eg WebGLRenderingContext::POINTS or ::TRIANGLES
        offset: i32, // in bytes, so must be a multiple of the size of the given type
        count: i32, // the number of elements to be rendered
        primcount: u32, // the number of instances of the set of elements to execute
    ) {
        r.ext_instanced_arrays.drawElementsInstancedANGLE(
            mode,
            count,
            GL::UNSIGNED_SHORT, // type
            offset,
            primcount
        );
        if SLOWLY_GET_ERROR_WARM_PATH {
            let e = r.gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("drawElementsInstancedANGLE() {}", e) }
        }
    }


    /// @TODO there should be an RkTepid equivalent of this function.
    pub fn set_uniform_mat4_f32(
        r: &RendererWebGl,
        shader_index: usize,
        uniform_name: UniformName,
        value: [f32; 16],
    ) {
        let gl = &r.gl;

        let uniform_location =
            r.shaders[shader_index].get_uniform_location(uniform_name);

        match uniform_name {
            // iu_*
            UniformName::Angle => gl.uniform_matrix4fv_with_f32_array(
                Some(&uniform_location),
                false, // transpose
                &value,
            ),
            UniformName::Bulge => gl.uniform_matrix4fv_with_f32_array(
                Some(&uniform_location),
                false, // transpose
                &value,
            ),
            UniformName::Lean => gl.uniform_matrix4fv_with_f32_array(
                Some(&uniform_location),
                false, // transpose
                &value,
            ),
            UniformName::Rise => gl.uniform_matrix4fv_with_f32_array(
                Some(&uniform_location),
                false, // transpose
                &value,
            ),
            UniformName::Scale => gl.uniform_matrix4fv_with_f32_array(
                Some(&uniform_location),
                false, // transpose
                &value,
            ),
            UniformName::Tilt => gl.uniform_matrix4fv_with_f32_array(
                Some(&uniform_location),
                false, // transpose
                &value,
            ),

            // Matrices.
            UniformName::ProjectionMatrix => gl.uniform_matrix4fv_with_f32_array(
                Some(&uniform_location),
                false, // transpose
                &value,
            ),
            UniformName::ViewMatrix => gl.uniform_matrix4fv_with_f32_array(
                Some(&uniform_location),
                false, // transpose
                &value,
            ),
            UniformName::ModelMatrix => gl.uniform_matrix4fv_with_f32_array(
                Some(&uniform_location),
                false, // transpose
                &value,
            ),

            // Error.
            _ => panic!("uniform name '{:?}' not recognised", uniform_name),
        }
        if SLOWLY_GET_ERROR_WARM_PATH {
            let e = gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("uniform_matrix4fv_with_f32_array() {}", e) }
        }
    }


    /// @TODO there should be an RkTepid equivalent of this function.
    pub fn set_uniform_vec4_f32(
        r: &RendererWebGl,
        shader_index: usize,
        uniform_name: UniformName,
        value: [f32; 4],
    ) {
        let gl = &r.gl;

        let uniform_location =
            r.shaders[shader_index].get_uniform_location(uniform_name);

        match uniform_name {
            UniformName::Slidermix => gl.uniform4fv_with_f32_array(
                Some(&uniform_location),
                &value,
            ),
            UniformName::Timermix => gl.uniform4fv_with_f32_array(
                Some(&uniform_location),
                &value,
            ),
            UniformName::Quaternion => gl.uniform4fv_with_f32_array(
                Some(&uniform_location),
                &value,
            ),
            UniformName::QuaternionX => gl.uniform4fv_with_f32_array(
                Some(&uniform_location),
                &value,
            ),
            UniformName::QuaternionY => gl.uniform4fv_with_f32_array(
                Some(&uniform_location),
                &value,
            ),
            _ => panic!("uniform name '{:?}' not recognised", uniform_name),
        }
        if SLOWLY_GET_ERROR_WARM_PATH {
            let e = gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("uniform_matrix4fv_with_f32_array() {}", e) }
        }
    }


    /// @TODO there should be an RkTepid equivalent of this function.
    pub fn set_uniform_point3_f32(
        r: &RendererWebGl,
        shader_index: usize,
        uniform_name: UniformName,
        value: Point3,
    ) {
        let gl = &r.gl;

        let uniform_location =
            r.shaders[shader_index].get_uniform_location(uniform_name);

        match uniform_name {
            UniformName::Placement => gl.uniform3fv_with_f32_array(
                Some(&uniform_location),
                &[value.x, value.y, value.z],
            ),
            _ => panic!("uniform name '{:?}' not recognised", uniform_name),
        }
        if SLOWLY_GET_ERROR_WARM_PATH {
            let e = gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("uniform_matrix4fv_with_f32_array() {}", e) }
        }
    }

}
