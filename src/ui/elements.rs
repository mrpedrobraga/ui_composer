use super::geometry::UIVector;

pub trait Fragment {
    fn primitives<'a>(&self) -> Vec<Primitive>;
}

#[derive(Clone)]
pub enum Primitive {
    Text(String),
    Point(UIVector),
}

impl Fragment for Primitive {
    fn primitives<'a>(&self) -> Vec<Primitive> {
        Vec::from([self.clone()])
    }
}

impl Fragment for i32 {
    fn primitives<'a>(&self) -> Vec<Primitive> {
        vec![Primitive::Text(format!("{}", self))]
    }
}
