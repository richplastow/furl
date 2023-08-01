#version 100
precision highp float;

attribute vec3 position;
uniform mat4 u_projection_matrix;
uniform mat4 u_view_matrix;
attribute vec3 color; // the color of the point
varying vec3 vColor; 

void main(void) {
    gl_Position = u_projection_matrix * u_view_matrix * vec4(position, 1.0);
    gl_PointSize = 2.;
    vColor = color;
}
