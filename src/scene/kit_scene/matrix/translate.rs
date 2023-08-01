//! Utilities for translating (moving) matrices.

pub fn translate(
    m: [f32; 16], // 4x4 matrix
    dx: f32,
    dy: f32,
    dz: f32,
) -> [f32; 16] {
    [
        m[0],    m[1],     m[2],     m[3],
        m[4],    m[5],     m[6],     m[7],
        m[8],    m[9],     m[10],    m[11],
       m[12]+dx, m[13]+dy, m[14]+dz, m[15],
    ]
}

