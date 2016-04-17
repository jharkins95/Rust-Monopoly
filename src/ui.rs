extern crate opengl_graphics;
extern crate piston;
extern crate glutin_window;

use glutin_window::GlutinWindow;
use std::thread;
use std::sync::{Arc, Mutex};
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL};

use super::board::{Board, Render};


const WINDOW_WIDTH: u32 = 600;
const WINDOW_HEIGHT: u32 = 600;

pub struct Ui {
    main_window: GlutinWindow,
    gl: GlGraphics,
}

impl Ui {
    pub fn new()-> Ui {
        let opengl = OpenGL::V3_2;
        let mut window: GlutinWindow = WindowSettings::new(
            "Monopoly",
            [WINDOW_WIDTH, WINDOW_HEIGHT]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
        Ui {
            main_window: window,
            gl: GlGraphics::new(opengl),
        }
    }

    pub fn run(&mut self, board: Arc<Mutex<Board>>) {
        let mut events = self.main_window.events();
        while let Some(e) = events.next(&mut self.main_window) {
            board.lock().unwrap().update_game_state();
            
            if let Some(r) = e.render_args() {
                board.lock().unwrap().render(&mut self.gl, &r);
            }
            
            if let Some(Button::Mouse(button)) = e.press_args() {
                //println!("Pressed mouse button {:?}", button);
            }
            
            e.mouse_cursor(|x, y| {
                println!("Mouse moved {}, {}", x, y);
            });
        }
    }
}
