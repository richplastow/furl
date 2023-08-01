#[derive(Clone,Copy,Debug,PartialEq)]
pub enum UniformName {
    // Instance Uniforms.
    Bulge,
    Lean,
    Rise,
    Scale,
    Angle,
    Tilt,

    // Vector Uniforms.
    Placement,
    Pointsize,
    ProjectionMatrix,
    Slidermix,
    Timermix,
    ViewMatrix,
    ModelMatrix,
    Quaternion,
    QuaternionX,
    QuaternionY,
}
