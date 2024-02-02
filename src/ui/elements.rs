use super::geometry::UIVector;

pub trait Fragmennt {
    fn primitives() -> [Primitive];
}

pub enum Primitive {
    Text(String),
    Point(UIVector),
}
