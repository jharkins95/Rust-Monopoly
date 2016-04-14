extern crate opengl_graphics;
extern crate piston;
extern crate glutin_window;

use glutin_window::GlutinWindow as Window;
use std::boxed::Box

use super::board::Board;

#[derive(Debug)]
pub struct Ui {
    window: Box<Window>,
    board: Box<Board>,
}

impl Ui {
    pub fn new(window: Window, board: Board) -> Ui {
        Ui {
            window: Box::new(window),
            board: Box::new(board),
        }
    }
}
