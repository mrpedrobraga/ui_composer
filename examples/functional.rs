#![allow(unused, non_snake_case)]

struct Context {
    counter: i32
}

fn main() {
    let cx = Context {
        counter: 0
    };

    let app = App(cx);
    println!("{app}");
}

fn App(cx: Context) -> String {
    format!("\n------ My App ------\n{}\n--------------------\n",    
        Counter (cx)
    )
}

fn Counter(cx: Context) -> String {
    ListContainer(&[
        Label(format!("Counter: {}", cx.counter)),
        Button("Click me!".into())
    ])
}

fn ListContainer(items: &[String]) -> String {
    items.iter()
        .map(|i| format!("\n- {i}") )
        .collect()
}

fn Label(text: String) -> String {
    text
}

fn Button(text: String) -> String {
    format!("[ {} ]", text)
}