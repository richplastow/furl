//! A smooth, pseudo-random looking curve.

pub const PI: f32 = std::f32::consts::PI;

pub fn wow(x: f32) -> f32 {
    // sin(2πx)/3 + 
    (2. * PI * x).sin() / 3. + 
    // sin(3πx)/3 + 
    (3. * PI * x).sin() / 3. + 
    // sin(5πx)/3 + 
    (5. * PI * x).sin() / 3. + 
    // sin(13πx)/5 + 
    (13. * PI * x).sin() / 5. + 
    // sin(23πx)/13 + 
    (23. * PI * x).sin() / 13. + 
    // sin(77πx)/51 
    (77. * PI * x).sin() / 51.
}
