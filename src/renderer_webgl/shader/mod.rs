//! Each Shader is a container for one linked WebGlProgram object.

mod fragment_shader;
pub use fragment_shader::{blue_frag,red_frag,passthru_frag,rainbow_frag};

mod vertex_shader;
pub use vertex_shader::{
    box_vert,
    furl_basic_vert,
    guides_vert,
    cactus_vert
};

mod shader_blue_box;
pub use shader_blue_box::ShaderBlueBox;

mod shader_furl_basic;
pub use shader_furl_basic::ShaderFurlBasic;

mod shader_guides;
pub use shader_guides::ShaderGuides;

mod shader_rainbow_cactus;
pub use shader_rainbow_cactus::ShaderRainbowCactus;

mod shader_red_box;
pub use shader_red_box::ShaderRedBox;

/// Utilities, just for Shaders.
mod sk_cool;
pub use sk_cool::SkCool;

/// The `ShaderProgram` trait, and `ShaderProgramName` enum.
mod shader_program;
pub use shader_program::{ShaderProgram,ShaderProgramName};
