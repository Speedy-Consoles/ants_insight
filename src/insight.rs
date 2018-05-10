use glium::glutin;
use glium::glutin::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::glutin::EventsLoop;
use glium::glutin::VirtualKeyCode;
use glium::glutin::ElementState;
use glium::backend::glutin::Display;

use graphics::Graphics;
use game_data::GameData;

pub struct Insight {
    board: GameData,
    graphics: Graphics,
    events_loop: EventsLoop,
    display: Display,
    turn: u32,
    closing: bool,
}

impl Insight {
    pub fn open(file_name: &str) -> Insight {
        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new()
            .with_fullscreen(events_loop.get_available_monitors().next())
            .with_title("Ants Insight");
        let context = ContextBuilder::new()
            .with_vsync(true);
        let display = Display::new(window, context, &events_loop).unwrap();

        let board = GameData::load(file_name);
        let graphics = Graphics::new(board.num_rows(), board.num_cols(), &display);
        Insight {
            board,
            graphics,
            events_loop,
            display,
            turn: 0,
            closing: false,
        }
    }

    pub fn run(&mut self) {
        while !self.closing {
            self.handle_events();
            // TODO
            self.graphics.draw_turn(&self.board, self.turn, &self.display);
        }
    }

    fn handle_events(&mut self) {
        use self::glutin::Event::*;
        use self::glutin::WindowEvent as WE;

        let closing = &mut self.closing;
        let graphics = &mut self.graphics;
        let turn = &mut self.turn;
        let num_turns = self.board.num_turns();
        self.events_loop.poll_events(|ev| {
            match ev {
                WindowEvent { event: wev, .. } => match wev {
                    WE::Resized(width, height) => graphics.set_view_port(width, height),
                    WE::Closed => *closing = true,
                    WE::KeyboardInput { input, .. } => {
                        if let ElementState::Pressed = input.state {
                            match input.virtual_keycode {
                                Some(VirtualKeyCode::Q) => {
                                    *closing = true
                                },
                                Some(VirtualKeyCode::Right) => {
                                    *turn += 1;
                                    if *turn >= num_turns {
                                        *turn = num_turns - 1;
                                    }
                                },
                                Some(VirtualKeyCode::Left) => {
                                    if *turn > 0 {
                                        *turn -= 1;
                                    }
                                },
                                _ => (),
                            }
                        }
                    },
                    _ => (),
                },
                _ => (),
            }
        });
    }
}