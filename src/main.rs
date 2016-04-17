extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use std::thread;
use std::sync::{Arc, Mutex};
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

pub mod board;
pub mod player;
pub mod property;
pub mod cards;
pub mod ui;

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
    let mut ui = ui::Ui::new();
    let mut board = Arc::new(Mutex::new(board::Board::new()));
    board.lock().unwrap().setup_game();
    ui.run(board.clone());
}
