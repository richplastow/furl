use super::{
    AttributeKind,
    AttributeName,
    AttributeSignature,
    ShaderSignature,
    ShaderSignatureName,
    UniformKind,
    UniformName,
    UniformSignature,
};


impl ShaderSignature {
    pub fn rainbow_cactus() -> Self {
        Self {
            attribute_signatures: vec![
                AttributeSignature {
                    kind: AttributeKind::Vec3,
                    location: 0,
                    name: AttributeName::Position,
                    name_glsl: "position",
                },
                AttributeSignature {
                    kind: AttributeKind::F32,
                    location: 1,
                    name: AttributeName::InstanceLog,
                    name_glsl: "a_instance_log",
                },
                AttributeSignature {
                    kind: AttributeKind::F32,
                    location: 2,
                    name: AttributeName::InstanceStep,
                    name_glsl: "a_instance_step",
                },
                AttributeSignature {
                    kind: AttributeKind::Vec3,
                    location: 3,
                    name: AttributeName::Color,
                    name_glsl: "color",
                },
                AttributeSignature {
                    kind: AttributeKind::F32,
                    location: 4,
                    name: AttributeName::InstanceLogRev,
                    name_glsl: "a_instance_log_rev",
                },
            ],
            name: ShaderSignatureName::RainbowCactus,
            uniform_signatures: vec![
                UniformSignature {
                    kind: UniformKind::Mat4,
                    name: UniformName::ProjectionMatrix,
                    name_glsl: "u_projection_matrix",
                },
                UniformSignature {
                    kind: UniformKind::Mat4,
                    name: UniformName::ViewMatrix,
                    name_glsl: "u_view_matrix",
                },
                UniformSignature {
                    kind: UniformKind::Mat4,
                    name: UniformName::ModelMatrix,
                    name_glsl: "u_model_matrix",
                },
                UniformSignature {
                    kind: UniformKind::Vec4,
                    name: UniformName::Quaternion,
                    name_glsl: "u_quaternion",
                },
            ],
        }
    }
}
