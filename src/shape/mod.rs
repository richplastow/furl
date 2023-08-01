//! 2D and 3D geometries.

// mod initialize_shapes;
// pub use initialize_shapes::initialize_shapes;

mod point_3;
pub use point_3::Point3;

mod shape_axes;
pub use shape_axes::{ShapeAxes,ShapeAxesCnnxMap};

// mod shape_cube;
// pub use shape_cube::ShapeCube;

mod shape_furl;
pub use shape_furl::ShapeFurl;

mod shape_grids;
pub use shape_grids::{ShapeGrids,ShapeGridsVerticesMap};

mod shape_nubbin;
pub use shape_nubbin::{ShapeNubbin,ShapeNubbinCnnxMap};

// mod shape_tetrahedron_3;
// pub use shape_tetrahedron_3::ShapeTetrahedron3;

// mod shape_triangle_3;
// pub use shape_triangle_3::ShapeTriangle3;
