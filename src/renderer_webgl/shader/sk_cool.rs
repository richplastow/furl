//! A kit of ‘cool path’ operations for Shaders.

use web_sys::{
    WebGlProgram,
    WebGlRenderingContext as GL,
    WebGlShader,
    WebGlUniformLocation
};
use super::super::{ShaderSignature,UniformName};
use crate::error::{ERROR as E,error_to_string as e,SLOWLY_GET_ERROR_COOL_PATH};


/// #### A kit of ‘cool path’ operations for Shaders.
/// 
/// These are executed rarely — when a Scene initialises.
pub struct SkCool;

impl SkCool {

    /// @TODO describe
    pub fn link_program(
        gl: &GL,
        shader_signature: &ShaderSignature,
        vert_source: &str,
        frag_source: &str,
    ) -> Result<WebGlProgram, String> {
        let vert_shader = compile_shader(
            &gl,
            GL::VERTEX_SHADER,
            vert_source,
        ).expect(e(E::R11530));
        if SLOWLY_GET_ERROR_COOL_PATH { let e = gl.get_error(); // @TODO see it work
            if e != 0 { panic!("{}: compile_shader(&gl, VERTEX_SHADER, vert_source)", e) } }

        let frag_shader = compile_shader(
            &gl,
            GL::FRAGMENT_SHADER,
            frag_source,
        ).expect(e(E::R11572));
        if SLOWLY_GET_ERROR_COOL_PATH { let e = gl.get_error(); // @TODO see it work
            if e != 0 { panic!("{}: compile_shader(&gl, FRAGMENT_SHADER, frag_source)", e) } }

        let program = gl.create_program()
            .expect(e(E::R11331)); // @TODO check that R11331 can get thrown
        if SLOWLY_GET_ERROR_COOL_PATH { let e = gl.get_error(); // @TODO see it work
            if e != 0 { panic!("{}: create_program()", e) } }

        // crate::info(&format!("shader_signature.name: {:?}", &shader_signature.name));
        for sig in &shader_signature.attribute_signatures {
            gl.bind_attrib_location(
                &program,
                sig.location,
                sig.name_glsl,
            );
            // Panic if sig.location is invalid. WebGL’s warning is:
            //   bindAttribLocation: `location` must be less than MAX_VERTEX_ATTRIBS.
            // @TODO pick up other errors, eg a nonexistant attribute name
            if SLOWLY_GET_ERROR_COOL_PATH { let e = gl.get_error();
                if e != 0 { panic!("{}: bind_attrib_location(&program, {}, \"{}\")",
                    e, sig.location, sig.name_glsl) } }
            // crate::info(&format!("name_glsl: {:?}", attribute_signature.name_glsl));
            // crate::info(&format!("location: {:?}", attribute_signature.location));
        }

        gl.attach_shader(&program, &vert_shader);
        if SLOWLY_GET_ERROR_COOL_PATH { let e = gl.get_error(); // @TODO see it work
            if e != 0 { panic!("{}: attach_shader(&program, &vert_shader)", e) } }

        gl.attach_shader(&program, &frag_shader);
        if SLOWLY_GET_ERROR_COOL_PATH { let e = gl.get_error(); // @TODO see it work
            if e != 0 { panic!("{}: attach_shader(&program, &frag_shader)", e) } }

        gl.link_program(&program);
        if SLOWLY_GET_ERROR_COOL_PATH { let e = gl.get_error(); // @TODO see it work
            if e != 0 { panic!("{}: link_program(&program)", e) } }

        // Probably (?) free up resources.
        gl.detach_shader(&program, &vert_shader);
        gl.detach_shader(&program, &frag_shader);
        gl.delete_shader(Some(&vert_shader));
        gl.delete_shader(Some(&frag_shader));

        if gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap_or(false) {
            Ok(program)
        } else {
            Err(gl.get_program_info_log(&program).expect(e(E::R11982))) // @TODO check that R11982 can get thrown
        }
    }


    /// Returns the ‘location’ of a WebGL attribute with the given name.  
    /// Panics if no attribute with that name exists in the given program.
    pub fn _locate_attribute(
        gl: &GL,
        program: &WebGlProgram,
        name: &'static str,
    ) -> u32 {
        let location = gl.get_attrib_location(&program, name);
        if SLOWLY_GET_ERROR_COOL_PATH {
            let e = gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("get_attrib_location() {} {}", name, e) }
        }
        if location < 0 { panic!("attribute name '{}' not recognised", name) }
        let msg = format!("{} {}", name, location);
        crate::info(&msg);
        location as u32
    }


    /// Returns the ‘location’ of a WebGL uniform with the given name.  
    /// Panics if no uniform with that name exists in the given program.
    pub fn locate_uniform(
        gl: &GL,
        program: &WebGlProgram,
        signature: &ShaderSignature,
        name: UniformName,
    ) -> WebGlUniformLocation {
        let name_glsl = signature.get_uniform_name_glsl(name);
        // gl.bind_attrib_location(&program, 123, name_glsl);
        let location = gl.get_uniform_location(&program, name_glsl)
            .expect(&format!("{:?}' get_uniform_location(..., \"{}\")", e(E::R11006), name_glsl));
        if SLOWLY_GET_ERROR_COOL_PATH {
            let e = gl.get_error(); // @TODO does this actually pick up errors?
            if e != 0 { panic!("{}: get_uniform_location(..., \"{}\")", e, name_glsl) }
        }
        location
    }
}




// HELPERS

fn compile_shader(
    gl: &GL,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl.create_shader(shader_type)
        .expect(e(E::R11872));
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false) {
        Ok(shader)
    } else {
        Err(gl.get_shader_info_log(&shader).expect(e(E::R11982))) // @TODO check that R11982 can get thrown
    }
}
