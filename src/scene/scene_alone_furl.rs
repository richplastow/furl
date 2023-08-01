//! Demonstrates a single Furl, with no ‘stripes’ or ‘spots’.

use web_sys::WebGlRenderingContext as GL;
use crate::app::Timer;
use crate::develop::{
    CameraPreset,
    Develop,
    GuidesPreset,
    LodPreset,
    WireframePreset,
};
use crate::renderer_webgl::{
    AttributeName,
    RkCool,
    RkWarm,
    RendererWebGl,
    ShaderProgramName,
    ShaderSignatureName,
    UniformName
};
use crate::shape::{Point3,ShapeAxes,ShapeFurl,ShapeGrids};
use super::kit_scene::{PI,SimplePrng,wow};
use super::kit_scene::matrix::{dot,IDENTITY,ortho,orthographic,perspective,rotate_x,rotate_y,translate};
use super::Scene;

// Six fieldsets, each containing four pairs (val_a and val_b) plus four Shadermix, plus four Timermix.
const NUM_AB: usize = 6 * 16;

// Each fieldset contains various numbers of sliders, but there are 7 val_s sliders in total.
const NUM_SINGLE: usize = 7;

const MAX_INSTANCES: usize = 1024;

struct ShaderIndices {
    furl_basic: usize,
    guides: usize,
}

struct Projection {
    choice: [f32; 16],
    orthographic_front: [f32; 16],
    orthographic_left: [f32; 16],
    orthographic_top: [f32; 16],
}
struct View {
    choice: [f32; 16],
    orthographic_front: [f32; 16],
    orthographic_left: [f32; 16],
    orthographic_top: [f32; 16],
}

pub struct Quaternions {
    x: [f32; 4],
    y: [f32; 4],
}

pub struct Guides {
    axes: ShapeAxes,
    grids: ShapeGrids,
}
pub struct Furls {
    furl1: ShapeFurl,
}
pub struct Shapes {
    guides: Guides,
    furls: Furls,
}

pub struct SceneAloneFurl {
    iu_angle: [f32;16],
    iu_bulge: [f32;16],
    iu_lean: [f32;16],
    iu_rise: [f32;16],
    iu_scale: [f32;16],
    iu_tilt: [f32;16],
    parameter_values_raw: String,
    parameter_values: [f32;NUM_AB + NUM_SINGLE],
    projection: Projection,
    quaternions: Quaternions,
    shader_indices: ShaderIndices,
    shapes: Shapes,
    slidermix: [f32;4],
    timermix: [f32;4],
    view: View,
}

