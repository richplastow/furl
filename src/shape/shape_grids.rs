use web_sys::WebGlRenderingContext as GL;
use crate::develop::GuidesPreset;
use crate::renderer_webgl::{RkWarm,RendererWebGl};
use super::point_3::Point3;

// The number of points in each size.
// So the 1 metre grids contain 1200 points, and so do the 10 metre grids.
const NUM_POINTS: usize = 10 * 10 * 4 * 3;

pub struct ShapeGridsVerticesMap {
    pub m1: (i32,i32), // (offset,count)
    pub m10: (i32,i32),
}

/// Three 2D grids in the x, y and z planes, centered on the origin.
pub struct ShapeGrids {
    m1_points: [Point3; NUM_POINTS],
    m10_points: [Point3; NUM_POINTS],
    vertices_map: ShapeGridsVerticesMap,
}

impl ShapeGrids {

    pub fn new(
    ) -> Self {
        Self {

            // 1 metre.
            m1_points: generate_points(1.),

            // 10 metres.
            m10_points: generate_points(10.),

            vertices_map: ShapeGridsVerticesMap {
                m1: (0, 0),
                m10: (0, 0),
            },
        }
    }

    pub fn get_colors(
        &mut self,
        _start_vertex: usize,
    ) -> Vec<f32> {
        let p0 = &self.m1_points;

        let mut colors: Vec<f32> = vec![];
        colors.append(&mut generate_colors(&p0));
        colors.append(&mut generate_colors(&p0)); // NOTE: using m1_points data to colour the m10_points
        assert_eq!(colors.len(), NUM_POINTS * 3 * 2); // three f32s per Point3, and two sizes of grids

        colors
    }

    pub fn get_vertices(
        &mut self,
        start_vertex: usize,
    ) -> Vec<f32> {
        let p0 = &self.m1_points;
        let p1 = &self.m10_points;

        let mut vertices: Vec<f32> = vec![];
        vertices.append(&mut generate_vertices(&p0));
        vertices.append(&mut generate_vertices(&p1));
        assert_eq!(vertices.len(), NUM_POINTS * 3 * 2); // three f32s per Point3, and two sizes of grids

        self.vertices_map.m1 = (start_vertex as i32,NUM_POINTS as i32);
        self.vertices_map.m10 = ((start_vertex + NUM_POINTS) as i32,NUM_POINTS as i32);

        vertices
    }

    pub fn render(
        &self,
        r: &RendererWebGl, // the app’s singleton Renderer instance
        guides_preset: &GuidesPreset,
    ) {
        let vertices = match guides_preset {
            GuidesPreset::GridsOnly1m | GuidesPreset::All1m => self.vertices_map.m1,
            _ => self.vertices_map.m10, // must be ::GridsOnly10m or ::All10m
        };
        RkWarm::draw(r, GL::POINTS, vertices.0, vertices.1);
    }

}


fn generate_points(size: f32) -> [Point3; NUM_POINTS] {
    // Create a correctly sized array, filled with Point3s placed at the origin.
    let mut points:[Point3; NUM_POINTS] = [Point3::new(0.,0.,0.); NUM_POINTS];
    let mut i = 0;

    // Generate a 2D grid in the x = 0 plane.
    for y in -10..=10 {
        if y == 0 { continue }
        for z in -10..=10 {
            if z == 0 { continue }
            // points[i].x = 0.; // should already be zero
            points[i].y = y as f32 * size / 10.;
            points[i].z = z as f32 * size / 10.;
            i += 1;
        }
    }

    // Generate a 2D grid in the y = 0 plane (the ground).
    for x in -10..=10 {
        if x == 0 { continue }
        for z in -10..=10 {
            if z == 0 { continue }
            points[i].x = x as f32 * size / 10.;
            // points[i].y = 0.; // should already be zero
            points[i].z = z as f32 * size / 10.;
            i += 1;
        }
    }

    // Generate a 2D grid in the y = 0 plane.
    for x in -10..=10 {
        if x == 0 { continue }
        for y in -10..=10 {
            if y == 0 { continue }
            points[i].x = x as f32 * size / 10.;
            points[i].y = y as f32 * size / 10.;
            // points[i].z = 0.; // should already be zero
            i += 1;
        }
    }

    points
}


fn generate_colors(p: &[Point3; NUM_POINTS]) -> Vec<f32> {
    // Create an empty vector.
    let mut colors:Vec<f32> = vec![];

    for i in 0..NUM_POINTS {
        let mut r = 0.;
        let mut g = 0.;
        let mut b = 0.;
        if p[i].x > 0. { r += p[i].x } else { g -= p[i].x; b -= p[i].x }
        if p[i].y > 0. { g += p[i].y } else { r -= p[i].y; b -= p[i].y }
        if p[i].z > 0. { b += p[i].z } else { r -= p[i].z; g -= p[i].z }
        colors.push(r);
        colors.push(g);
        colors.push(b);
    }

    colors
}


fn generate_vertices(p: &[Point3; NUM_POINTS]) -> Vec<f32> {
    // Create an empty vector.
    let mut vertices:Vec<f32> = vec![];

    for i in 0..NUM_POINTS {
        vertices.push(p[i].x);
        vertices.push(p[i].y);
        vertices.push(p[i].z);
    }

    vertices
}
