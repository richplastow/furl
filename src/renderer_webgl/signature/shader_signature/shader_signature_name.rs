#[derive(Debug,PartialEq)]
pub enum ShaderSignatureName {
    /// An experiment in switching between shader programs during a render().
    BlueRedBox,
    /// Shaders which can render phyllotactic spirals.
    FurlBasic,
    /// Used to draw guide axes and grids during development.
    Guides,
    /// An experiment with WebGL instanced elements.
    RainbowCactus,
}
