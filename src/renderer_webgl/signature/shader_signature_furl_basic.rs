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
    pub fn furl_basic() -> Self {
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
                AttributeSignature {
                    kind: AttributeKind::Vec4,
                    location: 8,
                    name: AttributeName::Curves,
                    name_glsl: "ia_curves",
                },
            ],
            name: ShaderSignatureName::FurlBasic,
            uniform_signatures: vec![
                UniformSignature {
                    kind: UniformKind::Mat4,
                    name: UniformName::Angle,
                    name_glsl: "iu_angle",
                },
                UniformSignature {
                    kind: UniformKind::Mat4,
                    name: UniformName::Bulge,
                    name_glsl: "iu_bulge",
                },
                UniformSignature {
                    kind: UniformKind::Mat4,
                    name: UniformName::Lean,
                    name_glsl: "iu_lean",
                },
                UniformSignature {
                    kind: UniformKind::Mat4,
                    name: UniformName::Rise,
                    name_glsl: "iu_rise",
                },
                UniformSignature {
                    kind: UniformKind::Mat4,
                    name: UniformName::Scale,
                    name_glsl: "iu_scale",
                },
                UniformSignature {
                    kind: UniformKind::Mat4,
                    name: UniformName::Tilt,
                    name_glsl: "iu_tilt",
                },
                UniformSignature {
                    kind: UniformKind::Vec3,
                    name: UniformName::Placement,
                    name_glsl: "u_placement",
                },
                UniformSignature {
                    kind: UniformKind::Vec4,
                    name: UniformName::Slidermix,
                    name_glsl: "u_slidermix",
                },
                UniformSignature {
                    kind: UniformKind::Vec4,
                    name: UniformName::Timermix,
                    name_glsl: "u_timermix",
                },
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
                    kind: UniformKind::Vec4,
                    name: UniformName::QuaternionX,
                    name_glsl: "u_quaternion_x",
                },
                UniformSignature {
                    kind: UniformKind::Vec4,
                    name: UniformName::QuaternionY,
                    name_glsl: "u_quaternion_y",
                },
            ],
        }
    }
}
