#version 100
precision highp float;

attribute vec3 color; // the color of the point
attribute vec3 position;
attribute vec4 ia_curves;

uniform mat4 iu_angle;
uniform mat4 iu_bulge;
uniform mat4 iu_lean;
uniform mat4 iu_rise;
uniform mat4 iu_scale;
uniform mat4 iu_tilt;
uniform mat4 u_projection_matrix;
uniform mat4 u_view_matrix;
uniform vec3 u_placement;
uniform vec4 u_quaternion_x;
uniform vec4 u_quaternion_y;
uniform vec4 u_slidermix;
uniform vec4 u_timermix;

varying vec3 vColor; 

// Create a quaternion from an axis and angle.
vec4 quatFromAxisAngle(vec3 axis, float angle) {
    float halfAngle = angle * 0.5;
    return vec4(axis.xyz * sin(halfAngle), cos(halfAngle));
}

// Apply the quaternion (q) to a vector (v).
vec3 rotateVector(vec4 q, vec3 v) {
    return v + 2.0 * cross(q.xyz, cross(q.xyz, v) + q.w * v);
}
// Rotates a point about the x-axis.
vec3 rotateAboutXAxis(vec3 pt, float angle) {
    float ha = angle * 0.5; // half angle
    vec3 quatXAxis = vec3(sin(ha), 0.0, 0.0); // xyz part of the quaternion
    return pt + 2.0 * cross(
        quatXAxis,
        cross(quatXAxis, pt) + cos(ha) * pt // cos(ha) is the w part of the quat
    );
}

// Rotates a point about the y-axis.
vec3 rotateAboutYAxis(vec3 pt, float angle) {
    float ha = angle * 0.5; // half angle
    vec3 quatYAxis = vec3(0.0, sin(ha), 0.0); // xyz part of the quaternion
    return pt + 2.0 * cross(
        quatYAxis,
        cross(quatYAxis, pt) + cos(ha) * pt // cos(ha) is the w part of the quat
    );
}

