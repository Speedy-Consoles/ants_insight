use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use glium::VertexBuffer;
use glium::IndexBuffer;
use glium::index::NoIndices;
use glium::backend::glutin::Display;
use glium::index::PrimitiveType;
use glium::Surface;
use glium::program::Program;
use glium::DrawParameters;
use glium::Blend;
use glium::Frame;

use cgmath::Matrix4;
use cgmath::SquareMatrix;

use game_data::GameData;
use game_data::Shape;

#[derive(Copy, Clone)]
struct MyTile {
    position: [f32; 3],
    color: [f32; 4],
    radius2: f32,
}

#[derive(Copy, Clone)]
struct MyLine {
    start: [f32; 3],
    end: [f32; 3],
    color: [f32; 4],
}

#[derive(Copy, Clone)]
struct MyVertex {
    position: [f32; 3],
}

implement_vertex!(MyTile, position, color, radius2);
implement_vertex!(MyLine, start, end, color);
implement_vertex!(MyVertex, position);

pub struct Graphics {
    tile_vertex_data: Vec<MyTile>,
    line_vertex_data: Vec<MyLine>,
    background_vertex_buffer: VertexBuffer<MyVertex>,
    background_index_buffer: IndexBuffer<u32>,
    tiles_program: Program,
    lines_program: Program,
    background_program: Program,
    width: u32,
    height: u32,
    transformation_matrix: Matrix4<f32>,
    layer_switches: [bool; 10],
}

impl Graphics {
    pub fn new(num_rows: u32, num_cols: u32, display: &Display) -> Graphics {
        let background_vss = Self::load_shader_source("shader_src/background.vert");
        let background_fss = Self::load_shader_source("shader_src/background.frag");
        let tiles_vss = Self::load_shader_source("shader_src/tiles.vert");
        let tiles_gss = Self::load_shader_source("shader_src/tiles.geom");
        let tiles_fss = Self::load_shader_source("shader_src/tiles.frag");
        let lines_vss = Self::load_shader_source("shader_src/lines.vert");
        let lines_gss = Self::load_shader_source("shader_src/lines.geom");
        let lines_fss = Self::load_shader_source("shader_src/lines.frag");

        let tiles_program = Program::from_source(
            display,
            &tiles_vss,
            &tiles_fss,
            Some(&tiles_gss),
        ).unwrap();

        let lines_program = Program::from_source(
            display,
            &lines_vss,
            &lines_fss,
            Some(&lines_gss),
        ).unwrap();

        let background_program = Program::from_source(
            display,
            &background_vss,
            &background_fss,
            None,
        ).unwrap();

        let height = num_rows as f32;
        let width = num_cols as f32;
        let background_vertex_data = [
            MyVertex { position: [0.0,   0.0,    0.0] },
            MyVertex { position: [width, 0.0,    0.0] },
            MyVertex { position: [width, height, 0.0] },
            MyVertex { position: [0.0,   height, 0.0] },
        ];
        let background_vertex_buffer = VertexBuffer::new(display, &background_vertex_data).unwrap();

        let background_index_data = [0, 1, 2, 0, 2, 3];
        let background_index_buffer = IndexBuffer::new(
            display,
            PrimitiveType::TriangleStrip,
            &background_index_data,
        ).unwrap();

        Graphics {
            tile_vertex_data: Vec::new(),
            line_vertex_data: Vec::new(),
            background_vertex_buffer,
            background_index_buffer,
            tiles_program: tiles_program,
            lines_program: lines_program,
            background_program,
            width: 0,
            height: 0,
            transformation_matrix: Matrix4::identity(),
            layer_switches: [true; 10],
        }
    }

    pub fn draw_turn(&mut self, game_data: &GameData, turn: u32, display: &Display) {
        let board_width = game_data.num_cols() as f32;
        let board_height = game_data.num_rows() as f32;
        let board_ratio = if game_data.num_rows() != 0 {
            board_width / board_height
        } else {
            0.0
        };
        let screen_ratio = if self.height != 0 {
            self.width as f32 / self.height as f32
        } else {
            0.0
        };
        let x_offset;
        let y_offset;
        let x_scaling;
        let y_scaling;
        if screen_ratio > board_ratio {
            x_scaling = 2.0 / board_height / screen_ratio;
            y_scaling = 2.0 / board_height;
            x_offset = -board_ratio / screen_ratio;
            y_offset = -1.0;
        } else {
            x_scaling = 2.0 / board_width;
            y_scaling = 2.0 / board_width * screen_ratio;
            x_offset = -1.0;
            y_offset = -screen_ratio / board_ratio;
        }
        self.transformation_matrix = Matrix4::new(
            x_scaling,  0.0,       0.0, 0.0,
            0.0,        y_scaling, 0.0, 0.0,
            0.0,        0.0,       1.0, 0.0,
            x_offset,   y_offset,  0.0, 1.0f32,
        );

        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);

