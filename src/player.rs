//
//! Player stores the player's properties, their token color,
//! the space the player is currently landed on, the last creditor
//! the player paid rent to, whether the player is in jail,
//! and whether the player is currently up for his turn.
//!

use std::rc::Rc;
use std::cell::RefCell;

use super::property::*;
use super::space::*;

const STARTING_CASH: i32 = 1500;
pub const PLAYER_WIDTH: i32 = 10;

pub const RED:    [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const ORANGE: [f32; 4] = [1.0, 153.0/255.0, 0.0, 1.0];
pub const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
pub const GREEN:  [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const BLUE:   [f32; 4] = [0.0, 0.0, 1.0, 1.0];
pub const PURPLE: [f32; 4] = [102.0/255.0, 0.0, 51.0/255.0, 1.0];

/// An action the player takes upon landing on a property
#[derive(Debug, Clone)]
pub enum LandAction {
    Rent(Rc<RefCell<Property>>),
    Own(Rc<RefCell<Property>>),
    InsFunds(Rc<RefCell<Property>>),
    MightPurchase(Rc<RefCell<Property>>),
    Space(Rc<RefCell<Space>>),
}

#[derive(Debug)]
pub struct Player {
    name: String,
    cash: i32,
    in_jail: bool,
    has_turn: bool,
    space: Rc<RefCell<Space>>,
    properties: Vec<Rc<RefCell<Property>>>,
    token_color: [f32; 4],
    creditor: Option<Rc<RefCell<Player>>>, // None if the creditor is
                                           // the bank
}

impl Player {
    pub fn new(name: String, start_space: Rc<RefCell<Space>>,
                token_color: [f32; 4]) -> Player {
        Player {
            name: name,
            cash: STARTING_CASH,
            in_jail: false,
            has_turn: false,
            space: start_space.clone(),
            properties: Vec::new(),
            token_color: token_color,
            creditor: None,
        }
    }
    
    pub fn get_token_color(&self) -> [f32; 4] {
        self.token_color
    }
    
    pub fn set_creditor(&mut self, creditor: Option<Rc<RefCell<Player>>>) {
        self.creditor = creditor.clone();
    }
    
    pub fn get_creditor(&self) -> Option<Rc<RefCell<Player>>> {
        self.creditor.clone()
    }
    
    pub fn get_property(&self, name: &str) -> Option<Rc<RefCell<Property>>> {
        for prop in &self.properties {
            if name == prop.borrow().get_name() {
                return Some(prop.clone());
            }
        }
        None
    }
    
    pub fn has_property(&self, name: String) -> bool {
        !(self.get_property(&name.clone()).is_none())
    }
    
    pub fn has_monopoly(&self, group: ColorGroup) -> bool {
        let num_in_group = self.get_num_props(&group);
        match group {
            ColorGroup::DarkPurple |
            ColorGroup::DarkBlue  |
            ColorGroup::Utility   => num_in_group == 2,
            
            ColorGroup::LightBlue |
            ColorGroup::LightPurple  |
            ColorGroup::Orange  |
            ColorGroup::Red |
            ColorGroup::Yellow  |
            ColorGroup::Green   => num_in_group == 3,
            
            ColorGroup::Railroad => num_in_group == 4,
            
            _ => unreachable!(),
        }
    }
    
    /// Get all properties of the player that are in a monopoly
    /// not counting railroads/utils; can't put houses and hotels on those
    pub fn get_monopolies(&self) -> Vec<Rc<RefCell<Property>>> {
        let mut props = Vec::new();
        if self.has_monopoly(ColorGroup::DarkPurple) {
            props.push(self.get_property("Mediterranean Avenue").unwrap());
            props.push(self.get_property("Baltic Avenue").unwrap());
        }
        if self.has_monopoly(ColorGroup::LightBlue) {
            props.push(self.get_property("Oriental Avenue").unwrap());
            props.push(self.get_property("Vermont Avenue").unwrap());
            props.push(self.get_property("Connecticut Avenue").unwrap());
        }
        if self.has_monopoly(ColorGroup::LightPurple) {
            props.push(self.get_property("St. Charles Place").unwrap());
            props.push(self.get_property("States Avenue").unwrap());
            props.push(self.get_property("Virginia Avenue").unwrap());
        }
        if self.has_monopoly(ColorGroup::Orange) {
            props.push(self.get_property("St. James Place").unwrap());
            props.push(self.get_property("Tennessee Avenue").unwrap());
            props.push(self.get_property("New York Avenue").unwrap());
        }
        if self.has_monopoly(ColorGroup::Red) {
            props.push(self.get_property("Kentucky Avenue").unwrap());
            props.push(self.get_property("Indiana Avenue").unwrap());
            props.push(self.get_property("Illinois Avenue").unwrap());
        }
        if self.has_monopoly(ColorGroup::Yellow) {
            props.push(self.get_property("Atlantic Avenue").unwrap());
            props.push(self.get_property("Ventnor Avenue").unwrap());
            props.push(self.get_property("Marvin Gardens").unwrap());
        }
        if self.has_monopoly(ColorGroup::Green) {
            props.push(self.get_property("Pennsylvania Avenue").unwrap());
            props.push(self.get_property("North Carolina Avenue").unwrap());
            props.push(self.get_property("Pacific Avenue").unwrap());
        }
        if self.has_monopoly(ColorGroup::DarkBlue) {
            props.push(self.get_property("Park Place").unwrap());
            props.push(self.get_property("Boardwalk").unwrap());
        }
        
        props
    }

    pub fn land(&mut self, space: Rc<RefCell<Space>>) -> LandAction {               
        self.space = space.clone();
        match *(space.borrow().get_type()) {
            SpaceEnum::Prop(ref property) => {
                if property.borrow().is_owned() {
                    if self.properties.contains(property) {
                        return LandAction::Own(property.clone());
                    }
                    return LandAction::Rent(property.clone());
                } else {
                    if self.cash < property.borrow().get_purchase_price() as i32 {
                        return LandAction::InsFunds(property.clone());
                    } else {
                        return LandAction::MightPurchase(property.clone());
                    }
                }
            },
            _ => return LandAction::Space(space.clone()),
        };
    }
    
    pub fn has_monopoly_cg(&self, color_group: &ColorGroup) -> bool {
        let num_props = self.get_num_props(color_group);
        match *color_group {
            ColorGroup::DarkPurple => num_props == 2,
            ColorGroup::LightBlue => num_props == 3,
            ColorGroup::LightPurple => num_props == 3,
            ColorGroup::Orange => num_props == 3,
            ColorGroup::Red => num_props == 3,
            ColorGroup::Yellow => num_props == 3, 
            ColorGroup::Green => num_props == 3, 
            ColorGroup::DarkBlue => num_props == 2,
            ColorGroup::Railroad => num_props == 4,
            ColorGroup::Utility => num_props == 2,
            ColorGroup::Space => unreachable!(),
        }
    }
    
    pub fn get_properties(&self) -> &Vec<Rc<RefCell<Property>>> {
        &self.properties
    }
    
    pub fn add_property(&mut self, property: Rc<RefCell<Property>>) {
        self.properties.push(property.clone());
    }
    
    pub fn purchase(&mut self, property: Rc<RefCell<Property>>) {
        println!("{} purchased {} for ${}!",
                self.name,
                property.borrow().get_name(),
                property.borrow().get_purchase_price());
        self.cash -= property.borrow().get_purchase_price() as i32;
        self.add_property(property.clone());
    }

    pub fn salary(&mut self, salary: i32) {
        self.cash += salary as i32;
    }
    
    pub fn tax(&mut self, tax: i32) {
        self.cash -= tax as i32;
    }
    
    pub fn get_space(&self) -> Rc<RefCell<Space>> {
        self.space.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_cash(&self) -> i32 {
        self.cash
    }

    pub fn is_bankrupt(&self) -> bool {
        self.cash <= 0
    }
    
    pub fn jail(&mut self, space: Rc<RefCell<Space>>) {
        self.space = space;
        self.in_jail = true;
    }
    
    pub fn unjail(&mut self) {
        self.in_jail = false;
    }
    
    pub fn is_in_jail(&self) -> bool {
        self.in_jail
    }
    
    pub fn set_turn(&mut self, turn: bool) {
        self.has_turn = turn;
    }
    
    pub fn print_assets(&self) {
        for asset in &(self.properties) {
            println!("{}", asset.borrow().get_name());
        }
    }

    pub fn collect_rent(&mut self, other: Rc<RefCell<Player>>, 
                        rent: i32) {
        self.cash += rent as i32;
        other.borrow_mut().cash -= rent as i32;
    }
    
    pub fn get_num_props(&self, color_group: &ColorGroup) -> i32 {
        let mut cnt: i32 = 0;
        for property in &self.properties {
            let property = property.borrow();
            if property.get_color_group() == *color_group {
                cnt += 1;
            }
        }
        cnt
    }
    
    pub fn get_x(&self) -> i32 {
        self.space.borrow().get_x()
    }
    
    pub fn get_y(&self) -> i32 {
        self.space.borrow().get_y()
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Player) -> bool {
        self.name == other.name
    }
}
