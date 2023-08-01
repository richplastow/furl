//! #### The WebGL renderer, and a kit of functions to work with it.  
//! 
//! In future we might offer several renderers. For now, there’s only WebGL.

mod signature;
pub use signature::*;

mod shader;
pub use shader::ShaderProgramName;

mod renderer;
pub use renderer::RendererWebGl;

mod rk_cold;
pub use rk_cold::RkCold;

mod rk_cool;
pub use rk_cool::RkCool;

mod rk_tepid;
pub use rk_tepid::RkTepid;

mod rk_warm;
pub use rk_warm::RkWarm;