impl SceneAloneFurl {
    pub fn new (
        r: &mut RendererWebGl, // the app’s singleton Renderer instance
    ) -> Self {


        // SHADERS

        // Init the shaders, and store them in the `renderer.shaders` vector.
        let shader_indices = ShaderIndices {
            furl_basic: RkCool::add_shader(r, ShaderProgramName::FurlBasic),
            guides: RkCool::add_shader(r, ShaderProgramName::Guides),
        };

        // Make sure all Shaders have compatible ShaderSignatures. @TODO
        // This restriction means that we can switch between shaders in render()
        // without having to rebind buffers to attributes — faster, less faff.
        let name = ShaderSignatureName::FurlBasic;
        // if r.shaders[shader_indices.guides].get_signature_name() != &name {
        //     panic!("Guides does not use the Furl ShaderSignature");
        // }
        if r.shaders[shader_indices.furl_basic].get_signature_name() != &name {
            panic!("FurlBasic does not use the Furl ShaderSignature");
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
        // RkCool::store_signature_locations(r, shader_indices.furl_basic);




        // PRECALCULATE

        // This Scene supports guides.
        let mut axes = ShapeAxes::new();
        let mut grids = ShapeGrids::new();

        // Create the Furls.
        let mut furl1 = ShapeFurl::new(
            0.03, // nubbin_rise_height
            0.02, // nubbin_lower_height
            0.08, // nubbin_nose_length
            0.03, // nubbin_tail_length
            0.16, // nubbin_width
            Point3::new(0., 0., 0.), // placement
        );

        // Attributes for per-instance transformations.

        let maxi = MAX_INSTANCES as f32;
        let mut curves_linear: Vec<f32> = vec![
            // _2p = 0:
            0. / maxi, // eg 0/256, which is 0
            // _2p = 1:
            (maxi / 2.) / maxi, // eg 128/256, which is 1/2
        ];
        // _2p = 2:
        let mut i = maxi / 4. - maxi / 2.; // eg 64 - 128
        while i < maxi - maxi / 2. { // eg 256 - 128
            i += maxi / 2.; // eg i += 128
            curves_linear.push(i / maxi);
        }
        // _2p = 3
        let mut i = maxi / 8. - maxi / 4.; // eg 32 - 64
        while i < maxi - maxi / 4. { // eg 256 - 64
            i += maxi / 4.; // eg i += 64
            curves_linear.push(i / maxi);
        }
        // _2p = 4:
        let mut i = maxi / 16. - maxi / 8.; // eg 16 - 32
        while i < maxi - maxi / 8. { // eg 256 - 32
            i += maxi / 8.; // eg i += 32
            curves_linear.push(i / maxi);
        }
        // _2p = 5:
        let mut i = maxi / 32. - maxi / 16.; // eg 8 - 16
        while i < maxi - maxi / 16. { // eg 256 - 16
            i += maxi / 16.; // eg i += 16
            curves_linear.push(i / maxi);
        }
        // _2p = 6:
        let mut i = maxi / 64. - maxi / 32.; // eg 4 - 8
        while i < maxi - maxi / 32. { // eg 256 - 8
            i += maxi / 32.; // eg i += 8
            curves_linear.push(i / maxi);
        }
        // _2p = 7:
        let mut i = maxi / 128. - maxi / 64.; // eg 2 - 4
        while i < maxi - maxi / 64. { // eg 256 - 4
            i += maxi / 64.; // eg i += 4
            curves_linear.push(i / maxi);
        }
        // _2p = 8:
        let mut i = maxi / 256. - maxi / 128.; // eg 1 - 2
        while i < maxi - maxi / 128. { // eg 256 - 2
            i += maxi / 128.; // eg i += 2
            curves_linear.push(i / maxi);
        }
        // _2p = 9:
        let mut i = maxi / 512. - maxi / 256.;
        while i < maxi - maxi / 256. {
            i += maxi / 256.;
            curves_linear.push(i / maxi);
        }
        // _2p = 10:
        let mut i = maxi / 1024. - maxi / 512.;
        while i < maxi - maxi / 512. {
            i += maxi / 512.;
            curves_linear.push(i / maxi);
        }

        // Combine the precalculated curves into a vector.
        let mut curves: Vec<f32> = vec![];
        let mut prng = SimplePrng::new(123);
        for i in 0..MAX_INSTANCES {
            curves.push(curves_linear[i]); // linear
            curves.push(wow(curves_linear[i])); // wow
            curves.push((curves_linear[i]*PI).sin()); // a ‘hump’, derived from a sine wave
            curves.push(prng.next_float()); // flutter
        }




        // INSTANCE BUFFERS

        let buffer_curves = RkCool::create_buffer_f32(r, curves);




        // VERTEX BUFFERS

        // Aggregate colors and vertices from all shapes, and all LoDs.
        let mut colors: Vec<f32> = vec![];
        colors.append(&mut axes.get_colors(0));
        colors.append(&mut grids.get_colors(colors.len() / 3)); // `/ 3` because each vertex is three numbers
        colors.append(&mut furl1.get_colors(colors.len() / 3));

        let mut vertices: Vec<f32> = vec![];
        vertices.append(&mut axes.get_vertices(0));
        vertices.append(&mut grids.get_vertices(vertices.len() / 3)); // `/ 3` because each vertex is three numbers
        vertices.append(&mut furl1.get_vertices(vertices.len() / 3));

        let buffer_colors = RkCool::create_buffer_f32(r, colors);
        let buffer_vertices = RkCool::create_buffer_f32(r, vertices);

        // @TODO describe
        // Note that ShapeGrids does not need to connect together any vertices.
        let mut cnnx: Vec<u16> = vec![];
        cnnx.append(&mut axes.get_cnnx(0));
        cnnx.append(&mut furl1.get_cnnx(cnnx.len()));
        RkCool::create_index_buffer_u16(r, cnnx);




        // INDEX ATTRIBUTES

        // From WebGL’s point of view, these are global variables which can be
        // read by all shader programs. Each shader program maintains its own
        // set of ‘locations’ (indices) for the attributes that it uses, 
        // __BUT WE HAVE ARRANGED IT SO THAT LOCATIONS MATCH IN ALL SHADERS__.  
        //
        // - Attributes can be read by vertex shaders, but not fragment shaders
        // - Attributes are disabled by default — so, enable_attribute()

        RkCool::enable_attribute(r, AttributeName::Curves);

        // Point the Shader attributes to the correct buffers.
        r.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_curves));
        RkCool::use_attribute(r, AttributeName::Curves, 4);

