//
//! CIS 198 - Rust Programming
//! Author: Jack Harkins
//! Project Name: Rust-Monopoly
//!

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

mod board;
mod player;
mod property;
mod cards;
mod game;
mod space;

/// The main entry point for the application
fn main() {
    game::Game::new().run();
}
