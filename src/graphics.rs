use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use glium::VertexBuffer;
use glium::IndexBuffer;
use glium::backend::glutin::Display;
use glium::index::PrimitiveType;
use glium::Surface;
use glium::program::Program;
use glium::Frame;

use cgmath::Matrix4;
use cgmath::SquareMatrix;
use cgmath::Vector4;

use insight::Board;

struct VertexObject {
    vertex_buffer: VertexBuffer<MyVertex>,
    index_buffer: IndexBuffer<u32>,
}

#[derive(Copy, Clone)]
struct MyVertex {
    position: [f32; 3],
}

implement_vertex!(MyVertex, position);

pub struct Graphics {
    square: VertexObject,
    circle: VertexObject,
    program: Program,
    width: u32,
    height: u32,
    projection_matrix: Matrix4<f32>,
}

impl Graphics {
    pub fn new(display: &Display) -> Graphics {
        let program = Program::from_source(
            display,
            &Self::load_shader_source("shader_src/default.vert"),
            &Self::load_shader_source("shader_src/default.frag"),
            None,
        ).unwrap();

        Graphics {
            square: Self::build_square(display),
            circle: Self::build_circle(display),
            program,
            width: 0,
            height: 0,
            projection_matrix: Matrix4::identity(),
        }
    }

    pub fn redraw(&mut self, board: &Board, display: &Display) {
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
        self.projection_matrix = Matrix4::new(
            x_scaling,  0.0,       0.0, 0.0,
            0.0,        y_scaling, 0.0, 0.0,
            0.0,        0.0,       1.0, 0.0,
            x_offset,   y_offset,  0.0, 1.0f32,
        );

        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);

        for r in 0..board.num_rows() {
            let y = board.num_rows() - r - 1;
            for c in 0..board.num_cols() {
                let x = c;
                for l in 0..board.num_layers() {
                    let field = board.get(r, c, l);
                    self.draw_square(x, y, field.color, &mut frame);
                }
            }
        }

        frame.finish().unwrap();
    }

    pub fn set_view_port(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn draw_square(&self, x: u32, y: u32, color: Vector4<f32>, frame: &mut Frame) {
        let translation_matrix = Matrix4::new(
            1.0,      0.0,      0.0, 0.0,
            0.0,      1.0,      0.0, 0.0,
            0.0,      0.0,      1.0, 0.0,
            x as f32, y as f32, 0.0, 1.0f32,
        );

        let transformation_matrix_uniform: [[f32; 4]; 4]
            = (self.projection_matrix * translation_matrix).into();
        let color_uniform: [f32; 4] = color.into();
        let uniforms = uniform! {
            trafo_matrix: transformation_matrix_uniform,
            color: color_uniform,
        };

        // draw parameters
        let draw_parameters = Default::default();
        frame.draw(
            &self.square.vertex_buffer,
            &self.square.index_buffer,
            &self.program,
            &uniforms,
            &draw_parameters,
        ).unwrap();
    }

    fn build_square(display: &Display) -> VertexObject {
        let vertex_data = &[
            MyVertex { position: [0.0, 0.0, 0.0] },
            MyVertex { position: [1.0, 0.0, 0.0] },
            MyVertex { position: [1.0, 1.0, 0.0] },
            MyVertex { position: [0.0, 1.0, 0.0] },
        ];
        let vertex_buffer = VertexBuffer::new(display, vertex_data).unwrap();

        let index_data = &[0, 1, 2, 0, 2, 3];
        let index_buffer = IndexBuffer::new(
            display,
            PrimitiveType::TrianglesList,
            index_data
        ).unwrap();

        VertexObject {
            vertex_buffer,
            index_buffer,
        }
    }

    fn build_circle(display: &Display) -> VertexObject {
        let vertex_data = &[
            MyVertex { position: [0.0, 0.0, 0.0] },
            MyVertex { position: [1.0, 0.0, 0.0] },
            MyVertex { position: [1.0, 1.0, 0.0] },
            MyVertex { position: [0.0, 1.0, 0.0] },
        ];
        let vertex_buffer = VertexBuffer::new(display, vertex_data).unwrap();

        let index_data = &[0, 1, 2, 0, 2, 3];
        let index_buffer = IndexBuffer::new(
            display,
            PrimitiveType::TrianglesList,
            index_data
        ).unwrap();

        VertexObject {
            vertex_buffer,
            index_buffer,
        }
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