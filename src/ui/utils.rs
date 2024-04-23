#[derive(Debug)]
pub struct RefStr(std::rc::Rc<str>);

impl Into<std::rc::Rc<str>> for RefStr {
    fn into(self) -> std::rc::Rc<str> {
        self.0
    }
}

impl RefStr {
    pub fn new(content: &str) -> Self {
        Self(content.into())
    }
}
