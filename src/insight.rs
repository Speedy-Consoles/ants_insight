use std::env;
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

#[derive(Clone, Copy)]
pub struct Field {
    pub color: Vector4<f32>,
    pub circle_color: Vector4<f32>,
}

impl Default for Field {
    fn default() -> Field {
        Field {
            color: Vector4::new(0.0, 0.0, 0.0, 1.0),
            circle_color: Vector4::new(0.0, 0.0, 0.0, 0.0),
        }
    }
}

pub struct Board {
    fields: Vec<Field>,
    num_rows: u32,
    num_cols: u32,
}

impl Board {
    fn new(num_rows: u32, num_cols: u32) -> Board {
        let n = (num_rows * num_cols) as usize;
        Board {
            fields: iter::repeat(Default::default()).take(n).collect(),
            num_rows,
            num_cols,
        }
    }

    fn clear(&mut self) {
        for field in self.fields.iter_mut() {
            *field = Default::default();
        }
    }

    fn get_mut(&mut self, row: u32, col: u32) -> &mut Field {
        &mut self.fields[(row * self.num_cols + col) as usize]
    }

    pub fn num_rows(&self) -> u32 {
        self.num_rows
    }

    pub fn num_cols(&self) -> u32 {
        self.num_cols
    }

    pub fn get(&self, row: u32, col: u32) -> &Field {
        &self.fields[(row * self.num_cols + col) as usize]
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
    pub fn new() -> Insight {
        let file_name = env::args().nth(1).unwrap();
        let file = File::open(file_name).unwrap();
        let mut reader = BufReader::new(file);
        let mut line_buffer = String::new();

        // read board size
        let board;
        {
            reader.read_line(&mut line_buffer).unwrap();
            let words: Vec<_> = line_buffer.split_whitespace().collect();
            let num_rows = words[0].parse::<u32>().unwrap();
            let num_cols = words[1].parse::<u32>().unwrap();
            board = Board::new(num_rows, num_cols);
        }

        // read palette
        let mut palette = HashMap::new();
        loop {
            line_buffer.clear();
            reader.read_line(&mut line_buffer).unwrap();
            if line_buffer.trim() == "-" {
                break;
            }
            let words: Vec<_> = line_buffer.split_whitespace().collect();
            let character = words[0].chars().next().unwrap();
            let red   = words[1].parse::<f32>().unwrap();
            let green = words[2].parse::<f32>().unwrap();
            let blue  = words[3].parse::<f32>().unwrap();
            let alpha = words[4].parse::<f32>().unwrap();
            palette.insert(character, Vector4::new(red, green, blue, alpha));
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
        while !self.closing {
            self.handle_events();
            self.read_turn();
            self.graphics.redraw(&self.board, &self.display);
        }
    }

    fn read_turn(&mut self) {
        for r in 0..self.board.num_rows() {
            self.line_buffer.clear();
            self.reader.read_line(&mut self.line_buffer).unwrap();
            for (c, s) in self.line_buffer.split_whitespace().enumerate() {
                let character = s.chars().next().unwrap();
                match self.palette.get(&character) {
                    Some(&color) => self.board.get_mut(r, c as u32).color = color,
                    None => panic!("Unknown symbol: {}", character),
                };
            }
        }
        loop {
            self.line_buffer.clear();
            self.reader.read_line(&mut self.line_buffer).unwrap();
            if self.line_buffer.is_empty() || self.line_buffer.trim() == "-" {
                break;
            }
            // TODO
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