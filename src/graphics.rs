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

use cgmath::Matrix3;
use cgmath::SquareMatrix;

use board::Board;

#[derive(Copy, Clone)]
struct MyTile {
    position: [f32; 2],
    color: [f32; 4],
}

#[derive(Copy, Clone)]
struct MyVertex {
    position: [f32; 2],
}

implement_vertex!(MyTile, position, color);
implement_vertex!(MyVertex, position);

pub struct Graphics {
    vertex_data: Vec<MyTile>,
    background_vertex_buffer: VertexBuffer<MyVertex>,
    background_index_buffer: IndexBuffer<u32>,
    program: Program,
    background_program: Program,
    width: u32,
    height: u32,
    transformation_matrix: Matrix3<f32>,
}

impl Graphics {
    pub fn new(num_rows: u32, num_cols: u32, display: &Display) -> Graphics {
        let background_vss = Self::load_shader_source("shader_src/background.vert");
        let background_fss = Self::load_shader_source("shader_src/background.frag");
        let tiles_vss = Self::load_shader_source("shader_src/tiles.vert");
        let tiles_gss = Self::load_shader_source("shader_src/tiles.geom");
        let tiles_fss = Self::load_shader_source("shader_src/tiles.frag");

        let program = Program::from_source(
            display,
            &tiles_vss,
            &tiles_fss,
            Some(&tiles_gss),
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
            MyVertex { position: [0.0,   0.0   ] },
            MyVertex { position: [width, 0.0   ] },
            MyVertex { position: [width, height] },
            MyVertex { position: [0.0,   height] },
        ];
        let background_vertex_buffer = VertexBuffer::new(display, &background_vertex_data).unwrap();

        let background_index_data = [0, 1, 2, 0, 2, 3];
        let background_index_buffer = IndexBuffer::new(
            display,
            PrimitiveType::TriangleStrip,
            &background_index_data,
        ).unwrap();

        Graphics {
            vertex_data: Vec::new(),
            background_vertex_buffer,
            background_index_buffer,
            program,
            background_program,
            width: 0,
            height: 0,
            transformation_matrix: Matrix3::identity(),
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
        self.transformation_matrix = Matrix3::new(
            x_scaling,  0.0,       0.0,
            0.0,        y_scaling, 0.0,
            x_offset,   y_offset,  1.0f32,
        );

        self.vertex_data.clear();

        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);

        let num_rows = board.num_rows();
        for object in board.tiles(turn) {
            let x = object.col as f32;
            let y = (num_rows - object.row - 1) as f32;
            self.vertex_data.push(MyTile {
                position: [x, y],
                color: object.color.into(),
            });
        }
        let vertex_buffer = VertexBuffer::new(display, &self.vertex_data).unwrap();

        let transformation_matrix_uniform: [[f32; 3]; 3] = self.transformation_matrix.into();
        let uniforms = uniform! {
            trafo_matrix: transformation_matrix_uniform,
        };

        let background_color_uniform: [f32; 3] = board.background_color().into();
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

        // draw tiles
        frame.draw(
            &vertex_buffer,
            &NoIndices(PrimitiveType::Points),
            &self.program,
            &uniforms,
            &Default::default(),
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