use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::iter;
use std::collections::HashMap;

use glium::glutin;
use glium::glutin::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::glutin::EventsLoop;
use glium::glutin::VirtualKeyCode;
use glium::backend::glutin::Display;

use cgmath::Vector4;

use graphics::Graphics;

#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Square,
    Circle,
}

#[derive(Clone, Copy, Debug)]
pub struct Field {
    pub shape: Shape,
    pub color: Vector4<f32>,
}

impl Default for Field {
    fn default() -> Field {
        Field {
            shape: Shape::Square,
            color: Vector4::new(0.0, 0.0, 0.0, 1.0),
        }
    }
}

pub struct Board {
    fields: Vec<Field>,
    num_rows: u32,
    num_cols: u32,
    num_layers: u32,
}

impl Board {
    fn new(num_rows: u32, num_cols: u32, num_layers: u32) -> Board {
        let n = (num_rows * num_cols * num_layers) as usize;
        Board {
            fields: iter::repeat(Default::default()).take(n).collect(),
            num_rows,
            num_cols,
            num_layers,
        }
    }

    pub fn clear(&mut self) {
        for field in self.fields.iter_mut() {
            *field = Default::default();
        }
    }

    pub fn num_rows(&self) -> u32 {
        self.num_rows
    }

    pub fn num_cols(&self) -> u32 {
        self.num_cols
    }

    pub fn num_layers(&self) -> u32 {
        self.num_layers
    }

    pub fn get(&self, row: u32, col: u32, layer: u32) -> &Field {
        &self.fields[self.get_index(row, col, layer)]
    }

    pub fn get_mut(&mut self, row: u32, col: u32, layer: u32) -> &mut Field {
        let index = self.get_index(row, col, layer);
        &mut self.fields[index]
    }

    fn get_index(&self, row: u32, col: u32, layer: u32) -> usize {
        if row >= self.num_rows {
            panic!("Row index out of bounds!");
        } else if col >= self.num_cols {
            panic!("Column index out of bounds!");
        } else if layer >= self.num_layers {
            panic!("Layer index out of bounds!");
        }
        ((row * self.num_cols + col) * self.num_layers + layer) as usize
    }
}

pub struct Insight {
    board: Board,
    graphics: Graphics,
    palette: HashMap<char, Vector4<f32>>,
    events_loop: EventsLoop,
    display: Display,
    reader: BufReader<File>,
    line_buffer: String,
    closing: bool,
}

impl Insight {
    pub fn open(file_name: &str) -> Insight {
        let file = File::open(file_name).unwrap();
        let mut reader = BufReader::new(file);
        let mut line_buffer = String::new();

        // read board size
        let mut board;
        {
            reader.read_line(&mut line_buffer).unwrap();
            let words: Vec<_> = line_buffer.split_whitespace().collect();
            let num_rows = words[0].parse::<u32>().unwrap();
            let num_cols = words[1].parse::<u32>().unwrap();
            let num_layers = words[2].parse::<u32>().unwrap();
            board = Board::new(num_rows, num_cols, num_layers);
        }

        // read palette
        let mut palette = HashMap::new();
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
                    palette.insert(character, Vector4::new(red, green, blue, alpha));
                }
            }
        }

        let mut end = false;
        while !end {
            for r in 0..board.num_rows() {
                line_buffer.clear();
                reader.read_line(&mut line_buffer).unwrap();
                let mut words = line_buffer.split_whitespace();
                for c in 0..board.num_cols() {
                    let mut word = words.next().unwrap().chars();
                    for l in 0..board.num_layers() {
                        let symbol = word.next().unwrap();
                        match palette.get(&symbol) {
                            Some(&color) => board.get_mut(r, c, l as u32).color = color,
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
        }

        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new()
            .with_fullscreen(events_loop.get_available_monitors().next())
            .with_title("Ants Insight");
        let context = ContextBuilder::new()
            .with_vsync(false);
        let display = Display::new(window, context, &events_loop).unwrap();

        Insight {
            board,
            graphics: Graphics::new(&display),
            palette,
            events_loop,
            display,
            reader,
            line_buffer,
            closing: false,
        }
    }

    pub fn run(&mut self) {
        println!("{:?}", self.board.get(20, 0, 0));
        while !self.closing {
            self.handle_events();
            // TODO
            self.graphics.redraw(&self.board, &self.display);
        }
    }

    fn handle_events(&mut self) {
        use self::glutin::Event::*;
        use self::glutin::WindowEvent as WE;

        let closing = &mut self.closing;
        let graphics = &mut self.graphics;
        self.events_loop.poll_events(|ev| {
            match ev {
                WindowEvent { event: wev, .. } => match wev {
                    WE::Resized(width, height) => graphics.set_view_port(width, height),
                    WE::Closed => *closing = true,
                    WE::KeyboardInput { input, .. } => {
                        if let Some(VirtualKeyCode::Q) = input.virtual_keycode {
                            *closing = true;
                        }
                    },
                    _ => (),
                },
                _ => (),
            }
        });
    }
}