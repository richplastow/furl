use crate::app::Timer;
use crate::develop::Develop;
use crate::renderer_webgl::RendererWebGl;

pub trait Scene {
    fn render(
        &mut self,
        _develop: &Develop,
        _renderer: &RendererWebGl,
        _timer: &Timer,
    ) {}

    fn get_fieldsets(&self) -> String {
        "[]".into() // empty array by default
    }

    fn get_presets(&self) -> String {
        "[]".into() // empty array by default
    }

    fn get_parameter_mat4(&self, _col0: &str, _col1: &str, _col2: &str, _col3: &str) -> [f32;16] {
        panic!("This scene does not support parameters")
    }

    fn get_parameter_value(&self, _name: &str, _offset: usize) -> f32 {
        panic!("This scene does not support parameters")
    }

    fn set_parameter_values(&mut self, _parameter_values: String) {
        // no-op by default
    }
}
