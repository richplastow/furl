use web_sys::{WebGlProgram,WebGlRenderingContext as GL,WebGlUniformLocation};
use super::super::super::{AttributeName,ShaderSignatureName,ShaderSignature,UniformName};

/// Contains one linked WebGlProgram object.
pub trait ShaderProgram {

    /// Wraps the GLSL program in Some(), ready to pass to gl.use_program().
    fn option_program(&self) -> Option<&WebGlProgram>;

    // Tell WebGL to switch from whatever program it was using, to this one.
    fn use_program(
        &self,
        gl: &GL,
    ) {
        gl.use_program(self.option_program());
    }

    fn get_attribute_location(
        &self,
        _attribute_name: AttributeName,
    ) -> u32 {
        panic!("get_attribute_location() not implemented");
    }

    fn get_program(
        &self,
    ) -> &WebGlProgram {
        panic!("get_program() not implemented");
    }

    fn get_signature_name(
        &self,
    ) -> &ShaderSignatureName {
        panic!("get_signature_name() not implemented");
    }

    fn get_signature(
        &self,
    ) -> &ShaderSignature {
        panic!("get_signature() not implemented");
    }

    fn get_signature_mut(
        &mut self,
    ) -> &mut ShaderSignature {
        panic!("get_signature_mut() not implemented");
    }

    fn set_attribute_location(
        &mut self,
        _attribute_index: usize,
        _location: u32,
    ) {
        panic!("set_attribute_location() not implemented");
    }

    fn get_uniform_location(
        &self,
        _uniform_name: UniformName,
    ) -> &WebGlUniformLocation {
        panic!("get_uniform_location() not implemented");
    }
}
