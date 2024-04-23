#![allow(unused_variables, dead_code, non_snake_case)]

use std::rc::Rc;

#[derive(Clone)]
struct UIRange {
    range: std::ops::Range<usize>,
    replacer: Option<Rc<dyn Fn() -> Box<dyn Iterator<Item = Primitive>>>>
}

#[derive(Debug, Clone)]
enum Primitive {
    Rect(i32),
    Text(String),
}

impl UIRange {
    fn new(range: std::ops::Range<usize>) -> Self {
        Self { range, replacer: None }
    }

    fn with_replacer<F>(mut self, replacer: F) -> Self
        where
            F: Fn() -> Box<dyn Iterator<Item = Primitive>> + 'static
        {
        self.replacer = Some(Rc::new(replacer));
        self
    }

    fn range(&self) -> std::ops::Range<usize> {
        self.range.clone()
    }
}

struct App {
    pub ranges: Vec<UIRange>,
    pub primitives: Vec<Primitive>,
}

impl App {
    fn splice_primitives(&mut self, range: UIRange) {
        let mut prims = range.replacer.expect("No replacer on UI range???")();

        for idx in range.range {
            self.primitives[idx] = prims.next().expect("Too few replacements for spliced primitives.")
        }
    }
    
    fn splice_ui_ranges<I>(&mut self, range_position: usize, range_end: usize, mut ranges: I) where I: Iterator<Item = UIRange> {
        for (idx, ui_range) in self.ranges.iter_mut().skip(range_position).enumerate() {
            *ui_range = ranges.next().expect("Too few replacements for spliced ranges.");
            if ui_range.range.end > range_end { break; }
        }
    }
}

fn main() {
    let app = UIRange::new(0..4);
    let lc1 = UIRange::new(0..4);
    //let txa = UIRange::new(0..1);  // <-- Non-reactive
    let lc2 = UIRange::new(1..4)
        .with_replacer(|| Box::new([
            Primitive::Text("Enter your message...".into()),
            Primitive::Rect(1),
            Primitive::Text("*Send*".into())
        ].into_iter()));
    //let txe = UIRange::new(1..2);  // <-- Non-reactive
    let btn = UIRange::new(2..4)
        .with_replacer(|| Box::new([
            Primitive::Rect(2),
            Primitive::Text("Sent!".into())
        ].into_iter()));
    //let rec = UIRange::new(2..3);  // <-- Non-reactive
    //let txt = UIRange::new(3..4);  // <-- Non-reactive

    let mut ui_app = App {
        ranges: vec![app, lc1, lc2, btn],
        primitives: vec![
            Primitive::Text("Messages, etc".into()),
            Primitive::Text("Enter your message...".into()),
            Primitive::Rect(0),
            Primitive::Text("Send".into()),
        ],
    };

    // Original Primitives
    dbg!(&ui_app.primitives);
    
    // Imagine, Idk, the screen resized, and LC2
    // detects a state change!!!
    
    // Splice primitives under LC2
    ui_app.splice_primitives(ui_app.ranges[2].clone());
    // Splice primitives under BTN
    ui_app.splice_primitives(ui_app.ranges[3].clone());

    // Perhaps the spliced Primitives will be issued as VecDiffs
    // on a MutableVec, which can then be optimized and sent to the
    // GPU buffers.

    // Modified Primitives
    dbg!(&ui_app.primitives);
}
