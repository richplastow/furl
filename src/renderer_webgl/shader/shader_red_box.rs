use web_sys::{WebGlProgram,WebGlRenderingContext as GL,WebGlUniformLocation};
use crate::error::{ERROR as E,error_to_string as e};
use super::super::{AttributeName,ShaderSignatureName,ShaderSignature,UniformName};
use super::{box_vert,red_frag,ShaderProgram,SkCool};


struct LocationsUniform {
    /// uniform float u_pointsize;
    u_pointsize: WebGlUniformLocation,
}

pub struct ShaderRedBox {
    locations_uniform: LocationsUniform,
    program: WebGlProgram,
    pub signature: ShaderSignature,
}

impl ShaderRedBox {

    pub fn new(
        gl: &GL,
    ) -> Self {
        let signature = ShaderSignature::new(ShaderSignatureName::BlueRedBox);
        let program = SkCool::link_program(&gl, &signature, box_vert(), red_frag())
            .expect(e(E::R11418));

        Self {
            locations_uniform: LocationsUniform {
                u_pointsize:
                    SkCool::locate_uniform(&gl, &program, &signature, UniformName::Pointsize),
            },
            program,
            signature,
        }
    }
}

impl ShaderProgram for ShaderRedBox {
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
        panic!("ShaderRedBox does not use that attribute");
    }

    fn set_attribute_location(
        &mut self,
        attribute_index: usize,
        location: u32,
    ) {
        if attribute_index >= self.signature.attribute_signatures.len() {
            panic!("ShaderRedBox has no attribute at index {}", attribute_index);
        }
        self.signature.attribute_signatures[attribute_index].location = location;
    }

    fn get_uniform_location(
        &self,
        uniform_name: UniformName,
    ) -> &WebGlUniformLocation {
        match uniform_name {
            UniformName::Pointsize =>
                &self.locations_uniform.u_pointsize,
            _ => panic!("ShaderRedBox does not use that uniform"),
        }
    }

    fn get_program(&self) -> &WebGlProgram { &self.program }
    fn get_signature_name(&self) -> &ShaderSignatureName { &self.signature.name }
    fn get_signature(&self) -> &ShaderSignature { &self.signature }
}
