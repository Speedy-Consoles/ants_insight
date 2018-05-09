#[macro_use] extern crate glium;
extern crate cgmath;

mod insight;
mod graphics;

use insight::Insight;
use std::env;

fn main() {
    let file_name = env::args().nth(1).unwrap();
    let mut insight = Insight::open(&file_name);
    insight.run();
}