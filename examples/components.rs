#![allow(unused, non_snake_case)]

use std::pin::{pin, Pin};

use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
};

#[derive(Debug, Clone)]
enum Primitive {
    Text(AABB, String),
}

struct Context {
    layout_hints: Mutable<LayoutHints>,
    dummy_text: Mutable<String>,
}

#[derive(Debug, Clone, Copy)]
struct AABB(f32, f32, f32, f32);

#[derive(Debug, Clone, Copy)]
struct LayoutHints {
    aabb: AABB,
}

struct Rerender {
    range: std::ops::Range<usize>,
    command: RerenderInner,
}

enum RerenderInner {
    Nested(Box<dyn Signal<Item = Rerender> + Unpin>),
    Primitive(Primitive),
}

#[tokio::main]
async fn main() {
    // Setting up global states the app can listen to
    let lhints = Mutable::new(LayoutHints {
        aabb: AABB(0.0, 0.0, 10.0, 10.0),
    });

    let dummy_text = Mutable::new(format!("Lorem ipsum dolor sit amet!"));

    // Packaging them in one neat box.
    let cx = Context {
        layout_hints: lhints.clone(),
        dummy_text: dummy_text.clone(),
    };

    // Call the app with the Context
    let app = App(cx);

    // ...

    // Changing these values should rerender the respective primitives
    lhints.set(LayoutHints {
        aabb: AABB(10.0, 10.0, 20.0, 20.0),
        ..lhints.get()
    });

    dummy_text.set(format!("This is a new text that has been set!"));
}

fn App(cx: Context) -> impl Signal<Item = Rerender> {
    cx.layout_hints.signal().map(move |lh| Rerender {
        range: 0..2,
        command: RerenderInner::Nested(Box::new(Text(lh.aabb, cx.dummy_text.signal_cloned()))),
    })
}

fn Text<S>(aabb: AABB, text_signal: S) -> impl Signal<Item = Rerender>
where
    S: Signal<Item = String>,
{
    let r = text_signal.map(move |s| Rerender {
        range: 1..2,
        command: RerenderInner::Primitive(Primitive::Text(aabb, s)),
    });
    r
}
