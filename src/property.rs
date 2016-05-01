//
//! Contains the majority of the state associated with owning
//! a property, such as its name, the number of houses and hotels
//! held on it, its base rent, and the color group it belongs to.
//!

use std::rc::Rc;
use std::cell::RefCell;
use piston::input::*;
use opengl_graphics::GlGraphics;

use super::player::*;
use super::game::*;

pub const MAX_NUM_HOUSES: i32 = 4;
pub const MAX_NUM_HOTELS: i32 = 1;

pub const HOUSE_COST: i32 = 80;
pub const HOTEL_COST: i32 = 160;

/// ColorGroups are used to determine if
/// a player holds a monopoly.
#[derive(Debug, Clone, PartialEq)]
pub enum ColorGroup {
    DarkPurple,
    LightBlue,
    LightPurple,
    Orange,
    Red,
    Yellow,
    Green,
    DarkBlue,
    Railroad,
    Utility,
    Space
}

/// Represents a property by which players can collect rent.
/// Can be bought, sold, and traded (eventually).
#[derive(Debug)]
pub struct Property {
    name: String,
    purchase_price: i32,
    base_rent: i32,
    color_group: ColorGroup,
    owner: Option<Rc<RefCell<Player>>>,
    is_mortgaged: bool,
    num_houses: i32,
    num_hotels: i32,
}

impl Property {
    pub fn new(name: String, purchase_price: i32, base_rent: i32,
               color_group: ColorGroup) -> Self {
        Property {
            name: name,
            purchase_price: purchase_price,
            base_rent: base_rent,
            color_group: color_group,
            owner: None,
            is_mortgaged: false,
            num_houses: 0,
            num_hotels: 0,
        }
    }
    
    pub fn get_num_houses(&self) -> i32 {
        self.num_houses as i32
    }
    
    pub fn get_num_hotels(&self) -> i32 {
        self.num_hotels as i32
    }

    pub fn has_houses(&self) -> bool {
        self.num_houses > 0
    }

    pub fn has_hotel(&self) -> bool {
        self.num_hotels > 0
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn is_mortgaged(&self) -> bool {
        self.is_mortgaged
    }

    pub fn get_base_rent(&self) -> i32 {
        self.base_rent
    }
    
    pub fn get_color_group(&self) -> ColorGroup {
        self.color_group.clone()
    }

    pub fn get_owner(&self) -> &Rc<RefCell<Player>> {
        match self.owner {
            None => panic!(""),
            Some(ref owner) => owner,
        }
    }

    pub fn add_house(&mut self) {
        self.num_houses += 1;
    }

    pub fn get_purchase_price(&self) -> i32 {
        self.purchase_price
    }

    pub fn remove_house(&mut self) {
        self.num_houses -= 1
    }

    pub fn add_hotel(&mut self) {
        self.num_hotels += 1;
    }

    pub fn remove_hotel(&mut self) {
        self.num_hotels -= 1;
    }
    
    pub fn is_owned(&self) -> bool {
        !(self.owner == None)
    }
    
    pub fn set_owner(&mut self, owner: Option<Rc<RefCell<Player>>>) {
        self.owner = owner;
    }
}

impl Render for Property {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        println!("Drew a property");
    }
}

impl PartialEq for Property {
    fn eq(&self, other: &Property) -> bool {
        self.name == other.name
    }
}
