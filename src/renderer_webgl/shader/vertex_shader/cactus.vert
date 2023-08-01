#version 100
precision highp float;

attribute float a_instance_log;
attribute float a_instance_log_rev;
attribute float a_instance_step;
attribute vec3 position;
uniform mat4 u_projection_matrix;
uniform mat4 u_view_matrix;
uniform mat4 u_model_matrix;
uniform vec4 u_quaternion;
attribute vec3 color; // the color of the point
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

void main(void) {
    vec3 pt = position;

    // Translate-x based on instance _log_, less far near top.
    pt.x = pt.x + a_instance_log * 0.7;

    // Scale based on instance _log_, smaller near top.
    pt = pt * (a_instance_log_rev * 0.02 + 0.3);

    // // Rotation angle based on timer and instance _step_.
    // // Magic angle is 0.95532 radians, or appx 54.7356 degrees.
    // // Quaternion based on axis and current angle.
    // float angle = u_quaternion.w + a_instance_step * 0.95532;

    // Rotation angle based on timer and instance _step_.
    // Magic angle is 2.399963 radians, or appx 137.50775 degrees.
    // Quaternion based on axis and current angle.
    float angle = u_quaternion.w + a_instance_step * 2.399963;
    vec4 quat = quatFromAxisAngle(u_quaternion.xyz, angle);

    // Apply quaternion to vector to rotate it.
    pt = rotateVector(quat, pt);

    // Translate-y based on instance _log_.
    pt.y = pt.y + (a_instance_log * 0.2);

    // Translate-z based on instance _step_.
    pt.z = pt.z + 2.0 - (a_instance_step * 0.05);

    gl_Position = u_projection_matrix * u_view_matrix * u_model_matrix * vec4(pt, 1.0);


    // vColor = color * (gl_Position.xyz * 0.2);
    vColor = color * 0.8 + color * (gl_Position.xyz * 0.2);

    gl_PointSize = 2.0;

}
