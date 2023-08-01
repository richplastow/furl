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
    pub fn guides() -> Self {
        Self {
            attribute_signatures: vec![
                AttributeSignature {
                    kind: AttributeKind::Vec3,
                    location: 3,
                    name: AttributeName::Position,
                    name_glsl: "position",
                },
                AttributeSignature {
                    kind: AttributeKind::Vec3,
                    location: 7,
                    name: AttributeName::Color,
                    name_glsl: "color",
                },
            ],
            name: ShaderSignatureName::Guides,
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
            ],
        }
    }
}