        // @TODO describe
        RkCool::set_repeat_gap(r, AttributeName::Curves, 1);




        // VERTEX ATTRIBUTES

        RkCool::enable_attribute(r, AttributeName::Color);
        RkCool::enable_attribute(r, AttributeName::Position);

        // Point the Shader attributes to the correct buffers.
        r.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_colors));
        RkCool::use_attribute(r, AttributeName::Color, 3);
        r.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_vertices));
        RkCool::use_attribute(r, AttributeName::Position, 3);

        // Switch to ShaderFurlBasic, ready for the first render() call.
        RkCool::use_shader(r, shader_indices.furl_basic);

        let ortho_zoom = 2.8; // smaller is more zoomed in

        Self {
            // Set initial values for the instance uniforms. These will be 
            // filled using parameters b_2, b_3, s_2, etc.
            iu_angle: [0.;16],
            iu_bulge: [0.;16],
            iu_lean: [0.;16],
            iu_rise: [0.;16],
            iu_scale: [0.;16],
            iu_tilt: [0.;16],

            parameter_values_raw: "".into(), // the first app.update_state() will fill this
            parameter_values: [0.;NUM_AB + NUM_SINGLE], // the first app.update_state() will fill this
            projection: Projection {
                choice: perspective(
                    20.,
                    r.aspect_ratio,
                    0.1,
                    100.0
                ),
                orthographic_front: ortho(
                    0.,
                    r.aspect_ratio * ortho_zoom,
                    ortho_zoom,
                    0.,
                    0.1,
                    100.,
                ),
                orthographic_left: ortho(
                    0.,
                    r.aspect_ratio * ortho_zoom,
                    ortho_zoom,
                    0.,
                    0.1,
                    100.,
                ),
                orthographic_top: orthographic(
                    0.,
                    r.aspect_ratio * ortho_zoom,
                    ortho_zoom,
                    0.,
                    0.1,
                    100.,
                ),
            },
            quaternions: Quaternions {
                x: [
                    1.0,0.0,0.0, // normalised axis
                    0.0 // initial angle in radians (updated using the qx3 parameter)
                ],
                y: [
                    0.0,1.0,0.0,
                    0.0
                ],
            },
            shader_indices,
            shapes: Shapes {
                guides: Guides {
                    axes,
                    grids,
                },
                furls: Furls {
                    furl1,
                },
            },
            slidermix: [0., 0., 0., 0.],
            timermix: [0., 0., 0., 0.],
            view: View {
                choice: dot(
                    rotate_x(IDENTITY, PI * -0.25), // tilt the camera down 45°
                    translate(IDENTITY, -0.3, 4., -4.), // raise the camera and pull it back
                ),
                orthographic_front:
                    translate(IDENTITY, r.aspect_ratio * 0.5 * ortho_zoom, 0.5 * ortho_zoom, -4.),
                orthographic_left: dot(
                    translate(IDENTITY, r.aspect_ratio * 0.5 * ortho_zoom, 0.5 * ortho_zoom, -4.),
                    rotate_y(IDENTITY, PI * 0.5), // tilt the camera to the right 90°
                ),
                orthographic_top: dot(
                    translate(IDENTITY, r.aspect_ratio * 0.5 * ortho_zoom, 0.5 * ortho_zoom, -4.), // y was ` 0.5 * ortho_zoom - 1.96` ??!!
                    rotate_x(IDENTITY, PI * -0.5), // tilt the camera down 90°
                ),

            },
        }
    }
}

