#[macro_use] extern crate glium;
extern crate cgmath;

mod insight;
mod graphics;

use insight::Insight;

fn main() {
    let mut insight = Insight::new();
    insight.run();
}