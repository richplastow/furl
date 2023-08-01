//! Demonstrates how to switch between two Shaders on each `render()` call.

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
use super::Scene;

struct RefsBuffer {
    instance_steps: WebGlBuffer,
    a_position_x: WebGlBuffer,
    a_position_y: WebGlBuffer,
}

struct IndicesShader {
    blue_box: usize,
    red_box: usize,
}

pub struct SceneBlueRedBoxes {
    indices_shader: IndicesShader,
    _refs_buffer: RefsBuffer,
}

impl SceneBlueRedBoxes {
    pub fn new (
        r: &mut RendererWebGl, // the app’s singleton Renderer instance
    ) -> Self {


        // SHADERS

        // Init the Shaders, and store them in the `renderer.shaders` vector.
        let indices_shader = IndicesShader {
            blue_box: RkCool::add_shader(r, ShaderProgramName::BlueBox),
            red_box: RkCool::add_shader(r, ShaderProgramName::RedBox),
        };

        // Make sure all Shaders use the ‘BlueRedBox’ ShaderSignature.
        // This restriction means that we can switch between shaders in render()
        // without having to rebind buffers to attributes — faster, less faff.
        let name = ShaderSignatureName::BlueRedBox;
        if r.shaders[indices_shader.blue_box].get_signature_name() != &name {
            panic!("BlueBox does not use the BlueRedBox ShaderSignature");
        }
        if r.shaders[indices_shader.red_box].get_signature_name() != &name {
            panic!("RedBox does not use the BlueRedBox ShaderSignature");
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
        RkCool::store_signature_locations(r, indices_shader.blue_box);
        RkCool::store_signature_locations(r, indices_shader.red_box);

        // Make sure that WebGPU has assigned the locations we expected to each
        // attribute, in all Shaders. It’s possible that some browser or 
        // graphics card out there in the wild may not play nice!
        //
        // @TODO implement confirm_signature_locations(), so that it checks that
        // all Shaders have identical attribute locations.
        // RkCool::confirm_signature_locations(r, indices_shader.blue_box);
        // RkCool::confirm_signature_locations(r, indices_shader.red_box);




        // BUFFERS

        let refs_buffer = RefsBuffer {

            // Set the offset attribute, used for the x _and_ the y positions.
            instance_steps: RkCool::create_buffer_f32(r, vec![
                0.0, 0.15, 0.3, // xy offset of the three blue boxes, widedly spaced
                0.0, -0.05, -0.1, // xy offset of the three red boxes, overlapping
            ]),

            // Set the vertex x and y coordinate attribute.
            // No projection, so they’re in GL-space, -1.0 to 1.0 in both directions.
            a_position_x: RkCool::create_buffer_f32(r, vec![
                0.1, 0.0, 0.0, // x positions of the three blue boxes, right-angle
                0.2, 0.15, 0.25, // x positions of the three red boxes, equilateralish
            ]),
            a_position_y: RkCool::create_buffer_f32(r, vec![
                0.1, 0.1, 0.0, // y positions of the three blue boxes, right-angle
                0.4, 0.3, 0.3,  // y positions of the three red boxes, equilateralish
            ]),
        };




        // ATTRIBUTES

        // From WebGL’s point of view, these are global variables which can be
        // read by all shader programs. Each shader program maintains its own
        // set of ‘locations’ (indices) for the attributes that it uses, 
        // __BUT WE HAVE ARRANGED IT SO THAT LOCATIONS MATCH IN ALL SHADERS__.  
        //
        // - Attributes can be read by vertex shaders, but not fragment shaders
        // - Attributes are disabled by default — so, enable_attribute()

        RkCool::enable_attribute(r, AttributeName::InstanceStep);
        RkCool::enable_attribute(r, AttributeName::PositionX);
        RkCool::enable_attribute(r, AttributeName::PositionY);

        // Point the Shader attributes to the correct buffers.
        r.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&refs_buffer.instance_steps));
        RkCool::use_attribute(r, AttributeName::InstanceStep, 1);
        r.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&refs_buffer.a_position_x));
        RkCool::use_attribute(r, AttributeName::PositionX, 1);
        r.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&refs_buffer.a_position_y));
        RkCool::use_attribute(r, AttributeName::PositionY, 1);

        // @TODO describe
        RkCool::set_repeat_gap(r, AttributeName::InstanceStep, 1);




        // UNIFORMS

        // Unlike attributes, WebGL treats uniforms as variables local to each
        // Shader program. So if uniform names match between different programs,
        // they’re not treated as connected, in any way.
        //
        // - Uniforms can be read by vertex _and_ fragment shaders
    
        // Set the sizes of the blue and red boxes, using a uniform.
        let index = indices_shader.blue_box;
        RkCool::use_shader(r, index);
        RkCool::set_uniform_f32(r, index, UniformName::Pointsize, 5.0);

        let index = indices_shader.red_box;
        RkCool::use_shader(r, index);
        RkCool::set_uniform_f32(r, index, UniformName::Pointsize, 10.0);





        Self {
            indices_shader,
            _refs_buffer: refs_buffer,
        }
    }
}

impl Scene for SceneBlueRedBoxes {
    fn render(
        &mut self,
        _develop: &Develop, // the app’s `develop` instance
        r: &RendererWebGl, // the app’s `renderer` instance
        _timer: &Timer, // the app’s `timer` instance
    ) {


        // Switch to the ‘blue box’ Shader.
        RkWarm::use_shader(r, self.indices_shader.blue_box);

        // Draw the three blue boxes.
        RkWarm::draw(r, GL::POINTS, 0, 3);

        // Draw three blue triangles.
        RkWarm::repeat(r, 
            GL::TRIANGLES,
            0, // first — starting index in the array of vector points
            3, // count — number of vertices per instance
            3, // primcount — number of instances
        );


        // Switch to the ‘red box’ Shader.
        RkWarm::use_shader(r, self.indices_shader.red_box);

        // Draw the three red boxes.
        RkWarm::draw(r, GL::POINTS, 3, 3);

        // Draw three red triangles.
        RkWarm::repeat(r, 
            GL::TRIANGLES,
            3, // first — starting index in the array of vector points
            3, // count — number of vertices per instance
            3, // primcount — number of instances
        );

    }
}
