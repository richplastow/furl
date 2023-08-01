//! Utilities for working with matrices.

mod dot;
pub use dot::dot;

mod projection;
pub use projection::{ortho,orthographic,perspective};

mod rotate;
pub use rotate::{rotate_x,rotate_y};

mod translate;
pub use translate::translate;

pub const IDENTITY: [f32; 16] = [
    1.,0.,0.,0.,
    0.,1.,0.,0.,
    0.,0.,1.,0.,
    0.,0.,0.,1.,
];
