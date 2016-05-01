//
//! Space stores the player(s) who currently are landed on
//! the space, if any. It also acts as a container for Properties
//! since every property is a space on the board (the converse isn't
//! necessarily true)
//!

extern crate rand;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;
extern crate core;

use std::rc::Rc;
use std::cell::RefCell;
use piston::input::*;
use opengl_graphics::GlGraphics;

use super::player::*;
use super::property::*;
use super::game::*;

/// Space indices
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

/// Represents the type of a space (property, tax, card draw, jail, etc.)
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
    
    /// Removes a player from the space 
    // (there's no remove() method in Vec other than removing by index)
    pub fn remove_player(&mut self, other: Rc<RefCell<Player>>) {
        for i in 0..self.players.len() {
            if self.players[i] == other {
                self.players.remove(i);
                break; // otherwise, would lead to the index going
                       // out of bounds
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
        use graphics::*;
        
        let house_width: f64 = 5.0;
        let house_height: f64 = 5.0;
        let hotel_width: f64 = 10.0;
        let hotel_height: f64 = 5.0;
        
        let mut offset: f64 = 0.0;
        if let SpaceEnum::Prop(prop) = self.s_type.clone() {
            let num_houses = prop.borrow().get_num_houses();
            let num_hotels = prop.borrow().get_num_hotels();
            
            // houses and hotels are mutually exclusive
            for _ in 0..num_houses {
                let house: [graphics::types::Scalar; 4] = 
                    [self.x as f64, 
                     self.y as f64, 
                     house_width, 
                     house_height];
                gl.draw(args.viewport(), |c, gl| {
                    let transform = c.transform.trans(offset as f64, 0 as f64);
                    rectangle(GREEN, house, transform, gl);
                });
                offset += house_width + 1.0; // border between houses
            }
            for _ in 0..num_hotels {
                let hotel: [graphics::types::Scalar; 4] = 
                    [self.x as f64, 
                     self.y as f64, 
                     hotel_width, 
                     hotel_height];
                gl.draw(args.viewport(), |c, gl| {
                    let transform = c.transform.trans(offset as f64, 0 as f64);
                    rectangle(RED, hotel, transform, gl);
                });
                offset = offset + hotel_width + 1.0;
            }
        }
        
        offset = 0.0;
        for player in &self.players {
            let player = player.borrow();
            let token = rectangle::square(self.x as f64, 
                                          self.y as f64, 
                                          PLAYER_WIDTH as f64);

            gl.draw(args.viewport(), |c, gl| {
                let transform = c.transform.trans(0 as f64, 
                                                  house_height + 1 as f64 + offset as f64);
                rectangle(player.get_token_color(), token, transform, gl);
            });
            offset += PLAYER_WIDTH as f64;
        }
    }
}
