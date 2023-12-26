/// A two dimensional, cartesian vector.
pub struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn new(x: f64, y: f64) {
        Self { x, y }
    }

    /// Returns a vector in a small circle,
    fn circ(angle: f64) -> Self {
        Self {
            x: f64::cos(angle),
            y: f64::sin(angle),
        }
    }
}

pub struct AABB {
    position: Vec2,
    size: Vec2,
}
