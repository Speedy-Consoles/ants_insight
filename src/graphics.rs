use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use glium::VertexBuffer;
use glium::index::NoIndices;
use glium::backend::glutin::Display;
use glium::index::PrimitiveType;
use glium::Surface;
use glium::program::Program;

use cgmath::Matrix4;
use cgmath::SquareMatrix;

use board::Board;

#[derive(Copy, Clone)]
struct MyVertex {
    position: [f32; 2],
    color: [f32; 4],
}

implement_vertex!(MyVertex, position, color);

pub struct Graphics {
    vertex_data: Vec<MyVertex>,
    program: Program,
    width: u32,
    height: u32,
    transformation_matrix: Matrix4<f32>,
}

impl Graphics {
    pub fn new(num_rows: u32, num_cols: u32, display: &Display) -> Graphics {
        let program = Program::from_source(
            display,
            &Self::load_shader_source("shader_src/default.vert"),
            &Self::load_shader_source("shader_src/default.frag"),
            Some(&Self::load_shader_source("shader_src/default.geo")),
        ).unwrap();

        Graphics {
            vertex_data: Vec::new(),
            program,
            width: 0,
            height: 0,
            transformation_matrix: Matrix4::identity(),
        }
    }

    pub fn draw_turn(&mut self, board: &Board, turn: u32, display: &Display) {
        let board_width = board.num_cols() as f32;
        let board_height = board.num_rows() as f32;
        let board_ratio = if board.num_rows() != 0 {
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

        self.vertex_data.clear();

        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);

        let num_rows = board.num_rows();
        for object in board.objects(turn) {
            let x = object.col as f32;
            let y = (num_rows - object.row - 1) as f32;
            self.vertex_data.push(MyVertex {
                position: [x, y],
                color: object.color.into(),
            });
        }
        let vertex_buffer = VertexBuffer::new(display, &self.vertex_data).unwrap();

        let transformation_matrix_uniform: [[f32; 4]; 4] = self.transformation_matrix.into();
        let uniforms = uniform! {
            trafo_matrix: transformation_matrix_uniform,
        };

        // draw parameters
        let draw_parameters = Default::default();
        frame.draw(
            &vertex_buffer,
            &NoIndices(PrimitiveType::Points),
            &self.program,
            &uniforms,
            &draw_parameters,
        ).unwrap();

        frame.finish().unwrap();
    }

    pub fn set_view_port(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
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