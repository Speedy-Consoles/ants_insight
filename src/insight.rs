use glium::glutin;
use glium::glutin::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::glutin::EventsLoop;
use glium::glutin::VirtualKeyCode;
use glium::backend::glutin::Display;

use graphics::Graphics;
use board::Board;

pub struct Insight {
    board: Board,
    graphics: Graphics,
    events_loop: EventsLoop,
    display: Display,
    closing: bool,
}

impl Insight {
    pub fn open(file_name: &str) -> Insight {
        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new()
            .with_fullscreen(events_loop.get_available_monitors().next())
            .with_title("Ants Insight");
        let context = ContextBuilder::new()
            .with_vsync(false);
        let display = Display::new(window, context, &events_loop).unwrap();

        let board = Board::open(file_name);
        let graphics = Graphics::new(board.num_rows(), board.num_cols(), &display);
        Insight {
            board,
            graphics,
            events_loop,
            display,
            closing: false,
        }
    }

    pub fn run(&mut self) {
        while !self.closing {
            self.handle_events();
            // TODO
            self.graphics.draw_turn(&self.board, 0, &self.display);
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