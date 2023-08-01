//! Utilities for creating projection matrices (useful for cameras).

const PI: f32 = std::f32::consts::PI;

/// Same as orthographic(), but more succinct.
/// @TODO benchmark and just keep the fastest.
pub fn ortho(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
) -> [f32; 16] {
    [
        2. / (right - left), 0.,                  0.,                 0.,
        0.,                  2. / (top - bottom), 0.,                 0.,
        0.,                  0.,                  2. / (near - far),  0.,

        (left + right) / (left - right),
        (bottom + top) / (bottom - top),
        (near + far) / (near - far),
        1.,
    ]
}

/// Same as ortho(), but more verbose.
/// @TODO benchmark and just keep the fastest.
pub fn orthographic(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
) -> [f32; 16] {
    let c0r0 = 2. / (right - left);
    let c0r1 = 0.;
    let c0r2 = 0.;
    let c0r3 = 0.;

    let c1r0 = 0.;
    let c1r1 = 2. / (top - bottom);
    let c1r2 = 0.;
    let c1r3 = 0.;

    let c2r0 = 0.;
    let c2r1 = 0.;
    let c2r2 = -2. / (far - near);
    let c2r3 = 0.;

    let c3r0 = -(right + left) / (right - left);
    let c3r1 = -(top + bottom) / (top - bottom);
    let c3r2 = -(far + near) / (far - near);
    let c3r3 = 1.;

    [
        c0r0, c0r1, c0r2, c0r3,
        c1r0, c1r1, c1r2, c1r3,
        c2r0, c2r1, c2r2, c2r3,
        c3r0, c3r1, c3r2, c3r3,
    ]
}

/// Generates a hard-to-read matrix, but a standard 4x4 matrix nonetheless.
/// Typical values:
/// - `angle` usually between 90° (extra wide) and 30° (quite zoomed in)
/// - `aspect` 1.333 for fullscreen 1280/960, or 1.778 for 1080P
/// - `near` 0.1 will clip anything less than 1cm from the camera
/// - `far` 100.0 will clip anything more than 100m from the camera
/// 
/// <http://www.opengl-tutorial.org/beginners-tutorials/tutorial-3-matrices/#the-projection-matrix>
pub fn perspective(
    angle: f32,  // vertical Field of View, in radians: the amount of ‘zoom’
    aspect: f32, // aspect ratio — depends on the size of the <CANVAS> element
    near: f32,   // near clipping plane — as big as possible to avoid precision issues
    far: f32    // far clipping plane — as little as possible
) -> [f32; 16] {
    let ang = f32::tan((angle*0.5)*PI/180.); // angle*.5
    return [
        0.5/ang,  0.,              0.,                         0.,
        0.,      -0.5*aspect/ang,  0.,                         0., // added -ve to c1r1
        0.,       0.,             -(far+near)/(far-near),     -1.,
        0.,       0.,              (-2.*far*near)/(far-near),  0.,
    ];
}
