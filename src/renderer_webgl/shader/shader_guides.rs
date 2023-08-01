use web_sys::{WebGlProgram,WebGlRenderingContext as GL,WebGlUniformLocation};
use crate::error::{ERROR as E,error_to_string as e};
use super::super::{AttributeName,ShaderSignatureName,ShaderSignature,UniformName};
use super::{guides_vert,passthru_frag,ShaderProgram,SkCool};


struct LocationsUniform {
    /// uniform mat4 ProjectionMatrix;
    u_projection_matrix: WebGlUniformLocation,
    /// uniform mat4 ViewMatrix;
    u_view_matrix: WebGlUniformLocation,
}

pub struct ShaderGuides {
    program: WebGlProgram,
    locations_uniform: LocationsUniform,
    pub signature: ShaderSignature,
}

impl ShaderGuides {
    pub fn new(
        gl: &GL,
    ) -> Self {
        let signature = ShaderSignature::new(ShaderSignatureName::Guides);
        let program = SkCool::link_program(&gl, &signature, guides_vert(), passthru_frag())
            .expect(e(E::R11418));

        Self {
            locations_uniform: LocationsUniform {
                u_projection_matrix:
                    SkCool::locate_uniform(&gl, &program, &signature, UniformName::ProjectionMatrix),
                u_view_matrix:
                    SkCool::locate_uniform(&gl, &program, &signature, UniformName::ViewMatrix),
            },
            program,
            signature,
        }
    }
}

impl ShaderProgram for ShaderGuides {
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
        panic!("ShaderGuides does not use AttributeName::{:?}", attribute_name);
    }

    fn set_attribute_location(
        &mut self,
        attribute_index: usize,
        location: u32,
    ) {
        if attribute_index >= self.signature.attribute_signatures.len() {
            panic!("ShaderGuides has no attribute at index {}", attribute_index);
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
            _ => panic!("ShaderGuides does not use that uniform"),
        }
    }

    fn get_program(&self) -> &WebGlProgram { &self.program }
    fn get_signature_name(&self) -> &ShaderSignatureName { &self.signature.name }
    fn get_signature(&self) -> &ShaderSignature { &self.signature }
}
