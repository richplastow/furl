use super::{UniformKind,UniformName};

/// Note that in WebGL, different shader programs which use the same frag/vert
/// shader will have a different location for the same uniform. In other words,
/// there is no ‘global’ location for a uniform which all shader programs can
/// share. Therefore, UniformSignature does not have a `location` field (unlike
/// AttributeSignature). Instead, uniform locations are stored in a shader’s
/// `locations_uniform` field.
pub struct UniformSignature {
    /// The WebGL data type, eg UniformKind::Vec2 for two f32s in a vector.
    pub kind: UniformKind,
    /// How Rust code should refer to this uniform.
    pub name: UniformName,
    /// How vertex shader code refers to this uniform.
    pub name_glsl: &'static str,
}
