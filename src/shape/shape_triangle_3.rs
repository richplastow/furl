use super::point_3::Point3;

/// Three Point3s connected as a triangle — the simplest possible 2D Shape.
#[derive(Debug)]
pub struct ShapeTriangle3 {
    points: [Point3; 3],
}

impl ShapeTriangle3 {

    pub fn new(
        p0: Point3,
        p1: Point3,
        p2: Point3,
    ) -> Self {
        Self {
            points: [
                p0,
                p1,
                p2,
            ]
        }
    }

}
