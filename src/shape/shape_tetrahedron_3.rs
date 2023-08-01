use super::point_3::Point3;

/// Four Point3s connected as a tetrahedron — the simplest possible 3D Shape.
pub struct ShapeTetrahedron3 {
    points: [Point3; 4],
}

impl ShapeTetrahedron3 {

    pub fn new(
        p0: Point3,
        p1: Point3,
        p2: Point3,
        p3: Point3,
    ) -> Self {
        Self {
            points: [
                p0,
                p1,
                p2,
                p3,
            ]
        }
    }

}
