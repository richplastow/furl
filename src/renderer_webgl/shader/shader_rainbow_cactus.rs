use web_sys::{WebGlProgram,WebGlRenderingContext as GL,WebGlUniformLocation};
use crate::error::{ERROR as E,error_to_string as e};
use super::super::{AttributeName,ShaderSignatureName,ShaderSignature,UniformName};
use super::{cactus_vert,rainbow_frag,ShaderProgram,SkCool};


struct LocationsUniform {
    /// uniform mat4 ProjectionMatrix;
    u_projection_matrix: WebGlUniformLocation,
    /// uniform mat4 ViewMatrix;
    u_view_matrix: WebGlUniformLocation,
    /// uniform mat4 ModelMatrix;
    u_model_matrix: WebGlUniformLocation,
    /// uniform vec4 u_quaternion;
    u_quaternion: WebGlUniformLocation,
}

pub struct ShaderRainbowCactus {
    pub signature: ShaderSignature,
    locations_uniform: LocationsUniform,
    program: WebGlProgram,
}

impl ShaderRainbowCactus {

    pub fn new(
        gl: &GL,
    ) -> Self {
        let signature = ShaderSignature::new(ShaderSignatureName::RainbowCactus);
        let program = SkCool::link_program(&gl, &signature, cactus_vert(), rainbow_frag())
            .expect(e(E::R11418));

        Self {
            locations_uniform: LocationsUniform {
                u_projection_matrix:
                    SkCool::locate_uniform(&gl, &program, &signature, UniformName::ProjectionMatrix),
                u_view_matrix:
                    SkCool::locate_uniform(&gl, &program, &signature, UniformName::ViewMatrix),
                u_model_matrix:
                    SkCool::locate_uniform(&gl, &program, &signature, UniformName::ModelMatrix),
                u_quaternion:
                    SkCool::locate_uniform(&gl, &program, &signature, UniformName::Quaternion),
            },
            program,
            signature,
        }
    }
}

impl ShaderProgram for ShaderRainbowCactus {
    fn option_program(&self) -> Option<&WebGlProgram> {
        Some(&self.program)
    }

    fn get_attribute_location(
        &self,
        attribute_name: AttributeName,
    ) -> u32 {
        for attribute_signature in self.signature.attribute_signatures.iter() {
            if attribute_signature.name == attribute_name {
                return attribute_signature.location;
            }
        }
        panic!("ShaderRainbowCactus does not use \"{:?}\"", attribute_name);
    }

    fn set_attribute_location(
        &mut self,
        attribute_index: usize,
        location: u32,
    ) {
        if attribute_index >= self.signature.attribute_signatures.len() {
            panic!("ShaderRainbowCactus has no attribute at index {}", attribute_index);
        }
        self.signature.attribute_signatures[attribute_index].location = location;
    }

    fn get_uniform_location(
        &self,
        uniform_name: UniformName,
    ) -> &WebGlUniformLocation {
        match uniform_name {
            UniformName::ProjectionMatrix =>
                &self.locations_uniform.u_projection_matrix,
            UniformName::ViewMatrix =>
                &self.locations_uniform.u_view_matrix,
            UniformName::ModelMatrix =>
                &self.locations_uniform.u_model_matrix,
            UniformName::Quaternion =>
                &self.locations_uniform.u_quaternion,
            _ => panic!("ShaderRainbowCactus does not use that uniform"),
        }
    }

    fn get_program(&self) -> &WebGlProgram { &self.program }
    fn get_signature_name(&self) -> &ShaderSignatureName { &self.signature.name }
    fn get_signature(&self) -> &ShaderSignature { &self.signature }

}
