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
    pub fn blue_red_box() -> Self {
        Self {
            attribute_signatures: vec![
                AttributeSignature {
                    kind: AttributeKind::F32,
                    location: 0,
                    name: AttributeName::PositionX,
                    name_glsl: "a_position_x",
                },
                AttributeSignature {
                    kind: AttributeKind::F32,
                    location: 1,
                    name: AttributeName::PositionY,
                    name_glsl: "a_position_y",
                },
                AttributeSignature {
                    kind: AttributeKind::F32,
                    location: 2,
                    name: AttributeName::InstanceStep,
                    name_glsl: "a_instance_step",
                },
            ],
            name: ShaderSignatureName::BlueRedBox,
            uniform_signatures: vec![
                UniformSignature {
                    kind: UniformKind::F32,
                    name: UniformName::Pointsize,
                    name_glsl: "u_pointsize",
                },
            ],
        }
    }
}
