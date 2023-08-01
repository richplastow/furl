//! Switching between complex Scenes may be slow — do this rarely!

mod kit_scene;

mod scene_container;
pub use scene_container::{SceneContainer,SceneContainerName};

mod scene_blue_red_boxes;
pub use scene_blue_red_boxes::SceneBlueRedBoxes;

mod scene_alone_furl;
pub use scene_alone_furl::SceneAloneFurl;

mod scene_empty;
pub use scene_empty::SceneEmpty;

mod scene_rainbow_cactus;
pub use scene_rainbow_cactus::SceneRainbowCactus;

mod scene;
pub use scene::Scene;
