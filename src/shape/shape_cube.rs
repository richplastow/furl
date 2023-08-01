use super::point_3::Point3;

const LOD0_POINTS: usize = 8;
const LOD0_CNNX: i32 = 12;

pub struct LodOffsets {
    lod0: u16,
}

pub struct StartIndices {
    pub colors: LodOffsets,
    pub connections: LodOffsets,
    pub vertices: LodOffsets,
}

/// A simple cube, where each face is two triangles.
pub struct ShapeCube {
    lod0_points: [Point3; LOD0_POINTS],
    pub start_indices: StartIndices,
}

impl ShapeCube {

    pub fn new(
        side_length: f32,
    ) -> Self {
        let l = side_length;
        Self {
        /*      .2 ------.6
              .' |     .' |
            3'------ 7'   |
            |    |   |    |
            |   .0 - | --.4
            | .'     | .'
            1'------ 5'        */
            lod0_points: [
                Point3::new(-l, -l, -l), // 0
                Point3::new(-l, -l,  l), // 1
                Point3::new(-l,  l, -l), // 2
                Point3::new(-l,  l,  l), // 3
                Point3::new( l, -l, -l), // 4
                Point3::new( l, -l,  l), // 5
                Point3::new( l,  l, -l), // 6
                Point3::new( l,  l,  l), // 7
            ],
            start_indices: StartIndices { // values in here will be updated
                colors: LodOffsets { lod0: 0 },
                connections: LodOffsets { lod0: 0 },
                vertices: LodOffsets { lod0: 0 },
            },
        }
    }

    pub fn get_lod0_start(&self) -> i32 { self.start_indices.connections.lod0 as i32 }
    pub fn get_lod0_length(&self) -> i32 { LOD0_CNNX * 3 }

    // @TODO should all be anticlockwise?
    pub fn get_cnnx(
        &mut self,
        start_index: usize,
    ) -> Vec<u16> {
        let l0 = self.start_indices.colors.lod0;
        assert!(l0 == self.start_indices.vertices.lod0, "lod0 offset mismatch");

        let connections: Vec<u16> = vec![

            // lod0
            /*      .2 ------.6
                .' |     .' |
                3'------ 7'   |
                |    |   |    |
                |   .0 - | --.4
                | .'     | .'
                1'------ 5'        */
            l0+0, l0+1, l0+2, // 0 - 1 - 2    -ve x face (left)
            l0+2, l0+1, l0+3, // 2 - 1 - 3    -ve x face (left)
            l0+5, l0+4, l0+7, // 5 - 4 - 7    +ve x face (right)
            l0+7, l0+4, l0+6, // 7 - 4 - 6    +ve x face (right)
            l0+2, l0+3, l0+6, // 2 - 3 - 6    -ve y face (top)
            l0+6, l0+3, l0+7, // 6 - 3 - 7    -ve y face (top)
            l0+0, l0+1, l0+4, // 0 - 1 - 4    +ve y face (bottom)
            l0+4, l0+1, l0+5, // 4 - 1 - 5    +ve y face (bottom)
            l0+4, l0+0, l0+6, // 4 - 0 - 6    -ve z face (back)
            l0+6, l0+0, l0+2, // 6 - 0 - 2    -ve z face (back)
            l0+1, l0+5, l0+3, // 1 - 5 - 3    +ve z face (front)
            l0+3, l0+5, l0+7, // 3 - 5 - 7    +ve z face (front)
        ];

        self.start_indices.connections.lod0 = ((start_index + 0) * 2) as u16;

        connections
    }

    pub fn get_colors(
        &mut self,
        start_vertex: usize,
    ) -> Vec<f32> {
        let colors: Vec<f32> = vec![

            // lod0
            0.9,0.0,0.1, // 0 red
            0.9,0.5,0.0, // 1 orange
            0.8,0.9,0.3, // 2 yellow
            0.3,0.9,0.3, // 3 green
            0.1,0.8,0.9, // 4 cyan
            0.1,0.3,0.8, // 5 blue
            0.6,0.0,0.8, // 6 purple
            0.8,0.1,0.7, // 7 magenta
        ];

        self.start_indices.colors.lod0 = (start_vertex + 0) as u16;

        colors
    }

    pub fn get_vertices(
        &mut self,
        start_vertex: usize,
    ) -> Vec<f32> {
        let p0 = &self.lod0_points;
        let vertices: Vec<f32> = vec![

            // lod0
            p0[0].x, p0[0].y, p0[0].z,
            p0[1].x, p0[1].y, p0[1].z,
            p0[2].x, p0[2].y, p0[2].z,
            p0[3].x, p0[3].y, p0[3].z,
            p0[4].x, p0[4].y, p0[4].z,
            p0[5].x, p0[5].y, p0[5].z,
            p0[6].x, p0[6].y, p0[6].z,
            p0[7].x, p0[7].y, p0[7].z,
        ];

        self.start_indices.vertices.lod0 = (start_vertex + 0) as u16;

        vertices
    }

}