impl Scene for SceneAloneFurl {
    fn render(
        &mut self,
        develop: &Develop, // the app’s `develop` instance
        r: &RendererWebGl, // the app’s singleton Renderer instance
        timer: &Timer, // the app’s `timer` instance
    ) {
        // Get presets from the app’s `develop` instance.
        let lod: u8 = match develop.lod_preset {
            LodPreset::All0 => 0,
            _ => 1,
        };
        let wireframe_mode = match develop.wireframe_preset {
            WireframePreset::Dots => GL::POINTS,
            WireframePreset::Lines => GL::LINE_STRIP,
            _ => GL::TRIANGLES,
        };

        // Convert the "_2p" slider to a `primcount` value, so 4 => 16, 5 => 32.
        let primcount = (2u32).pow(self.get_parameter_value("_2p", 0) as u32);

        // Update the [f32;4] uniforms which will combine with curves and
        // instance attributes.
        self.iu_angle = [
            self.get_parameter_value("a_0", 0) * primcount as f32,
            self.get_parameter_value("a_1", 0),
            self.get_parameter_value("a_2", 0),
            self.get_parameter_value("a_3", 0),
            self.get_parameter_value("a_0", NUM_AB / 4) * primcount as f32,
            self.get_parameter_value("a_1", NUM_AB / 4),
            self.get_parameter_value("a_2", NUM_AB / 4),
            self.get_parameter_value("a_3", NUM_AB / 4),
            self.get_parameter_value("a_0", NUM_AB / 2),
            self.get_parameter_value("a_1", NUM_AB / 2),
            self.get_parameter_value("a_2", NUM_AB / 2),
            self.get_parameter_value("a_3", NUM_AB / 2),
            self.get_parameter_value("a_0", NUM_AB * 3 / 4),
            self.get_parameter_value("a_1", NUM_AB * 3 / 4),
            self.get_parameter_value("a_2", NUM_AB * 3 / 4),
            self.get_parameter_value("a_3", NUM_AB * 3 / 4),
        ];
        self.iu_bulge = self.get_parameter_mat4("b_0", "b_1", "b_2", "b_3");
        self.iu_lean = self.get_parameter_mat4("l_0", "l_1", "l_2", "l_3");
        self.iu_rise = self.get_parameter_mat4("r_0", "r_1", "r_2", "r_3");
        self.iu_scale = self.get_parameter_mat4("s_0", "s_1", "s_2", "s_3");
        self.iu_tilt = self.get_parameter_mat4("t_0", "t_1", "t_2", "t_3");

        // Rotate on the x- and y-axis according to the qx3 and qy3 parameters.
        self.quaternions.x = [
            self.quaternions.x[0], self.quaternions.x[1], self.quaternions.x[2], // normalised axis
            self.get_parameter_value("qx3", 0), // angle in radians
        ];
        self.quaternions.y = [
            self.quaternions.y[0], self.quaternions.y[1], self.quaternions.y[2],
            self.get_parameter_value("qy3", 0),
        ];

        self.slidermix = [
            self.get_parameter_value("sm0", 0),
            self.get_parameter_value("sm1", 0),
            self.get_parameter_value("sm2", 0),
            self.get_parameter_value("sm3", 0),
        ];
        self.timermix = [
            timer.beat_continuous_normalised, // linear
            ((timer.beat_continuous_normalised - 0.25) * 2.0 * PI).sin() / 2.0 + 0.5, // ‘ease-in-out’, derived from a sine wave
            timer.beat4_continuous_normalised, // linear, quarter speed
            ((timer.beat4_continuous_normalised - 0.25) * 2.0 * PI).sin() / 2.0 + 0.5,
        ];

        // @TODO only send uniforms to WebGL which have changed since the last render()? Or is that premature optimisation?

        let projection = match develop.camera_preset {
            CameraPreset::OrthographicFront => self.projection.orthographic_front,
            CameraPreset::OrthographicLeft => self.projection.orthographic_left,
            CameraPreset::OrthographicTop => self.projection.orthographic_top,
            _ => self.projection.choice,
        };
        let view = match develop.camera_preset {
            CameraPreset::OrthographicFront => self.view.orthographic_front,
            CameraPreset::OrthographicLeft => self.view.orthographic_left,
            CameraPreset::OrthographicTop => self.view.orthographic_top,
            _ => self.view.choice,
        };
        let shader_index = self.shader_indices.furl_basic;
        RkWarm::set_uniform_mat4_f32(r, shader_index, UniformName::ProjectionMatrix, projection);
        RkWarm::set_uniform_mat4_f32(r, shader_index, UniformName::ViewMatrix, view);
        RkWarm::set_uniform_vec4_f32(r, shader_index, UniformName::QuaternionX, self.quaternions.x);
        RkWarm::set_uniform_vec4_f32(r, shader_index, UniformName::QuaternionY, self.quaternions.y);
        RkWarm::set_uniform_vec4_f32(r, shader_index, UniformName::Slidermix, self.slidermix);
        RkWarm::set_uniform_vec4_f32(r, shader_index, UniformName::Timermix, self.timermix);

        RkWarm::set_uniform_mat4_f32(r, shader_index, UniformName::Angle, self.iu_angle);
        RkWarm::set_uniform_mat4_f32(r, shader_index, UniformName::Bulge, self.iu_bulge);
        RkWarm::set_uniform_mat4_f32(r, shader_index, UniformName::Lean, self.iu_lean);
        RkWarm::set_uniform_mat4_f32(r, shader_index, UniformName::Rise, self.iu_rise);
        RkWarm::set_uniform_mat4_f32(r, shader_index, UniformName::Scale, self.iu_scale);
        RkWarm::set_uniform_mat4_f32(r, shader_index, UniformName::Tilt, self.iu_tilt);


        // Render the furls.
        self.shapes.furls.furl1.render(r, shader_index, lod, wireframe_mode, primcount);


        // Maybe show guides.
        // Without guides showing, render() doesn’t have to call use_shader() or
        // set_uniform_*(), so it may run a bit faster. @TODO benchmark
        if develop.guides_preset != GuidesPreset::ChosenByScene
            && develop.guides_preset != GuidesPreset::NoGuides {

            // Switch to ShaderGuides.
            RkWarm::use_shader(r, self.shader_indices.guides);
            RkWarm::set_uniform_mat4_f32(r, self.shader_indices.guides, UniformName::ProjectionMatrix, projection);
            RkWarm::set_uniform_mat4_f32(r, self.shader_indices.guides, UniformName::ViewMatrix, view);

            // Render the ShapeAxes, or the ShapeGrids, or both.
            let guides_preset = &develop.guides_preset;
            match guides_preset {
                GuidesPreset::All10m | GuidesPreset::All1m => {
                    self.shapes.guides.axes.render(r, guides_preset);
                    self.shapes.guides.grids.render(r, guides_preset);
                },            
                GuidesPreset::AxesOnly10m | GuidesPreset::AxesOnly1m => {
                    self.shapes.guides.axes.render(r, guides_preset);
                },
                GuidesPreset::GridsOnly10m | GuidesPreset::GridsOnly1m => {
                    self.shapes.guides.grids.render(r, guides_preset);
                },
                _ => (), // unreachable, because of the `if` conditional
            }

            // Switch back to ShaderFurlBasic, ready for the next render() call.
            RkWarm::use_shader(r, self.shader_indices.furl_basic);
            RkWarm::set_uniform_mat4_f32(r, self.shader_indices.furl_basic, UniformName::ProjectionMatrix, projection);
            RkWarm::set_uniform_mat4_f32(r, self.shader_indices.furl_basic, UniformName::ViewMatrix, view);
        }
    }

