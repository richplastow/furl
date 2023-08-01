use crate::renderer_webgl::RendererWebGl;
use super::Scene;
pub struct SceneEmpty {}
impl SceneEmpty {
    pub fn new (_renderer: &mut RendererWebGl) -> Self {
        Self {}
    }
}
impl Scene for SceneEmpty {
}
