use super::{AttributeKind,AttributeName};

pub struct AttributeSignature {
    /// The WebGL data type, eg AttributeKind::Vec2 for two f32s in a vector.
    pub kind: AttributeKind,
    /// __ALL VERTEX SHADERS MUST PROVIDE THE SAME `gl.get_attrib_location()`__
    pub location: u32,
    /// How Rust code should refer to this attribute.
    pub name: AttributeName,
    /// How vertex shader code refers to this attribute.
    pub name_glsl: &'static str,
}
