#![allow(unused, non_snake_case)]
use std::{ops::Range, pin::{pin, Pin}, task::{Context, Poll}, time::Duration};

use futures_signals::signal::{Mutable, SignalExt};
use futures::{stream::{BoxStream, Stream}, Future, StreamExt};
use tokio::time::{self, Sleep};

#[tokio::main]
async fn main () {
    let w = Rerender::Branch(vec![  ]);

    dbg!(w);
}

#[derive(Debug)]
enum Rerender {
    Branch(Vec<Rerender>),
    Leaf(i32)
}