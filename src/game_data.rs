use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::slice;
use std::collections::HashMap;

use cgmath::Vector3;
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
    layer: u8,
}

pub struct Tile {
    pub row: u32,
    pub col: u32,
    pub layer: u8,
    pub shape: Shape,
    pub color: Vector4<f32>,
}

pub struct TileIterator<'a> {
    inner: slice::Iter<'a, Index>,
    palette: &'a Vec<PaletteEntry>,
    num_cols: u32,
}

impl<'a> Iterator for TileIterator<'a> {
    type Item = Tile;
    fn next(&mut self) -> Option<Tile> {
        self.inner.next().map(|index| {
            let palette_entry = &self.palette[index.palette_index];
            Tile {
                row: index.position_index as u32 / self.num_cols,
                col: index.position_index as u32 % self.num_cols,
                shape: palette_entry.shape,
                color: palette_entry.color,
                layer: palette_entry.layer,
            }
        })
    }
}

pub struct GameData {
    num_rows: u32,
    num_cols: u32,
    palette: Vec<PaletteEntry>,
    data: Vec<Vec<Index>>,
    background_color: Vector3<f32>,
}

impl GameData {
    pub fn load(file_name: &str) -> GameData {
        let file = File::open(file_name).unwrap();
        let mut reader = BufReader::new(file);
        let mut line_buffer = String::new();

        // read board size
        let num_rows;
        let num_cols;
        {
            line_buffer.clear();
            reader.read_line(&mut line_buffer).unwrap();
            let mut words = line_buffer.split_whitespace();
            num_rows = words.next().unwrap().parse::<u32>().unwrap();
            num_cols = words.next().unwrap().parse::<u32>().unwrap();
        }

        // read background color
        let background_color;
        {
            line_buffer.clear();
            reader.read_line(&mut line_buffer).unwrap();
            let mut words = line_buffer.split_whitespace();
            let red   = words.next().unwrap().parse::<f32>().unwrap();
            let green = words.next().unwrap().parse::<f32>().unwrap();
            let blue  = words.next().unwrap().parse::<f32>().unwrap();
            background_color = Vector3::new(red, green, blue);
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
                    let shape = match words.next().unwrap().chars().nth(0).unwrap() {
                        'c' => Shape::Circle,
                        _ => Shape::Square,
                    };
                    let red   = words.next().unwrap().parse::<f32>().unwrap();
                    let green = words.next().unwrap().parse::<f32>().unwrap();
                    let blue  = words.next().unwrap().parse::<f32>().unwrap();
                    let alpha = words.next().unwrap().parse::<f32>().unwrap();
                    let layer = words.next().unwrap().parse::<u8>().unwrap();
                    assert!(layer < 10);
                    palette_map.insert(character, palette.len());
                    palette.push(PaletteEntry {
                        shape,
                        color: Vector4::new(red, green, blue, alpha),
                        layer,
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
                if line_buffer.is_empty() {
                    break;
                }
                let mut words = line_buffer.split_whitespace();
                for c in 0..num_cols {
                    for symbol in words.next().unwrap().chars() {
                        if symbol == '.' {
                            continue;
                        }
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
                if line_buffer.is_empty() {
                    end = true;
                    break;
                }
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
        GameData {
            data,
            palette,
            num_rows,
            num_cols,
            background_color,
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

    pub fn background_color(&self) -> Vector3<f32> {
        self.background_color
    }

    pub fn tiles(&self, turn: u32) -> TileIterator {
        TileIterator {
            inner: self.data[turn as usize].iter(),
            palette: &self.palette,
            num_cols: self.num_cols,
        }
    }
}