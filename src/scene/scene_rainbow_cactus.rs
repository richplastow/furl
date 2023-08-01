//! Demonstrates WebGL instanced elements.

use web_sys::{WebGlBuffer,WebGlRenderingContext as GL};
use crate::app::Timer;
use crate::develop::Develop;
use crate::renderer_webgl::{
    AttributeName,
    RkCool,
    RkWarm,
    RendererWebGl,
    ShaderProgramName,
    ShaderSignatureName,
    UniformName
};
// use crate::shape::{ShapeAxes,ShapeGrids,ShapeCube};
use super::Scene;
use super::kit_scene::matrix::{perspective,rotate_x};

const INSTANCE_TALLY: usize = 500;

struct RefsBuffer {
    a_instance_log: WebGlBuffer,
    a_instance_log_rev: WebGlBuffer,
    a_instance_step: WebGlBuffer,
    colors: WebGlBuffer,
    vertices: WebGlBuffer,
}

struct IndicesShader {
    main: usize,
}

pub struct SceneRainbowCactus {
    indices_shader: IndicesShader,
    _refs_buffer: RefsBuffer,
    mov_matrix: [f32; 16],
    proj_matrix: [f32; 16],
    quaternion: [f32; 4],
    view_matrix: [f32; 16],
}

impl SceneRainbowCactus {
    pub fn new (
        r: &mut RendererWebGl, // the app’s singleton Renderer instance
    ) -> Self {


        // SHADERS

        // Init the shader, and store it in the `renderer.shaders` vector.
        let indices_shader = IndicesShader {
            main: RkCool::add_shader(r, ShaderProgramName::RainbowCactus),
        };

        // Make sure the Shader uses the ‘RainbowCactus’
        // ShaderSignature. This restriction means that we could switch between
        // shaders in render(), if we had more than one, without having to
        // rebind buffers to attributes — faster, less faff.
        let name = ShaderSignatureName::RainbowCactus;
        if r.shaders[indices_shader.main].get_signature_name() != &name {
            panic!("main does not use the RainbowCactus ShaderSignature");
        }

        // We can’t predict which locations WebGL has assigned to each of the
        // attributes. It seems that any given browser will alway assigns them
        // the same way... but there’s no guarantee. In any case, each browser
        // has its own strategy for assigning attribute locations.
        // 
        // Attributes, and their locations, are global. So we should be able to
        // use any of the shaders to find the attribute locations. Once we have
        // them, we can store them in Rust fields for the lifetime of the Scene,
        // they _shouldn’t_ change.
        RkCool::store_signature_locations(r, indices_shader.main);




        // PRECALCULATE

        // let mut _cube = ShapeCube::new(1.0);

        // The step and log attributes are used for per-instance transformations.
        let mut a_instance_step: Vec<f32> = vec![];
        for i in 0..INSTANCE_TALLY { a_instance_step.push((i + 1) as f32) }

        let a_instance_log: Vec<f32> = a_instance_step[..].into_iter().map(
            |&raw| f32::log(raw, 1.5)
        ).collect();

        let a_instance_log_rev: Vec<f32> = a_instance_step[..].into_iter().map(
            |&raw| f32::log(raw, 2.5)
        ).rev().collect();




        // BUFFERS

        let refs_buffer = RefsBuffer {
            a_instance_log: RkCool::create_buffer_f32(r, a_instance_log),
            a_instance_log_rev: RkCool::create_buffer_f32(r, a_instance_log_rev),
            a_instance_step: RkCool::create_buffer_f32(r, a_instance_step),

            // from https://www.tutorialspoint.com/webgl/webgl_cube_rotation.htm
            colors: RkCool::create_buffer_f32(r, vec![
                0.9,0.0,0.1, 0.8,0.1,0.2, 0.8,0.0,0.1, 0.9,0.2,0.0, // reds
                0.9,0.5,0.0, 0.8,0.4,0.0, 0.9,0.4,0.0, 0.9,0.4,0.1, // oranges
                0.8,0.9,0.3, 0.9,1.0,0.2, 0.7,0.6,0.1, 0.6,0.8,0.2, // yellows
                0.3,0.9,0.3, 0.0,1.0,0.2, 0.2,0.9,0.1, 0.1,0.8,0.2, // greens
                0.1,0.3,0.8, 0.0,0.2,0.9, 0.2,0.1,0.7, 0.0,0.3,0.6, // blues
                0.6,0.0,0.8, 0.3,0.1,0.5, 0.7,0.0,0.5, 0.8,0.1,0.6, // purples
            ]),

            vertices: RkCool::create_buffer_f32(r, vec![
                -1.0,-1.0,-1.0,  1.0,-1.0,-1.0,  1.0, 1.0,-1.0, -1.0, 1.0,-1.0,
                -1.0,-1.0, 1.0,  1.0,-1.0, 1.0,  1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
                -1.0,-1.0,-1.0, -1.0, 1.0,-1.0, -1.0, 1.0, 1.0, -1.0,-1.0, 1.0,
                 1.0,-1.0,-1.0,  1.0, 1.0,-1.0,  1.0, 1.0, 1.0,  1.0,-1.0, 1.0,
                -1.0,-1.0,-1.0, -1.0,-1.0, 1.0,  1.0,-1.0, 1.0,  1.0,-1.0,-1.0,
                -1.0, 1.0,-1.0, -1.0, 1.0, 1.0,  1.0, 1.0, 1.0,  1.0, 1.0,-1.0,     
            ]),
        };

        RkCool::create_index_buffer_u16(r, vec![
            0, 1, 2,  0, 2, 3,  4, 5, 6,  4, 6, 7,
            8, 9,10,  8,10,11, 12,13,14, 12,14,15,
           16,17,18, 16,18,19, 20,21,22, 20,22,23,
        ]);




        // ATTRIBUTES

        // From WebGL’s point of view, these are global variables which can be
        // read by all shader programs. Each shader program maintains its own
        // set of ‘locations’ (indices) for the attributes that it uses, 
        // __BUT WE HAVE ARRANGED IT SO THAT LOCATIONS MATCH IN ALL SHADERS__.  
        //
        // - Attributes can be read by vertex shaders, but not fragment shaders
        // - Attributes are disabled by default — so, enable_attribute()

        RkCool::enable_attribute(r, AttributeName::Color);
        RkCool::enable_attribute(r, AttributeName::InstanceLog);
        RkCool::enable_attribute(r, AttributeName::InstanceLogRev);
        RkCool::enable_attribute(r, AttributeName::InstanceStep);
        RkCool::enable_attribute(r, AttributeName::Position);

        // Point the Shader attributes to the correct buffers.
        r.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&refs_buffer.colors));
        RkCool::use_attribute(r, AttributeName::Color, 3);
        r.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&refs_buffer.vertices));
        RkCool::use_attribute(r, AttributeName::Position, 3);
        r.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&refs_buffer.a_instance_log));
        RkCool::use_attribute(r, AttributeName::InstanceLog, 1);
        r.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&refs_buffer.a_instance_log_rev));
        RkCool::use_attribute(r, AttributeName::InstanceLogRev, 1);
        r.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&refs_buffer.a_instance_step));
        RkCool::use_attribute(r, AttributeName::InstanceStep, 1);

        // @TODO describe
        RkCool::set_repeat_gap(r, AttributeName::InstanceLog, 1);
        RkCool::set_repeat_gap(r, AttributeName::InstanceLogRev, 1);
        RkCool::set_repeat_gap(r, AttributeName::InstanceStep, 1);


        Self {
            indices_shader,
            _refs_buffer: refs_buffer,
            mov_matrix: [1.,0.,0.,0., 0.,1.,0.,0., 0.,0.,1.,0., 0.,0.,0.,1.],
            view_matrix: [1.,0.,0.,0., 0.,1.,0.,0., 0.,0.,1.,0., 0.,0.,3.0,1.], // 3 m[14] to push the camera forward
            proj_matrix: perspective(40., r.aspect_ratio, 1., 100.0),
            quaternion: [
                0.0,0.0,1.0, // normalised axis
                -1.0 // initial angle in radians (updated using the timer)
            ],
        }
    }
}

