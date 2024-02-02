
```rust
fn App(cx: RCx) -> iUI {
    let mut counter = cx.state(0);

    rsx! {
        <Label>Counter: {counter}</Label>
        <Button onclick={|| counter.incr(1)}>+</Button>
    }
}

fn Label( text: St<String> ) -> iUI {
    rsx! { <Text>{text}</Text> }
}

fn Button( text: St<String>, callback: Fn() -> () ) -> iUI {
    let aabb = AABB::Px(0, 0, 16, 16);
    
    zip!(text).component(rsx! {
        <Rect aabb={aabb} color={RGB(0, 0, 255)}>
            <Text>{text}</Text>
            <EventArea size={SizeHint::Expand} click={|| callback()}/>
        </Rect>
    })
}
```