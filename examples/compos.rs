#![allow(unused)]
#![allow(non_snake_case)]
use ui_composer::ui::{elements::{Fragment, Primitive}, render::RenderContext};

fn main() {
    render(App);
}

fn render<F, C>(A: F) where F: Fn(RenderContext) -> C, C: Fragment { }

fn App(cx: RenderContext) -> impl Fragment {
    Primitive::Text("Hello, World!".into())
}
