extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{PressEvent, Button, Key, MouseButton, MouseCursorEvent};
use std::any::Any;

pub struct App {
    gl: GlGraphics,
    squares: Vec<Vec<bool>>,
    game_running: bool
}

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

impl App {

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let r_len = self.squares.len();
        let state = self.squares.clone();

        self.gl.draw(args.viewport(), |context, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            // Draw white lines
            for i in 0..r_len {
                let y = (args.window_size[1] * (i as f64 / r_len as f64));
                let x = (args.window_size[0] * (i as f64 / r_len as f64));
                line(BLACK, 1., [0., y, args.window_size[0], y], context.transform, gl);
                line(BLACK, 1., [x, 0., x, args.window_size[1]], context.transform, gl);
            }

            // Draw squares
            for r in 0..r_len {
                let y = (args.window_size[1] * (r as f64 / r_len as f64));
                for c in 0..r_len {
                    if state[r][c] {
                        let x = (args.window_size[0] * (c as f64 / r_len as f64));
                        let square = rectangle::square(x, y, 10.0);
                        rectangle(RED, square, context.transform, gl);
                    }
                }
            }
        });
    }

    fn create_new_squares(&mut self) {
        let mut new_squares = vec![vec![false; 100]; 100];

        let r_len = self.squares.len();
        let c_len = self.squares[0].len();
        for r in 0..r_len {
            for c in 0..c_len {
                let mut n = 0 as i8;
                if r == 0 && c == 0 {
                    n += if self.squares[r][c + 1] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r + 1][c] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r + 1][c + 1] { 1 as i8 } else { 0 as i8 };
                }
                else if r == 0 && c == c_len - 1 {
                    n += if self.squares[r][c - 1] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r + 1][c] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r + 1][c - 1] { 1 as i8 } else { 0 as i8 };
                }
                else if r == r_len - 1 && c == 0 {
                    n += if self.squares[r][c + 1] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r - 1][c] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r - 1][c + 1] { 1 as i8 } else { 0 as i8 };
                }
                else if r == r_len - 1 && c == c_len - 1 {
                    n += if self.squares[r][c - 1] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r - 1][c] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r - 1][c - 1] { 1 as i8 } else { 0 as i8 };
                }
                else if r == 0 {
                    n += if self.squares[r][c - 1] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r][c + 1] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r + 1][c - 1] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r + 1][c] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r + 1][c + 1] { 1 as i8 } else { 0 as i8 };
                }
                else if r == r_len - 1 {
                    n += if self.squares[r][c - 1] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r][c + 1] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r - 1][c - 1] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r - 1][c] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r - 1][c + 1] { 1 as i8 } else { 0 as i8 };
                }
                else if c == 0 {
                    n += if self.squares[r - 1][c] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r - 1][c + 1] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r][c + 1] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r + 1][c] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r + 1][c + 1] { 1 as i8 } else { 0 as i8 };
                }
                else if c == c_len - 1 {
                    n += if self.squares[r - 1][c] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r - 1][c - 1] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r][c - 1] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r + 1][c] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r + 1][c - 1] { 1 as i8 } else { 0 as i8 };
                }
                else {
                    n += if self.squares[r - 1][c - 1] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r - 1][c] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r - 1][c + 1] { 1 as i8 } else { 0 as i8 };

                    n += if self.squares[r][c - 1] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r][c + 1] { 1 as i8 } else { 0 as i8 };

                    n += if self.squares[r + 1][c - 1] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r + 1][c] { 1 as i8 } else { 0 as i8 };
                    n += if self.squares[r + 1][c + 1] { 1 as i8 } else { 0 as i8 };
                }

                // alive cell 2 or 3 live neighbors
                if self.squares[r][c] && (n == 2 || n == 3) {
                    new_squares[r][c] = true;
                }
                // dead cell with 3 live neighbors
                else if !self.squares[r][c] && n == 3 {
                    new_squares[r][c] = true;
                }
                // all other cells die
                else if self.squares[r][c] {
                    new_squares[r][c] = false;
                }
            }
        }
        self.squares = new_squares;
    }

    fn update(&mut self, args: &UpdateArgs) {
        if self.game_running {
            self.create_new_squares()
        }
    }

    fn run(&mut self, mut window: Window, game_size: [u32; 2]) {
        let mut events = Events::new(EventSettings::new());
        let mut mouse_coords: [f64; 2] = [0., 0.];
        let r_len = self.squares.len();
        let c_len = self.squares[0].len();
        while let Some(e) = events.next(&mut window) {
            if let Some(args) = e.render_args() {
                self.render(&args);
            }

            if let Some(args) = e.update_args() {
                self.update(&args);
            }

            if let Some(args) = e.press_args() {
                match args {
                    Button::Keyboard(key) => {
                        if key.eq(&Key::Return) {
                            self.game_running = !self.game_running;
                        }
                    }
                    Button::Mouse(button) => {
                        if button.eq(&MouseButton::Left) {
                            let r = (mouse_coords[1] / (game_size[1] as f64 / r_len as f64)) as i8;
                            let c = (mouse_coords[0] / (game_size[0] as f64 / c_len as f64)) as i8;
                            self.squares[r as usize][c as usize] = true;
                        }
                    }
                    _ => {}
                }
            }

            if let Some(args) = e.mouse_cursor_args() {
                mouse_coords = args;
            }
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let game_size = [1080, 1080];
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Game of Life", game_size)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        squares: vec![vec![false; 100]; 100],
        game_running: false
    };

    app.run(window, game_size);
}