use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::slice;
use std::collections::HashMap;

use cgmath::Vector4;

#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Square,
    Circle,
}

struct Index {
    position_index: usize,
    palette_index: usize,
}

struct PaletteEntry {
    shape: Shape,
    color: Vector4<f32>,
}

pub struct Object {
    pub row: u32,
    pub col: u32,
    pub shape: Shape,
    pub color: Vector4<f32>,
}

pub struct ObjectIterator<'a> {
    inner: slice::Iter<'a, Index>,
    palette: &'a Vec<PaletteEntry>,
    num_cols: u32,
}

impl<'a> Iterator for ObjectIterator<'a> {
    type Item = Object;
    fn next(&mut self) -> Option<Object> {
        self.inner.next().map(|index| {
            let palette_entry = &self.palette[index.palette_index];
            Object {
                row: index.position_index as u32 / self.num_cols,
                col: index.position_index as u32 % self.num_cols,
                shape: palette_entry.shape,
                color: palette_entry.color,
            }
        })
    }
}

pub struct Board {
    num_rows: u32,
    num_cols: u32,
    palette: Vec<PaletteEntry>,
    data: Vec<Vec<Index>>,
}

impl Board {
    pub fn open(file_name: &str) -> Board {
        let file = File::open(file_name).unwrap();
        let mut reader = BufReader::new(file);
        let mut line_buffer = String::new();

        // read board size
        let num_rows;
        let num_cols;
        {
            reader.read_line(&mut line_buffer).unwrap();
            let words: Vec<_> = line_buffer.split_whitespace().collect();
            num_rows = words[0].parse::<u32>().unwrap();
            num_cols = words[1].parse::<u32>().unwrap();
        }

        // read palette
        let mut palette_map = HashMap::new();
        let mut palette = Vec::new();
        loop {
            line_buffer.clear();
            reader.read_line(&mut line_buffer).unwrap();
            let mut words = line_buffer.split_whitespace();
            match words.next().unwrap() {
                "turn" => break,
                word => {
                    let character = word.chars().nth(0).unwrap();
                    let red   = words.next().unwrap().parse::<f32>().unwrap();
                    let green = words.next().unwrap().parse::<f32>().unwrap();
                    let blue  = words.next().unwrap().parse::<f32>().unwrap();
                    let alpha = words.next().unwrap().parse::<f32>().unwrap();
                    palette_map.insert(character, palette.len());
                    palette.push(PaletteEntry {
                        shape: Shape::Square,
                        color: Vector4::new(red, green, blue, alpha),
                    });
                }
            }
        }

        let mut data = Vec::new();
        let mut end = false;
        let mut turn = 0;
        while !end {
            data.push(Vec::new());
            let turn_fields = &mut data[turn];
            for r in 0..num_rows {
                line_buffer.clear();
                reader.read_line(&mut line_buffer).unwrap();
                let mut words = line_buffer.split_whitespace();
                for c in 0..num_cols {
                    for symbol in words.next().unwrap().chars() {
                        match palette_map.get(&symbol) {
                            Some(&pi) => turn_fields.push(Index {
                                position_index: (r * num_cols + c) as usize,
                                palette_index: pi,
                            }),
                            None => panic!("Unknown symbol: {}", symbol),
                        };
                    }
                }
            }
            loop {
                line_buffer.clear();
                reader.read_line(&mut line_buffer).unwrap();
                let mut words = line_buffer.split_whitespace();
                let keyword = words.next().unwrap();
                match keyword {
                    "end" => {
                        end = true;
                        break;
                    },
                    "turn" => break,
                    _ => {
                        // TODO
                    },
                }
            }
            turn += 1;
        }
        Board {
            data,
            palette,
            num_rows,
            num_cols,
        }
    }

    pub fn num_turns(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn num_rows(&self) -> u32 {
        self.num_rows
    }

    pub fn num_cols(&self) -> u32 {
        self.num_cols
    }

    pub fn objects(&self, turn: u32) -> ObjectIterator {
        ObjectIterator {
            inner: self.data[turn as usize].iter(),
            palette: &self.palette,
            num_cols: self.num_cols,
        }
    }
}