void main(void) {
    vec3 pt = position;

    // Dereference the four curves.
    float linear = ia_curves[0];
    float wow = ia_curves[1];
    float hump = ia_curves[2];
    float flutter = ia_curves[3];

    // Dereference each iu_* matrix into four vectors.
    vec4 iu_angle_a = iu_angle[0];
    vec4 iu_angle_b = iu_angle[1];
    vec4 iu_angle_sm = iu_angle[2];
    vec4 iu_angle_tm = iu_angle[3];

    vec4 iu_bulge_a = iu_bulge[0];
    vec4 iu_bulge_b = iu_bulge[1];
    vec4 iu_bulge_sm = iu_bulge[2];
    vec4 iu_bulge_tm = iu_bulge[3];

    vec4 iu_lean_a = iu_lean[0];
    vec4 iu_lean_b = iu_lean[1];
    vec4 iu_lean_sm = iu_lean[2];
    vec4 iu_lean_tm = iu_lean[3];

    vec4 iu_rise_a = iu_rise[0];
    vec4 iu_rise_b = iu_rise[1];
    vec4 iu_rise_sm = iu_rise[2];
    vec4 iu_rise_tm = iu_rise[3];

    vec4 iu_scale_a = iu_scale[0];
    vec4 iu_scale_b = iu_scale[1];
    vec4 iu_scale_sm = iu_scale[2];
    vec4 iu_scale_tm = iu_scale[3];

    vec4 iu_tilt_a = iu_tilt[0];
    vec4 iu_tilt_b = iu_tilt[1];
    vec4 iu_tilt_sm = iu_tilt[2];
    vec4 iu_tilt_tm = iu_tilt[3];

    // Scale the Prim using the linear curve, wow, sine-hump, and an invariant.
    float scale_a = linear*iu_scale_a[0] + wow*iu_scale_a[1] + hump*iu_scale_a[2] + iu_scale_a[3];
    float scale_b = linear*iu_scale_b[0] + wow*iu_scale_b[1] + hump*iu_scale_b[2] + iu_scale_b[3];
    float scale_mix = dot(u_slidermix, iu_scale_sm) + dot(u_timermix, iu_scale_tm);
    // float scale_mix = u_slidermix[0]*iu_scale_sm[0] + u_slidermix[1]*iu_scale_sm[1] + u_slidermix[2]*iu_scale_sm[2] + u_slidermix[3]*iu_scale_sm[3]
    //                 +  u_timermix[0]*iu_scale_tm[0] +  u_timermix[1]*iu_scale_tm[1] +  u_timermix[2]*iu_scale_tm[2] +  u_timermix[3]*iu_scale_tm[3];
    float scale = (1.0-scale_mix) * scale_a + scale_mix * scale_b;
    pt *= scale;

    // Tilt the Prim using the linear curve, flutter, sine-hump, and an invariant.
    float tilt_a = linear*iu_tilt_a[0] + flutter*iu_tilt_a[1] + hump*iu_tilt_a[2] + iu_tilt_a[3];
    float tilt_b = linear*iu_tilt_b[0] + flutter*iu_tilt_b[1] + hump*iu_tilt_b[2] + iu_tilt_b[3];
    float tilt_mix = dot(u_slidermix, iu_tilt_sm) + dot(u_timermix, iu_tilt_tm);
    float tilt = (1.0-tilt_mix) * tilt_a + tilt_mix * tilt_b;
    pt = rotateAboutXAxis(pt, tilt);

    // Apply bulge to the Prim using the linear curve, flutter, sine-hump and an invariant.
    // This actually just translates the Prim in the Z direction.
    float bulge_a = linear*iu_bulge_a[0] + flutter*iu_bulge_a[1] + hump*iu_bulge_a[2] + iu_bulge_a[3];
    float bulge_b = linear*iu_bulge_b[0] + flutter*iu_bulge_b[1] + hump*iu_bulge_b[2] + iu_bulge_b[3];
    float bulge_mix = dot(u_slidermix, iu_bulge_sm) + dot(u_timermix, iu_bulge_tm);
    float bulge = (1.0-bulge_mix) * bulge_a + bulge_mix * bulge_b;
    pt.z += bulge;

    // Raise the Prim upwards using the linear curve, wow, sine-hump and an invariant.
    float rise_a = linear*iu_rise_a[0] + wow*iu_rise_a[1] + hump*iu_rise_a[2] + iu_rise_a[3];
    float rise_b = linear*iu_rise_b[0] + wow*iu_rise_b[1] + hump*iu_rise_b[2] + iu_rise_b[3];
    float rise_mix = dot(u_slidermix, iu_rise_sm) + dot(u_timermix, iu_rise_tm);
    float rise = (1.0-rise_mix) * rise_a + rise_mix * rise_b;
    pt.y -= rise;

    // Prepare for making the Furl lean to one side. Nonstandard use of the vec4:
    // The first pair of uniform values control how far the Furl leans.
    // The second pair of uniform values control which direction the Furl leans.
    float lean_inclination_a = linear*iu_lean_a[0] + iu_lean_a[1];
    float lean_inclination_b = linear*iu_lean_b[0] + iu_lean_b[1];
    float lean_orientation_a = hump*iu_lean_a[2] + iu_lean_a[3];
    float lean_orientation_b = hump*iu_lean_b[2] + iu_lean_b[3];
    float lean_mix = dot(u_slidermix, iu_lean_sm) + dot(u_timermix, iu_lean_tm);
    float lean_inclination = (1.0-lean_mix) * lean_inclination_a + lean_mix * lean_inclination_b;
    float lean_orientation = (1.0-lean_mix) * lean_orientation_a + lean_mix * lean_orientation_b;

    // Sweep the Prim about the Y axis to simulate phyllotaxis.
    // Magic angle is 2.399963 radians, or appx 137.50775 degrees.
    // We avoid the Lean's initial `pt = rotateAboutYAxis(pt, lean_orientation)`
    // by adding it to this rotateAboutYAxis() call.
    float angle_a = linear*iu_angle_a[0] + wow*iu_angle_a[1] + hump*iu_angle_a[2] + iu_angle_a[3];
    float angle_b = linear*iu_angle_b[0] + wow*iu_angle_b[1] + hump*iu_angle_b[2] + iu_angle_b[3];
    float angle_mix = dot(u_slidermix, iu_angle_sm) + dot(u_timermix, iu_angle_tm);
    float angle = (1.0-angle_mix) * angle_a + angle_mix * angle_b;
    pt = rotateAboutYAxis(pt, angle + lean_orientation);

    // Finish making the Furl lean to one side.
    pt = rotateAboutXAxis(pt, lean_inclination);
    pt = rotateAboutYAxis(pt, -lean_orientation); // undoes `+ lean_orientation`

    // Apply quaternions to vector to rotate it.
    vec4 quat_x = quatFromAxisAngle(u_quaternion_x.xyz, u_quaternion_x.w);
    pt = rotateVector(quat_x, pt);
    vec4 quat_y = quatFromAxisAngle(u_quaternion_y.xyz, u_quaternion_y.w);
    pt = rotateVector(quat_y, pt);

    // Move the entire Furl to the correct position in world space.
    pt += u_placement;

    gl_Position = u_projection_matrix * u_view_matrix * vec4(pt, 1.0);

    vColor = color;

    // Define point size for WireframePreset::Dots.
    gl_PointSize = pt.z * 3.0 + 2.0;
}
