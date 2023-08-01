use super::super::{AttributeSignature,UniformName,UniformSignature};
use super::ShaderSignatureName;

pub struct ShaderSignature {
    /// @TODO describe
    pub attribute_signatures: Vec<AttributeSignature>,
    /// Lets Rust code refer to this signature, eg ShaderSignatureName::BlueRedBox.
    pub name: ShaderSignatureName,
    /// @TODO describe
    pub uniform_signatures: Vec<UniformSignature>,
}

impl ShaderSignature {
    pub fn new(name: ShaderSignatureName) -> Self {
        match name {
            ShaderSignatureName::BlueRedBox => Self::blue_red_box(),
            ShaderSignatureName::FurlBasic => Self::furl_basic(),
            ShaderSignatureName::Guides => Self::guides(),
            ShaderSignatureName::RainbowCactus => Self::rainbow_cactus(),
        }
    }

    pub fn get_uniform_name_glsl(
        &self,
        name: UniformName,
    ) -> &'static str {
        for uniform_signature in self.uniform_signatures.iter() {
            if uniform_signature.name == name {
                return uniform_signature.name_glsl;
            }
        }
        panic!("get_uniform_name_glsl() cannot find \"{:?}\"", name);
    }
}
