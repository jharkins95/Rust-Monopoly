extern crate rand;
extern crate opengl_graphics;
extern crate piston;

use std;
use std::io::{self, Read, Write};
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
use rand::Rng;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;

use super::player::Player;
use super::property::Property;

const TOTAL_NUM_HOUSES: i32 = 32;
const TOTAL_NUM_HOTELS: i32 = 12;
const NUM_SPACES: usize = 40;

/// Objects that can be drawn to the screen with
/// the Piston/OpenGL framework
pub trait Draw {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs);
    fn update(&mut self, gl: &mut GlGraphics, args: &UpdateArgs);
}

/// Represents a space on the board that players can land on
/// (not necessarily a property)
#[derive(Debug)]
pub enum Space {
    Prop(Property),
    Go(i32),
    Chance,
    CommunityChest,
    Jail,
    FreeParking,
    GoToJail,
    IncomeTax(i32),
}

/// A Board contains all useful game state (players, spaces, properties)
#[derive(Debug)]
pub struct Board {
    unclaimed_houses: i32,
    unclaimed_hotels: i32,
    spaces: Rc<RefCell<VecDeque<Space>>>,
    players: Rc<RefCell<Vec<Player>>>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            unclaimed_houses: TOTAL_NUM_HOUSES,
            unclaimed_hotels: TOTAL_NUM_HOTELS,
            spaces: Rc::new(RefCell::new(VecDeque::with_capacity(NUM_SPACES))),
            players: Rc::new(RefCell::new(Vec::new())),
        }
    }
    
    /// Update the current state of the game
    pub fn update_game_state(&mut self) {
        
    }
    
    pub fn start_game(&mut self) {
        let mut input = String::new();
        let mut num_players = 2; // default
    
        println!("Welcome to Monopoly!");
        print!("How many players today? ");
        io::stdout().flush();
        loop {
            io::stdin().read_line(&mut input).unwrap();
            println!("Input was {}", input);
            match input.trim().parse::<i32>() {
                Ok(n) => {
                    num_players = n;
                    break;
                },
                Err(e) => { 
                    print!("Please enter a valid integer ");
                    io::stdout().flush();
                },
            }
        }
        
        for i in 0..num_players {
            println!("Please enter Player {}'s name: ", i + 1);
        }
        
        println!("Game setup OK");
        
    }
}

impl Draw for Board {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        unimplemented!();
    }
    
    
    fn update(&mut self, gl: &mut GlGraphics, args: &UpdateArgs) {
        unimplemented!();
    }
}

/// Simulate a dice roll: return an integer between 2 and 12, inclusive
pub fn get_dice_roll() -> i32 {
    let mut rng = rand::thread_rng();
    let first: i32 = rng.gen_range(1, 7); // [1, 7)
    let second: i32 = rng.gen_range(1, 7);
    first + second
}
