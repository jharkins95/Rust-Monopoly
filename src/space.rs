extern crate rand;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;
extern crate core;

use std::io::{self, Write};
use std::collections::BTreeMap;
use std::rc::Rc;
use std::fmt;
use std::cell::RefCell;
use rand::Rng;
use piston::input::*;
use opengl_graphics::{GlGraphics, Texture};
use std::path::Path;

use super::player::*;
use super::board::*;
use super::property::*;
use super::game::*;

pub const GO: usize = 0;
pub const MED_AVE: usize = 1;
pub const COMM_CHEST_BOT: usize = 2;
pub const BALT_AVE: usize = 3;
pub const INCOME_TAX: usize = 4;
pub const READING_RR: usize = 5;
pub const ORIENT_AVE: usize = 6;
pub const CHANCE_BOT: usize = 7;
pub const VERM_AVE: usize = 8;
pub const CONN_AVE: usize = 9;
pub const JAIL: usize = 10;
pub const ST_CHAR_PL: usize = 11;
pub const ELEC_UTIL: usize = 12;
pub const STATES_AVE: usize = 13;
pub const VA_AVE: usize = 14;
pub const PA_RR: usize = 15;
pub const ST_JAMES_PL: usize = 16;
pub const COMM_CHEST_LEFT: usize = 17;
pub const TN_AVE: usize = 18;
pub const NY_AVE: usize = 19;
pub const FREE_PARKING: usize = 20;
pub const KY_AVE: usize = 21;
pub const CHANCE_TOP: usize = 22;
pub const IN_AVE: usize = 23;
pub const IL_AVE: usize = 24;
pub const BO_RR: usize = 25;
pub const ATL_AVE: usize = 26;
pub const VENTNOR_AVE: usize = 27;
pub const WATER_UTIL: usize = 28;
pub const MAR_GARD: usize = 29;
pub const GO_TO_JAIL: usize = 30;
pub const PAC_AVE: usize = 31;
pub const NC_AVE: usize = 32;
pub const COMM_CHEST_RIGHT: usize = 33;
pub const PA_AVE: usize = 34;
pub const SL_RR: usize = 35;
pub const CHANCE_RIGHT: usize = 36;
pub const PARK_PL: usize = 37;
pub const LUXURY_TAX: usize = 38;
pub const BDWK: usize = 39;

#[derive(Debug, Clone)]
pub enum SpaceEnum {
    Prop(Rc<RefCell<Property>>),
    Go,
    Chance,
    CommunityChest,
    Jail,
    FreeParking,
    GoToJail,
    IncomeTax,
    LuxuryTax,
}

/// Represents a space on the board that players can land on
/// (not necessarily a property)
#[derive(Debug)]
pub struct Space {
    s_type: SpaceEnum,
    x: i32,
    y: i32,
    index: usize,
    players: Vec<Rc<RefCell<Player>>>,
}

impl Space {
    pub fn new(prop: SpaceEnum, x: i32, y: i32, index: usize) -> Space {
        Space {
            s_type: prop,
            x: x,
            y: y,
            index: index,
            players: Vec::new(),
        }
    }
    
    pub fn add_player(&mut self, player: Rc<RefCell<Player>>) {
        self.players.push(player.clone());
    }
    
    pub fn remove_player(&mut self, other: Rc<RefCell<Player>>) {
        for i in 0..self.players.len() {
            if self.players[i] == other {
                self.players.remove(i);
                break;
            }
        }
    }
    
    pub fn get_index(&self) -> usize {
        self.index
    }
    
    pub fn get_type(&self) -> &SpaceEnum {
        &(self.s_type)
    }
    
    pub fn get_x(&self) -> i32 {
        self.x
    }
    
    pub fn get_y(&self) -> i32 {
        self.y
    }
}

impl Render for Space {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let mut offset = 0;
        for player in &self.players {
        
            use graphics::*;
            let player = player.borrow();
            let x = self.get_x();
            let y = self.get_y();
            let token = rectangle::square(x as f64, 
                                          y as f64, 
                                          PLAYER_WIDTH as f64);

            gl.draw(args.viewport(), |c, gl| {
                let transform = c.transform.trans(0 as f64, offset as f64);
                rectangle(player.get_token_color(), token, transform, gl);
            });
            offset += PLAYER_WIDTH;
        }
    }
}