    fn get_fieldsets(&self) -> String {        
        r#"[
            { "kind":"single", "id":"slidermix", "heading":"Slidermix", "parameters":[
              { "name":"sm0", "min":0, "max":1, "step":0.01, "title":"i\nu_slidermix[0]" },
              { "name":"sm1", "min":0, "max":1, "step":0.01, "title":"ii\nu_slidermix[1]" },
              { "name":"sm2", "min":0, "max":1, "step":0.01, "title":"iii\nu_slidermix[2]" },
              { "name":"sm3", "min":0, "max":1, "step":0.01, "title":"iv\nu_slidermix[3]" }
            ]},
            { "kind":"single", "id":"density", "heading":"Density", "parameters":[
              { "name":"_2p", "min":0, "max":10, "step":1,   "title":"\nprimcount (2^n)" }
            ]},
            { "kind":"iu", "id":"angle", "heading":"Angle", "sm":[0,0,0,0], "tm":[0,0,0,0], "parameters":[
              { "name":"a_0", "min":0,     "max":6.28, "step":0.01, "title":"Linear\niu_angle[0]" },
              { "name":"a_1", "min":-1,    "max":1,    "step":0.01, "title":"Wow\niu_angle[1]" },
              { "name":"a_2", "min":-1,    "max":1,    "step":0.01, "title":"Sine\niu_angle[2]" },
              { "name":"a_3", "min":-3.14, "max":3.14, "step":0.01, "title":"Invariant\niu_angle[3]" }
            ]},
            { "kind":"iu", "id":"bulge", "heading":"Bulge", "sm":[0,0,0,0], "tm":[0,0,0,0], "parameters":[
              { "name":"b_0", "min":-1,    "max":1,    "step":0.01, "title":"Linear\niu_bulge[0]" },
              { "name":"b_1", "min":-0.1,  "max":0.1,  "step":0.001,"title":"Flutter\niu_bulge[1]" },
              { "name":"b_2", "min":-1,    "max":1,    "step":0.01, "title":"Sine\niu_bulge[2]" },
              { "name":"b_3", "min":-1,    "max":1,    "step":0.01, "title":"Invariant\niu_bulge[3]" }
            ]},
            { "kind":"iu", "id":"lean", "heading":"Lean", "sm":[0,0,0,0], "tm":[0,0,0,0], "parameters":[
              { "name":"l_0", "min":-3.14, "max":3.14, "step":0.01, "title":"Inclination Linear\niu_lean[0]" },
              { "name":"l_1", "min":-3.14, "max":3.14, "step":0.01, "title":"Inclination Invariant\niu_lean[1]" },
              { "name":"l_2", "min":-3.14, "max":3.14, "step":0.01, "title":"Orientation Linear\niu_lean[2]" },
              { "name":"l_3", "min":-3.14, "max":3.14, "step":0.01, "title":"Orientation Invariant\niu_lean[3]" }
            ]},
            { "kind":"iu", "id":"rise", "heading":"Rise", "sm":[0,0,0,0], "tm":[0,0,0,0], "parameters":[
              { "name":"r_0", "min":-2,    "max":2,    "step":0.01, "title":"Linear\niu_rise[0]" },
              { "name":"r_1", "min":-0.1,  "max":0.1,  "step":0.001,"title":"Wow\niu_rise[1]" },
              { "name":"r_2", "min":-2,    "max":2,    "step":0.01, "title":"Sine\niu_rise[2]" },
              { "name":"r_3", "min":-2,    "max":2,    "step":0.01, "title":"Invariant\niu_rise[3]" }
            ]},
            { "kind":"iu", "id":"scale", "heading":"Scale", "sm":[0,0,0,0], "tm":[0,0,0,0], "parameters":[
              { "name":"s_0", "min": -4,   "max":4,    "step":0.01, "title":"Linear\niu_scale[0]" },
              { "name":"s_1", "min":-1,    "max":1,    "step":0.01, "title":"Wow\niu_scale[1]" },
              { "name":"s_2", "min": -4,   "max":4,    "step":0.01, "title":"Sine\niu_scale[2]" },
              { "name":"s_3", "min": -4,   "max":4,    "step":0.01, "title":"Invariant\niu_scale[3]" }
            ]},
            { "kind":"iu", "id":"tilt", "heading":"Tilt", "sm":[0,0,0,0], "tm":[0,0,0,0], "parameters":[
              { "name":"t_0", "min": 0,    "max":6.28, "step":0.1,  "title":"Linear\niu_tilt[0]" },
              { "name":"t_1", "min":-1,    "max":1,    "step":0.01, "title":"Flutter\niu_tilt[1]" },
              { "name":"t_2", "min":-3.14, "max":3.14, "step":0.1,  "title":"Sine\niu_tilt[2]" },
              { "name":"t_3", "min":-4.71, "max":1.57, "step":0.1,  "title":"Invariant\niu_tilt[3]" }
            ]},
            { "kind":"single", "id":"rotate-furl", "heading":"Rotate Furl", "parameters":[
              { "name":"qx3", "min":-3.14, "max":3.14, "step":0.01, "title":"X\nquaternions.x[3]" },
              { "name":"qy3", "min":-3.14, "max":3.14, "step":0.01, "title":"Y\nquaternions.y[3]" }
            ]}
        ]"#.into()
    }

    fn get_presets(&self) -> String {        
        r#"[
            { "title":"Default", "notes":"A simple phyllotactic Furl. Use Slidermix sm0 to grow and shrink.", "values": [
              2.4,0,0.2,-3.14,0,0,0.2,0,0,0,0,0,0.5,0,0,0,0,0,1.5,0.5,1.6,0,-0.04,-1.61,
              2.4,0,0.2,3.14,0,0,0.29,0,0,0,0,0,1.18,0,0,-0.29,0.46,0,2.41,0.81,2.1,0,-0.04,-1.61,
              0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,1,0,0,0,1,0,0,0,
              0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
              0,0,0,0,5,0,0,
              0,0,1,1,1,1,1,1,0
              ]}
            ,{ "title":"Zero", "notes":"A blank slate, useful for starting new Furl designs. All values are set to zero, apart from Angle Invariant (set to slowly rotate) and Scale Invariant (set to 4).", "values": [
              0,0,0,-3.14, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,4, 0,0,-0.04,-0.01,
              0,0,0,3.14, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,4, 0,0,-0.04,-0.01,
              0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0,
              0,0,1,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0,
              0,0,0,0,0,0,0,
              1,1,0,1,1,1,0,1,0
              ]}
            ,{ "title":"Column", "values": [
              0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,-1.2,0,0,0,0.5,0,0,-0.04,-0.01,
              0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,-1.2,0,0,0,0.5,0,0,-0.04,-0.01,
              0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0,
              0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0,
              0,0,0,0,4,0,0,
              0,0,0,0,0,0,0,0,0
              ]}
            ,{ "title":"Dancing Pinecone", "values": [
              2.4,0,0.2,-1.67,0,0,0.39,0,0.21,0,0,-3.14,1.33,0,-0.27,-0.55,0,0,1.5,0.5,6.2,0,-0.04,-1.61,
              2.4,0,0.2,-1.67,0,0,0.39,0,0.24,0,0,3.14,1.54,0,0,-0.55,0,0,1.5,0.5,0,0,-0.04,-1.61,
              0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 
              0,0,0,0, 0,0,0,0, 0,1,1,0, 0,1,0,0, 0,0,0,0, 0,0,1,0,
              0,0,0,0,9,0.34,0.13,
              0,0,0,0,0,0,0,0,0
              ]}
            ,{ "title":"Spiral Claw", "values": [
              1.26,0,0.33,-3.14,0.16,0,0.63,-0.01,0.2,1.69,0.51,-0.31,-0.97,0,0.3,-0.02,0.05,0,1.95,-0.1,4.4,0,-0.74,-4.51,
              1.26,0,0.33,3.14,-0.03,0,0.63,-0.01,0.2,1.69,0.51,-0.31,-1.29,0,0.3,-0.02,0.05,0,1.95,-0.1,2.4,0,-0.74,-4.51,
              0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0,
              0,0,1,0, 0,1,0,0, 0,0,0,0, 0,1,0,0, 0,0,0,0, 0,1,0,0,
              0,0,0,0,10,1.02,-1.02,
              0,0,0,0,0,0,0,0,0
              ]}
            ,{ "title":"Armadillo", "notes":"This abstract armadillo was found by Bertie Young", "values": [
              2.4,0,0.2,0,0,-0.079,1,0,3.14,3.14,3.14,3.14,2,-0.1,-2,2,3.03,0,1.5,0.5,1.5,0.1,2.06,-1.51,
              2.4,0,0.2,0,0,-0.079,1,0,3.14,3.14,3.14,3.14,2,-0.1,-2,2,3.03,0,1.5,0.5,1.5,0.1,2.06,-1.51,
              0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0,
              0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0,
              0,0,0,0,10,0,-2.55,
              0,0,0,0,0,0,0,0,0
              ]}
            ,{ "title":"Horn", "values": [
              5.88,0,0.2,1.99,-0.51,0,0.2,0,-3.14,0,0,0,1.57,0,0,-0.37,-2.02,0,1.5,0.5,1.6,0,-0.04,-1.61,
              5.88,0,0.2,1.99,-0.51,0,0.2,0,-3.14,0,0,0,1.57,0,0,-0.37,-2.02,0,1.5,0.5,1.6,0,-0.04,-1.61,
              0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0,
              0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0,
              0,0,0,0,10,0.4,-1.83,
              0,0,0,0,0,0,0,0,0
              ]}
            ,{ "title":"Cup", "values": [
              0.13,0.04,-1,0.6,-0.88,0,0.2,0.52,0,0,0,0,-1.24,-0.025,0,0.26,0,0,1.5,0.5,0,0.47,-1.04,-1.71,
              0.13,0.04,-1,0.6,-0.88,0,0.2,0.52,0,0,0,0,-1.24,-0.025,0,0.26,0,0,1.5,0.5,0,0.47,-1.04,-1.71,
              0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0,
              0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0,
              0,0,0,0,10,0.35,-0.01,
              0,0,0,0,0,0,0,0,0
              ]}
        ]"#.into()
    }

    fn get_parameter_mat4(&self, col0: &str, col1: &str, col2: &str, col3: &str) -> [f32;16] {
        [
            self.get_parameter_value(col0, 0),
            self.get_parameter_value(col1, 0),
            self.get_parameter_value(col2, 0),
            self.get_parameter_value(col3, 0),
            self.get_parameter_value(col0, NUM_AB / 4),
            self.get_parameter_value(col1, NUM_AB / 4),
            self.get_parameter_value(col2, NUM_AB / 4),
            self.get_parameter_value(col3, NUM_AB / 4),
            self.get_parameter_value(col0, NUM_AB / 2),
            self.get_parameter_value(col1, NUM_AB / 2),
            self.get_parameter_value(col2, NUM_AB / 2),
            self.get_parameter_value(col3, NUM_AB / 2),
            self.get_parameter_value(col0, NUM_AB * 3 / 4),
            self.get_parameter_value(col1, NUM_AB * 3 / 4),
            self.get_parameter_value(col2, NUM_AB * 3 / 4),
            self.get_parameter_value(col3, NUM_AB * 3 / 4),
        ]
    }

    fn get_parameter_value(&self, name: &str, offset: usize) -> f32 {
        match name {
            "a_0" => self.parameter_values[offset + 0],
            "a_1" => self.parameter_values[offset + 1],
            "a_2" => self.parameter_values[offset + 2],
            "a_3" => self.parameter_values[offset + 3],
            "b_0" => self.parameter_values[offset + 4],
            "b_1" => self.parameter_values[offset + 5],
            "b_2" => self.parameter_values[offset + 6],
            "b_3" => self.parameter_values[offset + 7],
            "l_0" => self.parameter_values[offset + 8],
            "l_1" => self.parameter_values[offset + 9],
            "l_2" => self.parameter_values[offset + 10],
            "l_3" => self.parameter_values[offset + 11],
            "r_0" => self.parameter_values[offset + 12],
            "r_1" => self.parameter_values[offset + 13],
            "r_2" => self.parameter_values[offset + 14],
            "r_3" => self.parameter_values[offset + 15],
            "s_0" => self.parameter_values[offset + 16],
            "s_1" => self.parameter_values[offset + 17],
            "s_2" => self.parameter_values[offset + 18],
            "s_3" => self.parameter_values[offset + 19],
            "t_0" => self.parameter_values[offset + 20],
            "t_1" => self.parameter_values[offset + 21],
            "t_2" => self.parameter_values[offset + 22],
            "t_3" => self.parameter_values[offset + 23],
            "sm0" => self.parameter_values[NUM_AB + 0],
            "sm1" => self.parameter_values[NUM_AB + 1],
            "sm2" => self.parameter_values[NUM_AB + 2],
            "sm3" => self.parameter_values[NUM_AB + 3],
            "_2p" => self.parameter_values[NUM_AB + 4],
            "qx3" => self.parameter_values[NUM_AB + 5],
            "qy3" => self.parameter_values[NUM_AB + 6],
            _ => panic!("Parameter name {} does not exist", name),
        }
    }

    fn set_parameter_values(&mut self, values_raw: String) {
        if self.parameter_values_raw == values_raw { return }
        self.parameter_values_raw = values_raw;
        let values_split = self.parameter_values_raw.split(",");
        let mut i: usize = 0;
        for value_string in values_split {
            let value: f32 = value_string.parse().unwrap();
            self.parameter_values[i] = value;
            i += 1;
        }
        // crate::log(&format!("parameter_values_raw: {}", self.parameter_values_raw));
        // crate::log(&format!("parameter_values[0]: {}", self.parameter_values[0]));
        // crate::log(&format!("parameter_values[1]: {}", self.parameter_values[1]));
    }

}

