use super::point_3::Point3;

const LOD0_POINTS: usize = 5;
const LOD0_CNNX: i32 = 6;
const LOD1_POINTS: usize = 9;
const LOD1_CNNX: i32 = 14;

pub struct LodOffsets {
    lod0: u16,
    lod1: u16,
}

pub struct StartIndices {
    pub colors: LodOffsets,
    pub cnnx: LodOffsets,
    pub vertices: LodOffsets,
}

pub struct ShapeNubbinCnnxMap {
    pub lod0: (i32,i32), // (offset,count)
    pub lod1: (i32,i32),
}

/// A rounded tetrahedron which could look like a seed pod if used in a Furl.
pub struct ShapeNubbin {
    pub cnnx_map: ShapeNubbinCnnxMap,
    lod0_points: [Point3; LOD0_POINTS],
    lod1_points: [Point3; LOD1_POINTS],
    pub start_indices: StartIndices,
}

impl ShapeNubbin {

    pub fn new(
        upper_height: f32,
        lower_height: f32,
        nose_length: f32,
        tail_length: f32,
        width: f32,
    ) -> Self {
        let w = width / 2.0;
        let mid_z = (nose_length - tail_length) * 0.5;
        Self {
            cnnx_map: ShapeNubbinCnnxMap {
                lod0: (0,0),
                lod1: (0,0),
            },
            /*  2 ------- 3
                 \'.   .'/
                  \ '*' /  * is where 0 and 4 overlap
                   \ | /
                    \|/
                     1
            */
            lod0_points: [
                Point3::new( 0. ,  upper_height,  0.          ), // 0 upper
                Point3::new( 0. ,  0.,            nose_length ), // 1 nose
                Point3::new(-w  ,  0.,           -tail_length ), // 2 tail_left
                Point3::new( w  ,  0.,           -tail_length ), // 3 tail_right
                Point3::new( 0. , -lower_height,  0.          ), // 4 lower
            ],
            /*
                    .4.
                  .' | '.
                3'   |   '5
                | '. | .' |
                |  .'*'.  |  * is where 0 and 8 overlap
                6.' / \ '.7
                 \ /   \ /
                  1 --- 2
            */
            lod1_points: [
                Point3::new( 0.   ,  upper_height,  0.              ), // 0 upper
                Point3::new(-w*0.1,  0.,            nose_length     ), // 1 nose_left
                Point3::new( w*0.1,  0.,            nose_length     ), // 2 nose_right
                Point3::new(-w    ,  0.,           -tail_length*0.8 ), // 3 tail_left
                Point3::new( 0.   ,  0.,           -tail_length     ), // 4 tail_mid
                Point3::new( w    ,  0.,           -tail_length*0.8 ), // 5 tail_right
                Point3::new(-w*0.8,  0.,           mid_z            ), // 6 mid_left
                Point3::new( w*0.8,  0.,           mid_z            ), // 7 mid_right
                Point3::new( 0.   , -lower_height,  0.              ), // 8 lower
            ],

            start_indices: StartIndices { // values in here will be updated
                colors: LodOffsets { lod0: 0, lod1: 0 },
                cnnx: LodOffsets { lod0: 0, lod1: 0 },
                vertices: LodOffsets { lod0: 0, lod1: 0 },
            },
        }
    }

