extern crate console_error_panic_hook;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::develop::{CameraPreset,GuidesPreset,Develop,LodPreset,log,WireframePreset};
use crate::scene::{SceneContainer,SceneContainerName};
use crate::renderer_webgl::{RendererWebGl,RkWarm};
use super::Timer;

/// Wraps state, and the public-facing ‘Phase N’ methods.
#[wasm_bindgen]
pub struct App {
    develop: Develop,
    renderer: RendererWebGl,
    scene_container: SceneContainer,
    timer: Timer,
}

#[wasm_bindgen]
impl App {

    pub fn new(
        canvas_extent_horizontal: f32,
        canvas_extent_vertical: f32,
        canvas_id: String,
        scene_container_name: SceneContainerName,
        camera_preset: CameraPreset,
        guides_preset: GuidesPreset,
        lod_preset: LodPreset,
        wireframe_preset: WireframePreset,
    ) -> Self {

        // If a panic occurs, pass it to the browser’s `console.error()`.
        // github.com/rustwasm/console_error_panic_hook#usage
        console_error_panic_hook::set_once();

        log(&format!(
            "Canvas: {} {} {} {:?} {:?} {:?} {:?} {:?}",
            canvas_extent_horizontal,
            canvas_extent_vertical,
            canvas_id,
            scene_container_name,
            camera_preset,
            guides_preset,
            lod_preset,
            wireframe_preset,
        ));

        // Instantiate the renderer.
        let mut renderer = RendererWebGl::new(
            canvas_extent_horizontal,
            canvas_extent_vertical,
            canvas_id,
        );

        Self {
            develop: Develop {
                camera_preset,
                guides_preset,
                lod_preset,
                wireframe_preset,
            },
            scene_container: SceneContainer::new(&mut renderer, scene_container_name),
            renderer, // note, must be placed AFTER `...(&mut renderer, ...)`
            timer: Timer::new(),
        }
    }




    // DEVELOP

    pub fn log_timer(&self) {
        log(&"log_timer() was called!");
    }

    pub fn get_fieldsets(&self) -> String {
        self.scene_container.scene.get_fieldsets()
    }

    pub fn get_presets(&self) -> String {
        self.scene_container.scene.get_presets()
    }




    // TICK

    /// Runs each of the nine phases, in the proper order.
    /// This method should be run on each tick, in the production environment.
    pub fn tick(
        &mut self,
        time_in_ms: f32,
        origin_x: f32,
        origin_y: f32,
        down_evt_x: f32,
        down_evt_y: f32,
        camera_preset: CameraPreset,
        guides_preset: GuidesPreset,
        lod_preset: LodPreset,
        wireframe_preset: WireframePreset,
        parameter_values_raw: String,
    ) {
        self.update_state(
            time_in_ms,
            origin_x,
            origin_y,
            down_evt_x,
            down_evt_y,
            camera_preset,
            guides_preset,
            lod_preset,
            wireframe_preset,
            parameter_values_raw,
        );
        self.advise();
        self.reschedule();
        self.execute();
        self.simulate();
        self.draw();
        self.quickdraw();
        self.render();
        self.report()
    }




    /// Phase 1: Update the App’s state.
    pub fn update_state(
        &mut self,
        time_in_ms: f32,
        origin_x: f32,
        origin_y: f32,
        down_evt_x: f32,
        down_evt_y: f32,
        camera_preset: CameraPreset,
        guides_preset: GuidesPreset,
        lod_preset: LodPreset,
        wireframe_preset: WireframePreset,
        parameter_values_raw: String,
    ) {
        // Phase 1A: Update the App’s internal Timer.
        // Note that the App uses seconds, not milliseconds, internally.
        self.timer.update(time_in_ms);


        // Phase 1B: Take note of any recent input events.
        if down_evt_x > 0.0 {
            let down_x = (down_evt_x - origin_x) as u16;
            let down_y = (down_evt_y - origin_y) as u16;
            log(&format!("down: {:?}, {:?}", down_x, down_y));
        };

        if self.develop.camera_preset != camera_preset {
            self.develop.camera_preset = camera_preset;
            // log(&format!("camera_preset: {:?}", self.develop.camera_preset));
        }
        if self.develop.guides_preset != guides_preset {
            self.develop.guides_preset = guides_preset;
            // log(&format!("guides_preset: {:?}", self.develop.guides_preset));
        }
        if self.develop.lod_preset != lod_preset {
            self.develop.lod_preset = lod_preset;
            // log(&format!("lod_preset: {:?}", self.develop.lod_preset));
        }
        if self.develop.wireframe_preset != wireframe_preset {
            self.develop.wireframe_preset = wireframe_preset;
            // log(&format!("wireframe_preset: {:?}", self.develop.wireframe_preset));
        }
        self.scene_container.scene.set_parameter_values(parameter_values_raw);
    }




    // Phase 2: Propogate some of the state to the Nodes that need to know.
    // Will generate 0..n Reply instances.
    pub fn advise(
        &mut self,
    ) {

    }




    // Phase 3: Update the Schedule of Ops.
    pub fn reschedule(
        &mut self,
    ) {
       // Phase 3A: Transform the Reply instances to Op instances.
        // Will produce a vector of Ops with times.

        // Phase 3B: Use the new Ops to cancel some Ops in the Schedule.
        // A scheduled GUI prompt is cancelled if the user clicks the widget.

        // Phase 3C: Add the new Ops (not the cancellers) to the Schedule.
        // Some of the new Ops have time 0, to execute them immediately.

    }




    // Phase 4: Execute all Ops in the Schedule whose time has expired.
    // And remove the executed Ops which were not set to repeat.
    pub fn execute(
        &mut self,
    ) {
    }




    // Phase 5: Run physics simulations, animations and tweens.
    pub fn simulate(
        &mut self,
    ) {
    }




    // Phase 6: Propogate the draw() call to all visible Nodes.
    // Will generate 0..n Shapes.
    pub fn draw(
        &mut self,
    ) {
    }




    // Phase 7: Cull some Shapes and sort the rest in z-order.
    // May reduce the number of Shapes.
    pub fn quickdraw(
        &mut self,
    ) {
    }




    // Phase 8: Clear the canvas and render the Shapes.
    // Eventually we could substitute WebGL for WebGL2 or WebCPU.
    pub fn render(
        &mut self,
    ) {
        RkWarm::clear(&self.renderer);
        self.scene_container.scene.render(&self.develop, &self.renderer, &self.timer);
    }




    // Phase 9: If certain state values have changed, inform the browser.
    // For example, the cursor may need to change to a pointer.
    pub fn report(
        &self,
    ) {
    }

}
