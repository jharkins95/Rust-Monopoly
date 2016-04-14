extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use std::thread;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

pub mod board;
pub mod player;
pub mod property;
pub mod cards;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

pub struct App {
    x: i32,
    y: i32,
}

impl App {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0; 4];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);

        gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            let transform = c.transform.trans(self.x as f64, self.y as f64);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Monopoly",
            [WINDOW_WIDTH, WINDOW_HEIGHT]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        x: 0,
        y: 0,
    };
    
    let mut gl = GlGraphics::new(opengl);
    
    let mut board = board::Board::new();
    board.setup_game();
    
    let mut cursor = [0.0, 0.0];

    // Main event loop
    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Mouse(button)) = e.press_args() {
            println!("Pressed mouse button {:?}", button);
        }
        
        e.mouse_cursor(|x, y| {
            app.x = x as i32;
            app.y = y as i32;
            println!("Mouse moved {}, {}", x, y);
        });
        
        board.update_game_state();
    
        if let Some(r) = e.render_args() {
            app.render(&mut gl, &r);
        }
    }
}