    // @TODO should all be anticlockwise?
    pub fn get_cnnx(
        &mut self,
        start_index: usize,
    ) -> Vec<u16> {
        let l0 = self.start_indices.colors.lod0;
        let l1 = self.start_indices.colors.lod1;
        assert!(l0 == self.start_indices.vertices.lod0, "lod0 offset mismatch");
        assert!(l1 == self.start_indices.vertices.lod1, "lod1 offset mismatch");

        let cnnx: Vec<u16> = vec![

            // lod0
            /*  2 ------- 3
                 \'.   .'/
                  \ '*' /  * is where 0 and 4 overlap
                   \ | /
                    \|/
                     1
            */
            l0+0, l0+1, l0+2, // upper - nose - tail_left
            l0+0, l0+1, l0+3, // upper - nose - tail_right
            l0+0, l0+2, l0+3, // upper - tail_left - tail_right
            l0+4, l0+1, l0+2, // lower - nose - tail_left
            l0+4, l0+1, l0+3, // lower - nose - tail_right
            l0+4, l0+2, l0+3, // lower - tail_left - tail_right

            // lod1
            /*
                    .4.
                  .' | '.
                3'   |   '5
                | '. | .' |
                |  .'*'.  |  * is where 0 and 8 overlap
                6.' / \ '.7
                 \ /   \ /
                  1 --- 2
            */
            l1+0, l1+1, l1+2, // upper - nose_left - nose_right
            l1+0, l1+1, l1+6, // upper - nose_left - mid_left
            l1+0, l1+6, l1+3, // upper - mid_left - tail_left
            l1+0, l1+3, l1+4, // upper - tail_left - tail_mid
            l1+0, l1+4, l1+5, // upper - tail_mid - tail_right
            l1+0, l1+5, l1+7, // upper - tail_right - mid_right
            l1+0, l1+7, l1+2, // upper - mid_right - nose_right
            l1+8, l1+1, l1+2, // lower - nose_left - nose_right
            l1+8, l1+1, l1+6, // lower - nose_left - mid_left
            l1+8, l1+6, l1+3, // lower - mid_left - tail_left
            l1+8, l1+3, l1+4, // lower - tail_left - tail_mid
            l1+8, l1+4, l1+5, // lower - tail_mid - tail_right
            l1+8, l1+5, l1+7, // lower - tail_right - mid_right
            l1+8, l1+7, l1+2, // lower - mid_right - nose_right
        ];

        self.start_indices.cnnx.lod0 = ((start_index + 0) * 2) as u16;
        self.start_indices.cnnx.lod1 = ((start_index + 18) * 2) as u16;

        self.cnnx_map.lod0 = (self.start_indices.cnnx.lod0 as i32,LOD0_CNNX * 3);
        self.cnnx_map.lod1 = (self.start_indices.cnnx.lod1 as i32,LOD1_CNNX * 3);

        cnnx
    }

    pub fn get_colors(
        &mut self,
        start_vertex: usize,
    ) -> Vec<f32> {
        let colors: Vec<f32> = vec![

            // lod0
            0.3, 0.9, 0.3, // 0 upper       green (positive y direction)
            0.1, 0.3, 0.8, // 1 nose        blue (positive z direction)
            0.2, 0.9, 0.9, // 2 tail_left   cyan (-ve x, -ve z direction)
            0.9, 0.5, 0.0, // 3 tail_right  orange (+ve x, -ve z direction)
            0.6, 0.0, 0.7, // 4 lower       magenta (negative y direction)

            // lod1
            0.3, 0.9, 0.3, // 0 upper       green (positive y direction)
            0.1, 0.3, 0.8, // 1 nose_left   blue (positive z direction)
            0.0, 0.2, 0.9, // 2 nose_right  blue (positive z direction)
            0.2, 0.9, 0.9, // 3 tail_left   cyan (-ve x, -ve z direction)
            0.9, 0.9, 0.0, // 4 tail_mid    yellow (-ve z direction)
            0.9, 0.5, 0.0, // 5 tail_right  orange (+ve x, -ve z direction)
            0.5, 0.5, 0.5, // 6 mid_left    grey @TODO
            0.5, 0.5, 0.5, // 7 mid_right    grey @TODO
            0.6, 0.0, 0.7, // 8 lower       magenta (negative y direction)
        ];

        self.start_indices.colors.lod0 = (start_vertex + 0) as u16;
        self.start_indices.colors.lod1 = (start_vertex + LOD0_POINTS) as u16;

        colors
    }

    pub fn get_vertices(
        &mut self,
        start_vertex: usize,
    ) -> Vec<f32> {
        let p0 = &self.lod0_points;
        let p1 = &self.lod1_points;
        let vertices: Vec<f32> = vec![

            // lod0
            p0[0].x, p0[0].y, p0[0].z, // upper
            p0[1].x, p0[1].y, p0[1].z, // nose
            p0[2].x, p0[2].y, p0[2].z, // tail_left
            p0[3].x, p0[3].y, p0[3].z, // tail_right
            p0[4].x, p0[4].y, p0[4].z, // lower

            // lod1
            p1[0].x, p1[0].y, p1[0].z, // 0 upper
            p1[1].x, p1[1].y, p1[1].z, // 1 nose_left
            p1[2].x, p1[2].y, p1[2].z, // 2 nose_right
            p1[3].x, p1[3].y, p1[3].z, // 3 tail_left
            p1[4].x, p1[4].y, p1[4].z, // 4 tail_mid
            p1[5].x, p1[5].y, p1[5].z, // 5 tail_right
            p1[6].x, p1[6].y, p1[6].z, // 6 mid_left
            p1[7].x, p1[7].y, p1[7].z, // 7 mid_right
            p1[8].x, p1[8].y, p1[8].z, // 8 lower
        ];

        self.start_indices.vertices.lod0 = (start_vertex + 0) as u16;
        self.start_indices.vertices.lod1 = (start_vertex + LOD0_POINTS) as u16;

        vertices
    }

}
