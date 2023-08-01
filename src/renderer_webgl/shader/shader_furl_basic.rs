use web_sys::{WebGlProgram,WebGlRenderingContext as GL,WebGlUniformLocation};
use crate::error::{ERROR as E,error_to_string as e};
use super::super::{AttributeName,ShaderSignatureName,ShaderSignature,UniformName};
use super::{furl_basic_vert,passthru_frag,ShaderProgram,SkCool};


struct LocationsUniform {
    /// A mat4 uniform for controlling instance Angle.
    iu_angle: WebGlUniformLocation,
    /// A mat4 uniform for controlling instance Bulge.
    iu_bulge: WebGlUniformLocation,
    /// A mat4 uniform for controlling instance lean.
    iu_lean: WebGlUniformLocation,
    /// A mat4 uniform for controlling instance Rise.
    iu_rise: WebGlUniformLocation,
    /// A mat4 uniform for controlling instance Scale.
    iu_scale: WebGlUniformLocation,
    /// A mat4 uniform for controlling instance Tilt.
    iu_tilt: WebGlUniformLocation,
    /// uniform vec3 Placement;
    u_placement: WebGlUniformLocation,
    /// uniform mat4 ProjectionMatrix;
    u_projection_matrix: WebGlUniformLocation,
    /// uniform vec4 Slidermix;
    u_slidermix: WebGlUniformLocation,
    /// uniform vec4 Timermix;
    u_timermix: WebGlUniformLocation,
    /// uniform mat4 ViewMatrix;
    u_view_matrix: WebGlUniformLocation,
    /// uniform vec4 QuaternionX;
    u_quaternion_x: WebGlUniformLocation,
    /// uniform vec4 QuaternionY;
    u_quaternion_y: WebGlUniformLocation,
}

pub struct ShaderFurlBasic {
    program: WebGlProgram,
    locations_uniform: LocationsUniform,
    pub signature: ShaderSignature,
}

impl ShaderFurlBasic {
    pub fn new(
        gl: &GL,
    ) -> Self {
        let signature = ShaderSignature::new(ShaderSignatureName::FurlBasic);
        let program = SkCool::link_program(&gl, &signature, furl_basic_vert(), passthru_frag())
            .expect(e(E::R11418));

        Self {
            locations_uniform: LocationsUniform {
                iu_angle:
                    SkCool::locate_uniform(&gl, &program, &signature, UniformName::Angle),
                iu_bulge:
                    SkCool::locate_uniform(&gl, &program, &signature, UniformName::Bulge),
                iu_lean:
                    SkCool::locate_uniform(&gl, &program, &signature, UniformName::Lean),
                iu_rise:
                    SkCool::locate_uniform(&gl, &program, &signature, UniformName::Rise),
                iu_scale:
                    SkCool::locate_uniform(&gl, &program, &signature, UniformName::Scale),
                iu_tilt:
                    SkCool::locate_uniform(&gl, &program, &signature, UniformName::Tilt),
                u_placement:
                    SkCool::locate_uniform(&gl, &program, &signature, UniformName::Placement),
                u_projection_matrix:
                    SkCool::locate_uniform(&gl, &program, &signature, UniformName::ProjectionMatrix),
                u_slidermix:
                    SkCool::locate_uniform(&gl, &program, &signature, UniformName::Slidermix),
                u_timermix:
                    SkCool::locate_uniform(&gl, &program, &signature, UniformName::Timermix),
                u_view_matrix:
                    SkCool::locate_uniform(&gl, &program, &signature, UniformName::ViewMatrix),
                u_quaternion_x:
                    SkCool::locate_uniform(&gl, &program, &signature, UniformName::QuaternionX),
                u_quaternion_y:
                    SkCool::locate_uniform(&gl, &program, &signature, UniformName::QuaternionY),
            },
            program,
            signature,
        }
    }
}

impl ShaderProgram for ShaderFurlBasic {
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
        panic!("ShaderFurlBasic does not use that attribute");
    }

    fn set_attribute_location(
        &mut self,
        attribute_index: usize,
        location: u32,
    ) {
        if attribute_index >= self.signature.attribute_signatures.len() {
            panic!("ShaderFurlBasic has no attribute at index {}", attribute_index);
        }
        self.signature.attribute_signatures[attribute_index].location = location;
    }

    fn get_uniform_location(
        &self,
        uniform_name: UniformName,
    ) -> &WebGlUniformLocation {
        match uniform_name {
            UniformName::Angle =>
                &self.locations_uniform.iu_angle,
            UniformName::Bulge =>
                &self.locations_uniform.iu_bulge,
            UniformName::Lean =>
                &self.locations_uniform.iu_lean,
            UniformName::Rise =>
                &self.locations_uniform.iu_rise,
            UniformName::Scale =>
                &self.locations_uniform.iu_scale,
            UniformName::Tilt =>
                &self.locations_uniform.iu_tilt,
            UniformName::Placement =>
                &self.locations_uniform.u_placement,
            UniformName::ProjectionMatrix =>
                &self.locations_uniform.u_projection_matrix,
            UniformName::Slidermix =>
                &self.locations_uniform.u_slidermix,
            UniformName::Timermix =>
                &self.locations_uniform.u_timermix,
            UniformName::ViewMatrix =>
                &self.locations_uniform.u_view_matrix,
            UniformName::QuaternionX =>
                &self.locations_uniform.u_quaternion_x,
            UniformName::QuaternionY =>
                &self.locations_uniform.u_quaternion_y,
            _ => panic!("ShaderFurlBasic does not use UniformName::{:?}", uniform_name),
        }
    }

    fn get_program(&self) -> &WebGlProgram { &self.program }
    fn get_signature_name(&self) -> &ShaderSignatureName { &self.signature.name }
    fn get_signature(&self) -> &ShaderSignature { &self.signature }
}
