
```rust
use ui_composer::prelude::*;

fn App(cx: RenderContext) -> impl UIFragment {
    let mut counter = cx.state(0);

    let text = Text(counter.map(|n| format!("Counter: {}", n)));
    let button = Button("+".into(), || bc.set(bc.get() + 1));

    return all! {
        text,
        button
    }
}

fn Label( text: State<String> ) -> impl UIFragment {
    zip!(text).component(|t| {
        Primitive::Text(t)
    })
}

fn Button( text: State<String>, callback: Fn() -> () ) -> impl UIFragment {
    let aabb = AABB::Px(0, 0, 16, 16);
    
    zip!(text).component(move |t| all! {
        Primitive::Rect {
            aabb,
            color: Color::RGB(0x00, 0xA0, 0xFF),
        },
        Primitive::Text(t),
        Primitive::EventArea {
            aabb,
            click: || callback(),
            ..Default::default()
        }
    })
}
```