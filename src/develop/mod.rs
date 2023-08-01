mod console;
pub use console::{error,info,log};

mod camera_preset;
pub use camera_preset::CameraPreset;

mod guides_preset;
pub use guides_preset::GuidesPreset;

mod lod_preset;
pub use lod_preset::LodPreset;

mod wireframe_preset;
pub use wireframe_preset::WireframePreset;

pub struct Develop {
    pub camera_preset: CameraPreset,
    pub guides_preset: GuidesPreset,
    pub lod_preset: LodPreset,
    pub wireframe_preset: WireframePreset,
}