        self.draw_background(game_data, &mut frame);
        self.draw_tiles(game_data, turn, &mut frame, &display);
        self.draw_lines(game_data, turn, &mut frame, &display);

        frame.finish().unwrap();
    }

    pub fn toggle_layer(&mut self, layer: u32) {
        self.layer_switches[layer as usize] ^= true;
    }

    pub fn set_view_port(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn draw_background(&mut self, game_data: &GameData, frame: &mut Frame) {
        let transformation_matrix_uniform: [[f32; 4]; 4] = self.transformation_matrix.into();
        let background_color_uniform: [f32; 3] = game_data.background_color().into();
        let background_uniforms = uniform! {
            trafo_matrix: transformation_matrix_uniform,
            background_color: background_color_uniform,
        };

        // draw background
        frame.draw(
            &self.background_vertex_buffer,
            &self.background_index_buffer,
            &self.background_program,
            &background_uniforms,
            &Default::default(),
        ).unwrap();
    }

    fn draw_tiles(&mut self, game_data: &GameData, turn: u32, frame: &mut Frame,
                  display: &Display) {
        self.tile_vertex_data.clear();

        let num_rows = game_data.num_rows();
        for tile in game_data.tiles(turn) {
            if !self.layer_switches[tile.layer as usize] {
                continue;
            }
            let x = tile.col as f32;
            let y = (num_rows - tile.row - 1) as f32;
            let z = 1.0 - tile.layer as f32 / 100.0;
            let radius2 = match tile.shape {
                Shape::Square => 1.0,
                Shape::Circle => 0.25,
            };
            self.tile_vertex_data.push(MyTile {
                position: [x, y, z],
                color: tile.color.into(),
                radius2,
            });
        }
        let vertex_buffer = VertexBuffer::new(display, &self.tile_vertex_data).unwrap();

        let transformation_matrix_uniform: [[f32; 4]; 4] = self.transformation_matrix.into();
        let uniforms = uniform! {
            trafo_matrix: transformation_matrix_uniform,
        };

        // draw tiles
        let draw_parameters = DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        };
        frame.draw(
            &vertex_buffer,
            &NoIndices(PrimitiveType::Points),
            &self.tiles_program,
            &uniforms,
            &draw_parameters,
        ).unwrap();
    }

    fn draw_lines(&mut self, game_data: &GameData, turn: u32, frame: &mut Frame,
                  display: &Display) {
        self.line_vertex_data.clear();

        let num_rows = game_data.num_rows();
        for line in game_data.lines(turn) {
            if !self.layer_switches[line.layer as usize] {
                continue;
            }
            let x1 = line.c1 as f32;
            let y1 = (num_rows - line.r1 - 1) as f32;
            let x2 = line.c2 as f32;
            let y2 = (num_rows - line.r2 - 1) as f32;
            let z = 1.0 - line.layer as f32 / 100.0;
            self.line_vertex_data.push(MyLine {
                start: [x1, y1, z],
                end: [x2, y2, z],
                color: line.color.into(),
            });
        }
        let vertex_buffer = VertexBuffer::new(display, &self.line_vertex_data).unwrap();

        let transformation_matrix_uniform: [[f32; 4]; 4] = self.transformation_matrix.into();
        let uniforms = uniform! {
            trafo_matrix: transformation_matrix_uniform,
        };

        // draw tiles
        let draw_parameters = DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        };
        frame.draw(
            &vertex_buffer,
            &NoIndices(PrimitiveType::Points),
            &self.lines_program,
            &uniforms,
            &draw_parameters,
        ).unwrap();
    }

    fn load_shader_source(file_name: &str) -> String {
        let file = File::open(file_name).expect("Could not load shader source!");
        let mut buffer_reader = BufReader::new(file);
        let mut shader_source = String::new();
        buffer_reader.read_to_string(&mut shader_source)
            .expect("Error while reading shader source!");
        shader_source
    }
}