const SHAPE_TRIANGLE_3_MAX: usize = 1;

use crate::renderer_webgl::RendererWebGl;
use crate::shape::{Point3,ShapeTriangle3};

/// Root node of the scene graph.
pub struct NodeScene {
    _shape_triangle_3s: [ShapeTriangle3; SHAPE_TRIANGLE_3_MAX],
}

impl NodeScene {

    pub fn new(
    ) -> Self {
        Self {
            _shape_triangle_3s: [
                ShapeTriangle3::new(
                    Point3::new( 10., -10., -10.),
                    Point3::new(-10., -10., 0.),
                    Point3::new( 0.,  10., 5.),
                ),
            ],
        }
    }


    // Phase 8: Render the Shapes.
    pub fn render(
        &self,
        _renderer: &RendererWebGl,
    ) {
    }

}
