#![allow(unused_variables, dead_code, non_snake_case)]

use std::future::Future;
use futures_signals::signal::{Mutable, Signal, SignalExt};

#[derive(Debug, Clone, Copy)]
enum Primitive {
    A(Rect),
    B(Rect),
    C(Rect)
}

#[derive(Debug, Clone, Copy)]
struct Rect (f32, f32, f32, f32);

struct App {
    primitives: Vec<Box<dyn Future<Output = ()>>>
}

impl App {
    fn new() -> Self {
        Self {
            primitives: vec![]
        }
    }

    fn push_primitive<P>(&mut self, primitive: P) where P: Signal<Item = Primitive> + 'static + Send {
        let fut = primitive.for_each(|i| {
            async move {
                println!("Rerendering this primitive: {:#?}!", i)
            }
        });
        
        //self.primitives.push(Box::new(fut));
        tokio::spawn(fut);
    }

    async fn poll_all(self) {
        // Poll all futures.
    }
}

#[tokio::main]
async fn main() {
    let mut app = App::new();

    let sig = Mutable::new(Primitive::A(Rect(0.0, 0.0, 10.0, 10.0)));
    app.push_primitive(sig.signal());

    sig.set(Primitive::A(Rect(0.0, 0.0, 20.0, 10.0)));
}
