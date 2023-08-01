//! Utilities for calculating the dot product of matrices.

pub fn dot(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
    // Slice the second matrix up into rows.
    let row0 = [ b[ 0], b[ 1], b[ 2], b[ 3] ];
    let row1 = [ b[ 4], b[ 5], b[ 6], b[ 7] ];
    let row2 = [ b[ 8], b[ 9], b[10], b[11] ];
    let row3 = [ b[12], b[13], b[14], b[15] ];

    // Multiply each row by the first matrix.
    let out0 = dot_matrix_and_point(a, row0);
    let out1 = dot_matrix_and_point(a, row1);
    let out2 = dot_matrix_and_point(a, row2);
    let out3 = dot_matrix_and_point(a, row3);

    // Turn the out rows back into a single matrix
    return [
        out0[0], out0[1], out0[2], out0[3],
        out1[0], out1[1], out1[2], out1[3],
        out2[0], out2[1], out2[2], out2[3],
        out3[0], out3[1], out3[2], out3[3]
    ];
}

/// Used by dot().
fn dot_matrix_and_point(m: [f32; 16], p: [f32; 4]) -> [f32; 4] {
    // Give a simple variable name to each part of the matrix, a column and row number
    let c0r0 = m[ 0]; let c1r0 = m[ 1]; let c2r0 = m[ 2]; let c3r0 = m[ 3];
    let c0r1 = m[ 4]; let c1r1 = m[ 5]; let c2r1 = m[ 6]; let c3r1 = m[ 7];
    let c0r2 = m[ 8]; let c1r2 = m[ 9]; let c2r2 = m[10]; let c3r2 = m[11];
    let c0r3 = m[12]; let c1r3 = m[13]; let c2r3 = m[14]; let c3r3 = m[15];
  
    // Now set some simple names for the point.
    let x = p[0];
    let y = p[1];
    let z = p[2];
    let w = p[3];
  
    // Multiply the point against each part of the 1st column, then add together
    let out_x = (x * c0r0) + (y * c0r1) + (z * c0r2) + (w * c0r3);
  
    // Multiply the point against each part of the 2nd column, then add together
    let out_y = (x * c1r0) + (y * c1r1) + (z * c1r2) + (w * c1r3);
  
    // Multiply the point against each part of the 3rd column, then add together
    let out_z = (x * c2r0) + (y * c2r1) + (z * c2r2) + (w * c2r3);
  
    // Multiply the point against each part of the 4th column, then add together
    let out_w = (x * c3r0) + (y * c3r1) + (z * c3r2) + (w * c3r3);
  
    return [ out_x, out_y, out_z, out_w ];
}
