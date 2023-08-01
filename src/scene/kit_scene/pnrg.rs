//! A simple pseudo-random number generator.


/// For our simple use-case, there’s no need for bulky RNG dependencies.
/// Besides, Rust’s RNGs don’t play nice with WASM:
/// `Fails to build on wasm32-unknown-unknown using getrandom`
/// 
/// From https://gist.github.com/blixt/f17b47c62508be59987b
/// Uses an optimized version of the Park-Miller PRNG.
/// http://www.firstpr.com.au/dsp/rand31/
pub struct SimplePrng {
    seed: u32,
}

impl SimplePrng {
    pub fn new(seed_arg: u32) -> Self {
        let mut seed = seed_arg % 2147483647;
        if seed <= 0 { seed += 2147483646 }
        Self { seed }
    }
  
    /// Returns a pseudo-random value between 1 and 2^32 - 2.
    pub fn next(&mut self) -> u32 {
        self.seed = self.seed * 16807 % 2147483647;
        self.seed
    }

    /// Returns a pseudo-random floating point number in range [0, 1).
    pub fn next_float(&mut self) -> f32 {
        // We know that result of next() will be 1 to 2147483646 (inclusive).
        return (self.next() - 1) as f32 / 2147483646.;
    }
}
