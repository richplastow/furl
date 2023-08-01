#version 100
precision highp float;

attribute float a_instance_step;
attribute float a_position_x;
attribute float a_position_y;

uniform float u_pointsize;

void main() {
    gl_Position = vec4(
        a_position_x + a_instance_step,
        a_position_y - a_instance_step,
        0.0,
        1.0
    );
    gl_PointSize = u_pointsize;
}
