use crate::renderer_webgl::{RkWarm,RendererWebGl,UniformName};
use super::point_3::Point3;
use super::ShapeNubbin;

/// A group of ShapeNubbins arranged in a spiral.
pub struct ShapeFurl {
    nubbin: ShapeNubbin,
    placement: Point3,
}

impl ShapeFurl {

    pub fn new(
        nubbin_upper_height: f32,
        nubbin_lower_height: f32,
        nubbin_nose_length: f32,
        nubbin_tail_length: f32,
        nubbin_width: f32,
        placement: Point3,
    ) -> Self {
        Self {
            nubbin: ShapeNubbin::new(
                nubbin_upper_height,
                nubbin_lower_height,
                nubbin_nose_length,
                nubbin_tail_length,
                nubbin_width,
            ),
            placement,
        }
    }

    pub fn get_cnnx(
        &mut self,
        start_index: usize,
    ) -> Vec<u16> {
        self.nubbin.get_cnnx(start_index)
    }

    pub fn get_colors(
        &mut self,
        start_vertex: usize,
    ) -> Vec<f32> {
        self.nubbin.get_colors(start_vertex)
    }

    pub fn get_vertices(
        &mut self,
        start_vertex: usize,
    ) -> Vec<f32> {
        self.nubbin.get_vertices(start_vertex)
    }

    pub fn render(
        &self,
        r: &RendererWebGl, // the app’s singleton Renderer instance
        shader_index: usize,
        lod: u8,
        wireframe_mode: u32, // eg WebGLRenderingContext::LINE_STRIP, ::POINTS or ::TRIANGLES
        primcount: u32,
    ) {
        let cnnx_map = match lod {
            0 => self.nubbin.cnnx_map.lod0,
            _ => self.nubbin.cnnx_map.lod1,
        };
        RkWarm::set_uniform_point3_f32(r, shader_index, UniformName::Placement, self.placement);
        RkWarm::draw_instances(r,
            wireframe_mode, // mode — eg WebGLRenderingContext::LINE_STRIP, ::POINTS or ::TRIANGLES
            cnnx_map.0, // offset — in bytes, so must be a multiple of the size of the given type
            cnnx_map.1, // count — number of vertices per instance
            primcount, // primcount — number of instances
        );
    }

}
