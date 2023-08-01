use crate::renderer_webgl::RendererWebGl;
use super::super::{
    Scene,
    SceneBlueRedBoxes,
    SceneEmpty,
    SceneAloneFurl,
    SceneRainbowCactus,
};
use super::SceneContainerName;

pub struct SceneContainer {
    /// @TODO describe
    pub name: SceneContainerName,
    /// @TODO describe
    pub scene: Box<dyn Scene>,
}

impl SceneContainer {

    pub fn new(
        renderer: &mut RendererWebGl,
        scene_container_name: SceneContainerName,
    ) -> Self {
        match scene_container_name {
            SceneContainerName::BlueRedBoxes => Self {
                name: SceneContainerName::BlueRedBoxes,
                scene: Box::new(
                    SceneBlueRedBoxes::new(renderer)
                ),
            },
            SceneContainerName::Empty => Self {
                name: SceneContainerName::Empty,
                scene: Box::new(
                    SceneEmpty::new(renderer)
                ),
            },
            SceneContainerName::AloneFurl => Self {
                name: SceneContainerName::AloneFurl,
                scene: Box::new(
                    SceneAloneFurl::new(renderer)
                ),
            },
            SceneContainerName::RainbowCactus => Self {
                name: SceneContainerName::RainbowCactus,
                scene: Box::new(
                    SceneRainbowCactus::new(renderer)
                ),
            },
        }
    }
}
