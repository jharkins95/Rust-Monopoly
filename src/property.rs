use std::rc::Rc;
use std::cell::RefCell;
use piston::input::*;
use opengl_graphics::GlGraphics;

use super::board::*;
use player::*;

const MAX_NUM_HOUSES: i32 = 4;
const MAX_NUM_HOTELS: i32 = 1;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Property {
    name: String,
    purchase_price: u32,
    base_rent: u32,
    color_group: ColorGroup,
    owner: Option<Rc<RefCell<Player>>>,
    is_mortgaged: bool,
    num_houses: u32,
    num_hotels: u32,
}

impl Property {
    pub fn new(name: String, purchase_price: u32, base_rent: u32,
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

    pub fn get_rent(&self) -> Result<u32, String> {
        Ok(self.base_rent) // TODO: calculate rent based on hotels, houses, monops
    }

    pub fn get_owner(&self) -> &Rc<RefCell<Player>> {
        match self.owner {
            None => panic!(""),
            Some(ref owner) => owner,
        }
    }

    pub fn add_houses(&mut self, houses: u32) -> Result<(), String> {
        let num_houses = self.num_houses as i32;
        let houses = houses as i32;
        if num_houses + houses >= MAX_NUM_HOUSES {
            return Err(format!("{} cannot have more than {} houses!",
                self.name, self.num_houses))
        }
        self.num_houses += houses as u32;
        Ok(())
    }

    pub fn get_purchase_price(&self) -> u32 {
        self.purchase_price
    }

    pub fn remove_houses(&mut self, houses: u32) -> Result<(), String> {
        let num_houses = self.num_houses as i32;
        let houses = houses as i32;
        if num_houses - houses < 0 {
            return Err(format!("{} cannot have less than 0 houses!", self.name))
        }
        self.num_houses -= houses as u32;
        Ok(())
    }

    pub fn add_hotel(&mut self) -> Result<(), String> {
        let num_hotels = self.num_hotels as i32;
        if num_hotels + 1 >= MAX_NUM_HOTELS {
            return Err(format!("{} already has a hotel!", self.name))
        }
        self.num_hotels += 1;
        Ok(())
    }

    pub fn remove_hotel(&mut self) -> Result<(), String> {
        let num_hotels = self.num_hotels as i32;
        if num_hotels - 1 < 0 {
            return Err(format!("{} cannot have less than 0 hotels!", self.name))
        }
        self.num_hotels -= 1;
        Ok(())
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
