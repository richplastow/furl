use web_sys::WebGlRenderingContext as GL;
use crate::develop::GuidesPreset;
use crate::renderer_webgl::{RkWarm,RendererWebGl};
use super::point_3::Point3;

const M1_POINTS: usize = 6;
const M10_POINTS: usize = 6;

pub struct SizeOffsets {
    m1: u16,
    m10: u16,
}

pub struct StartIndices {
    pub colors: SizeOffsets,
    pub cnnx: SizeOffsets,
    pub vertices: SizeOffsets,
}

pub struct ShapeAxesCnnxMap {
    pub m1: (i32,i32), // (offset,count)
    pub m10: (i32,i32),
}

/// An x, y and z axis, centered on the origin.
pub struct ShapeAxes {
    cnnx_map: ShapeAxesCnnxMap,
    m1_points: [Point3; M1_POINTS],
    m10_points: [Point3; M10_POINTS],
    pub start_indices: StartIndices,
}

impl ShapeAxes {

    pub fn new(
    ) -> Self {
        Self {
            cnnx_map: ShapeAxesCnnxMap {
                m1: (0, 0),
                m10: (0, 0),
            },
            /*      4
                    |
               0 -- * -- 1   * is where 2 and 3 overlap
                    | 
                    5
            */

            // 1 metre.
            m1_points: [
                Point3::new(-1.,  0.,  0.), // 0 left
                Point3::new( 1.,  0.,  0.), // 1 right
                Point3::new( 0., -1.,  0.), // 2 top
                Point3::new( 0.,  1.,  0.), // 3 bottom
                Point3::new( 0.,  0., -1.), // 4 back
                Point3::new( 0.,  0.,  1.), // 5 front
            ],

            // 10 metres.
            m10_points: [
                Point3::new(-10.,   0.,   0.), // 0 left
                Point3::new( 10.,   0.,   0.), // 1 right
                Point3::new(  0., -10.,   0.), // 2 top
                Point3::new(  0.,  10.,   0.), // 3 bottom
                Point3::new(  0.,   0., -10.), // 4 back
                Point3::new(  0.,   0.,  10.), // 5 front
            ],

            start_indices: StartIndices { // values in here will be updated
                colors: SizeOffsets { m1: 0, m10: 0 },
                cnnx: SizeOffsets { m1: 0, m10: 0 },
                vertices: SizeOffsets { m1: 0, m10: 0 },
            },
        }
    }

    pub fn get_cnnx(
        &mut self,
        start_index: usize,
    ) -> Vec<u16> {
        let m1 = self.start_indices.colors.m1;
        let m10 = self.start_indices.colors.m10;
        assert!(m1 == self.start_indices.vertices.m1, "m1 offset mismatch");
        assert!(m10 == self.start_indices.vertices.m10, "m10 offset mismatch");

        let cnnx: Vec<u16> = vec![
            /*      4
                    |
               0 -- * -- 1   * is where 2 and 3 overlap
                    | 
                    5
            */

            // 1 metre.
            m1+0, m1+1, // left - right
            m1+2, m1+3, // top - bottom
            m1+4, m1+5, // back - front

            // 10 metres.
            m10+0, m10+1, // left - right
            m10+2, m10+3, // top - bottom
            m10+4, m10+5, // back - front
        ];

        self.start_indices.cnnx.m1 = ((start_index + 0) * 2) as u16;
        self.start_indices.cnnx.m10 = ((start_index + M1_POINTS) * 2) as u16;

        self.cnnx_map.m1 = (self.start_indices.cnnx.m1 as i32,M1_POINTS as i32);
        self.cnnx_map.m10 = (self.start_indices.cnnx.m10 as i32,M10_POINTS as i32);

        cnnx
    }

    pub fn get_colors(
        &mut self,
        start_vertex: usize,
    ) -> Vec<f32> {
        let colors: Vec<f32> = vec![

            // 1 metre.
            0.0, 1.0, 1.0, // 0 cyan     left
            1.0, 0.0, 0.0, // 1 red      right
            1.0, 0.0, 1.0, // 2 magenta  top
            0.0, 1.0, 0.0, // 3 green    bottom
            1.0, 1.0, 0.0, // 4 yellow   back
            0.0, 0.0, 1.0, // 5 blue     front

            // 10 metres.
            0.0, 1.0, 1.0, // 0 cyan     left
            1.0, 0.0, 0.0, // 1 red      right
            1.0, 0.0, 1.0, // 2 magenta  top
            0.0, 1.0, 0.0, // 3 green    bottom
            1.0, 1.0, 0.0, // 4 yellow   back
            0.0, 0.0, 1.0, // 5 blue     front
        ];

        self.start_indices.colors.m1 = (start_vertex + 0) as u16;
        self.start_indices.colors.m10 = (start_vertex + &self.m1_points.len()) as u16;

        colors
    }

    pub fn get_vertices(
        &mut self,
        start_vertex: usize,
    ) -> Vec<f32> {
        let p0 = &self.m1_points;
        let p1 = &self.m10_points;
        let vertices: Vec<f32> = vec![

            // 1 metre.
            p0[0].x, p0[0].y, p0[0].z, // 0 left
            p0[1].x, p0[1].y, p0[1].z, // 1 right
            p0[2].x, p0[2].y, p0[2].z, // 2 top
            p0[3].x, p0[3].y, p0[3].z, // 3 bottom
            p0[4].x, p0[4].y, p0[4].z, // 4 back
            p0[5].x, p0[5].y, p0[5].z, // 5 front

            // 10 metres.
            p1[0].x, p1[0].y, p1[0].z, // 0 left
            p1[1].x, p1[1].y, p1[1].z, // 1 right
            p1[2].x, p1[2].y, p1[2].z, // 2 top
            p1[3].x, p1[3].y, p1[3].z, // 3 bottom
            p1[4].x, p1[4].y, p1[4].z, // 4 back
            p1[5].x, p1[5].y, p1[5].z, // 5 front
        ];

        self.start_indices.vertices.m1 = (start_vertex + 0) as u16;
        self.start_indices.vertices.m10 = (start_vertex + &self.m1_points.len()) as u16;

        vertices
    }

    pub fn render(
        &self,
        r: &RendererWebGl, // the app’s singleton Renderer instance
        guides_preset: &GuidesPreset,
    ) {
        let cnnx = match guides_preset {
            GuidesPreset::AxesOnly1m | GuidesPreset::All1m => self.cnnx_map.m1,
            _ => self.cnnx_map.m10, // must be ::AxesOnly10m or ::All10m
        };
        RkWarm::draw_elements(r,
            GL::LINES, // mode — GL::LINES connects pairs of vertices
            cnnx.0, // offset — in bytes, so must be a multiple of the size of the given type
            cnnx.1, // count — number of vertices per instance
            GL::UNSIGNED_SHORT, // type_ eg WebGLRenderingContext::UNSIGNED_SHORT
        );
    }

}
