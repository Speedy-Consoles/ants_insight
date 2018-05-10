use std::thread;
use std::time::Duration;
use std::time::Instant;

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
    game_data: GameData,
    graphics: Graphics,
    events_loop: EventsLoop,
    display: Display,
    turn: u32,
    closing: bool,
    need_redraw: bool,
    last_frame: Instant,
    playing: bool,
    play_speed: f64,
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
            game_data: board,
            graphics,
            events_loop,
            display,
            turn: 0,
            closing: false,
            need_redraw: true,
            last_frame: Instant::now(),
            playing: true,
            play_speed: 1.0,
        }
    }

    pub fn run(&mut self) {
        while !self.closing {
            self.handle_events();
            let play_interval = Duration::new(0, ((1.0 / (2.0 * self.play_speed)) * 1e9) as u32);
            if self.playing && self.last_frame.elapsed() >= play_interval {
                if self.turn < self.game_data.num_turns() - 1 {
                    self.turn += 1;
                    self.need_redraw = true;
                    self.last_frame = Instant::now();
                }
            }
            // TODO
            if self.need_redraw {
                self.graphics.draw_turn(&self.game_data, self.turn, &self.display);
                self.need_redraw = false;
            }
            thread::sleep(Duration::new(0, 16000000));
        }
    }

    fn handle_events(&mut self) {
        use self::glutin::Event::*;
        use self::glutin::WindowEvent as WE;

        let closing = &mut self.closing;
        let graphics = &mut self.graphics;
        let turn = &mut self.turn;
        let num_turns = self.game_data.num_turns();
        let need_redraw = &mut self.need_redraw;
        let playing = &mut self.playing;
        let play_speed = &mut self.play_speed;
        self.events_loop.poll_events(|ev| {
            match ev {
                WindowEvent { event: wev, .. } => match wev {
                    WE::Resized(width, height) => {
                        graphics.set_view_port(width, height);
                        *need_redraw = true;
                    },
                    WE::Closed => *closing = true,
                    WE::KeyboardInput { input, .. } => {
                        if let ElementState::Pressed = input.state {
                            match input.virtual_keycode {
                                Some(VirtualKeyCode::Q) => {
                                    *closing = true
                                },
                                Some(VirtualKeyCode::Right) => {
                                    *playing = false;
                                    if *turn < num_turns - 1 {
                                        *turn += 1;
                                        *need_redraw = true;
                                    }
                                },
                                Some(VirtualKeyCode::Left) => {
                                    *playing = false;
                                    if *turn > 0 {
                                        *turn -= 1;
                                        *need_redraw = true;
                                    }
                                },
                                Some(VirtualKeyCode::Down) => {
                                    if *play_speed > 0.2 {
                                        *play_speed -= 0.2;
                                    }
                                },
                                Some(VirtualKeyCode::Up) => {
                                    if *play_speed < 50.0 {
                                        *play_speed += 0.2;
                                    }
                                },
                                Some(VirtualKeyCode::Space) => {
                                    *playing = !*playing;
                                },
                                Some(VirtualKeyCode::Key0) => {
                                    graphics.toggle_layer(9);
                                    *need_redraw = true;
                                },
                                Some(VirtualKeyCode::Key1) => {
                                    graphics.toggle_layer(0);
                                    *need_redraw = true;
                                },
                                Some(VirtualKeyCode::Key2) => {
                                    graphics.toggle_layer(1);
                                    *need_redraw = true;
                                },
                                Some(VirtualKeyCode::Key3) => {
                                    graphics.toggle_layer(2);
                                    *need_redraw = true;
                                },
                                Some(VirtualKeyCode::Key4) => {
                                    graphics.toggle_layer(3);
                                    *need_redraw = true;
                                },
                                Some(VirtualKeyCode::Key5) => {
                                    graphics.toggle_layer(4);
                                    *need_redraw = true;
                                },
                                Some(VirtualKeyCode::Key6) => {
                                    graphics.toggle_layer(5);
                                    *need_redraw = true;
                                },
                                Some(VirtualKeyCode::Key7) => {
                                    graphics.toggle_layer(6);
                                    *need_redraw = true;
                                },
                                Some(VirtualKeyCode::Key8) => {
                                    graphics.toggle_layer(7);
                                    *need_redraw = true;
                                },
                                Some(VirtualKeyCode::Key9) => {
                                    graphics.toggle_layer(8);
                                    *need_redraw = true;
                                },
                                _ => (),
                            }
                        }
                    },
                    WE::Focused(true) => *need_redraw = true,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}