//! Utilities for rotating matrices.

pub fn rotate_x(
    m: [f32; 16], // 4x4 matrix
    angle: f32, // radians
) -> [f32; 16] {
    let mut out = [
        m[0],   m[1],  m[2],  m[3],
        m[4],   m[5],  m[6],  m[7],
        m[8],   m[9], m[10], m[11],
        m[12], m[13], m[14], m[15],
    ];

    let c = f32::cos(angle);
    let s = f32::sin(angle);
    let mv1 = m[1];
    let mv5 = m[5];
    let mv9 = m[9];

    out[1] = m[1]*c-m[2]*s;
    out[5] = m[5]*c-m[6]*s;
    out[9] = m[9]*c-m[10]*s;

    out[2] = m[2]*c+mv1*s;
    out[6] = m[6]*c+mv5*s;
    out[10] = m[10]*c+mv9*s;

    out
}


pub fn rotate_y(
    m: [f32; 16],
    angle: f32,
) -> [f32; 16] {
    let mut out = [
        m[0],   m[1],  m[2],  m[3],
        m[4],   m[5],  m[6],  m[7],
        m[8],   m[9], m[10], m[11],
        m[12], m[13], m[14], m[15],
    ];

    let c = f32::cos(angle);
    let s = f32::sin(angle);
    let mv0 = m[0];
    let mv4 = m[4];
    let mv8 = m[8];

    out[0] = c*m[0]+s*m[2];
    out[4] = c*m[4]+s*m[6];
    out[8] = c*m[8]+s*m[10];

    out[2] = c*m[2]-s*mv0;
    out[6] = c*m[6]-s*mv4;
    out[10] = c*m[10]-s*mv8;

    out
}


pub fn _rotate_z(
    m: [f32; 16],
    angle: f32,
) -> [f32; 16] {
    let mut out = [
        m[0],   m[1],  m[2],  m[3],
        m[4],   m[5],  m[6],  m[7],
        m[8],   m[9], m[10], m[11],
        m[12], m[13], m[14], m[15],
    ];

    let c = f32::cos(angle);
    let s = f32::sin(angle);
    let mv0 = m[0];
    let mv4 = m[4];
    let mv8 = m[8];

    out[0] = c*m[0]-s*m[1];
    out[4] = c*m[4]-s*m[5];
    out[8] = c*m[8]-s*m[9];

    out[1]=c*m[1]+s*mv0;
    out[5]=c*m[5]+s*mv4;
    out[9]=c*m[9]+s*mv8;

    out
}