impl Scene for SceneRainbowCactus {
    fn render(
        &mut self,
        _develop: &Develop, // the app’s `develop` instance
        r: &RendererWebGl, // the app’s singleton Renderer instance
        timer: &Timer, // the app’s `timer` instance
    ) {
        let main = self.indices_shader.main;

        // Rotate on the y-axis according to delta time.
        self.quaternion = [
            self.quaternion[0], self.quaternion[1], self.quaternion[2], // normalised axis
            self.quaternion[3] + timer.time_delta * 0.3 // angle in radians
        ];
        // self.mov_matrix = rotate_x(self.mov_matrix, timer.time_delta * 0.05);
        // self.mov_matrix = rotate_y(self.mov_matrix, timer.time_delta * 0.2);
        // self.mov_matrix = rotate_z(self.mov_matrix, timer.time_delta * 0.04);
        self.view_matrix = rotate_x(self.view_matrix, timer.time_delta * - 0.1);
        self.view_matrix[14] -= timer.time_delta * 0.2;

        RkWarm::set_uniform_mat4_f32(r, main, UniformName::ProjectionMatrix, self.proj_matrix);
        RkWarm::set_uniform_mat4_f32(r, main, UniformName::ViewMatrix, self.view_matrix);
        RkWarm::set_uniform_mat4_f32(r, main, UniformName::ModelMatrix, self.mov_matrix);
        RkWarm::set_uniform_vec4_f32(r, main, UniformName::Quaternion, self.quaternion);

        // Draw the rainbow cactus cubes.
        RkWarm::draw_instances(r, 
            GL::TRIANGLES, // mode — eg WebGLRenderingContext::POINTS or ::TRIANGLES
            0, // offset — in bytes, so must be a multiple of the size of the given type
            36, // count — number of vertices per instance
            INSTANCE_TALLY as u32, // primcount — number of instances
        );

    }
}
