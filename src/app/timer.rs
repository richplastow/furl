/// Used by the App to keep track of time.
#[derive(Debug)]
pub struct Timer {
    pub bar: u8, // lasts 6.4 seconds — there are 32 bars in a song
    // pub bar_continuous_normalised: f32, // like bar, but ranges from 0.0 to (nearly) 1.0
    pub beat: u8, // lasts 0.8 seconds — there are 8 beats in a bar
    pub beat_continuous_normalised: f32, // like beat, but ranges from 0.0 to (nearly) 1.0
    pub beat4: u8, // lasts 3.2 seconds
    pub beat4_continuous_normalised: f32,
    pub grid: u8, // lasts a tenth of a second — there are 8 grids in a beat
    pub time_delta: f32,
    pub time: f32,
    pub paused_total: f32,
    pub paused: bool,
}

impl Timer {

    pub fn new() -> Self {
        Self {
            bar: 0,
            // bar_continuous_normalised: 0.0,
            beat: 0,
            beat_continuous_normalised: 0.0,
            beat4: 0,
            beat4_continuous_normalised: 0.0,
            grid: 0,
            paused_total: 0.0,
            paused: false, //@TODO start the game paused, so set this to `true`
            time_delta: 0.0,
            time: 0.0,
        }
    }

    pub fn update(
        &mut self,
        time_in_ms: f32,
    ) {
        let time = time_in_ms / 1000.0; // simpler to deal in seconds
        if self.paused {
            self.paused_total += time - self.time;
        };
        self.time_delta = time - self.time;
        self.time = time;
        let unpaused = time - self.paused_total;

        // Calculate bar.
        let bar = (unpaused / 6.4) % 32.0;
        self.bar = bar.floor() as u8;
        // self.bar_continuous_normalised = bar / 32.0;

        // Calculate beat.
        let beat = (unpaused / 0.1) % 64.0;
        self.beat = beat.floor() as u8;
        self.beat_continuous_normalised = beat / 64.0;

        // Calculate beat4.
        let beat4 = (unpaused / 0.4) % 64.0;
        self.beat4 = beat4.floor() as u8;
        self.beat4_continuous_normalised = beat4 / 64.0;
   }

}