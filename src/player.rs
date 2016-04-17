use std;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use super::property::Property;
use super::board::*;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

const STARTING_CASH: i32 = 1500;

#[derive(Debug)]
pub enum LandAction {
    Rent,
    Own,
    InsFunds,
    Purchase(Rc<RefCell<Property>>),
    NoPurchase,
    Space,
}

#[derive(Debug)]
pub struct Player {
    name: String,
    cash: i32,
    in_jail: bool,
    has_turn: bool,
    space: usize, // index into spaces array in Board
    properties: Vec<Rc<RefCell<Property>>>,
}

impl Player {
    pub fn new(name: String, start_space: usize) -> Player {
        Player {
            name: name,
            cash: STARTING_CASH,
            in_jail: false,
            has_turn: false,
            space: start_space,
            properties: Vec::new(),
        }
    }

    pub fn land(&mut self, spaces: 
                &Vec<Rc<RefCell<Space>>>, index: usize) -> LandAction {
                
        let space = spaces[index].borrow();
        self.space = index;
        match *space {
            Space::Prop(ref property) => {
                if property.borrow().is_owned() {
                    if self.properties.contains(property) {
                        let property = property.borrow();
                        println!("You already own {}.",
                                property.get_name());
                        return LandAction::Own;
                    }
                    let property = property.borrow();
                    let owner_rc = property.get_owner();
                    let mut owner = (*owner_rc).borrow_mut();
                    println!("{} is owned by {}. Pay rent of ${}!", 
                             property.get_name(),
                             owner.get_name(), 
                             property.get_rent().unwrap());
                    owner.collect_rent(self, &*property);
                    return LandAction::Rent;
                } else {
                    if self.cash < property.borrow().get_purchase_price() as i32 {
                        println!("You don't have enough money for that!");
                        return LandAction::InsFunds;
                    } else {
                        println!("{} is not owned. Would you like to buy it?",
                                 property.borrow().get_name());
                        if confirm_prompt() {
                            println!("{} purchased {} for ${}!",
                                    self.name,
                                    property.borrow().get_name(),
                                    property.borrow().get_purchase_price());
                            self.cash -= property.borrow().get_purchase_price() as i32;
                            self.properties.push(property.clone());
                            return LandAction::Purchase(property.clone());
                        }
                        return LandAction::NoPurchase;
                    }
                }
            },
            Space::Go(salary) => {
                println!("You landed on GO! Collect ${}.", salary);
                self.cash += salary;
            },
            Space::Chance => {println!("Landed on Chance");},
            Space::CommunityChest => {println!("Landed on CC");},
            Space::Jail => println!("Just visiting..."),
            Space::FreeParking => {
                println!("Landed on Free Parking");
            },
            Space::GoToJail => {
                println!("Go to jail! Go directly to jail! Do not pass\
                          GO! Do not collect ${}!", GO_SALARY);
                self.space = 10;
                self.jail();
                return LandAction::Space;
            },
            Space::IncomeTax(tax) => {
                println!("INCOME TAX! Pay 10% or ${}", tax);
                self.cash -= tax;  // TODO: choose between 10% or tax
            },
            Space::LuxuryTax(tax) => {
                println!("Pay LUXURY TAX of {}!", tax);
                self.cash -= tax;
            },
        };
        LandAction::Space
    }
    
    pub fn get_space(&self) -> usize {
        self.space
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
    
    pub fn jail(&mut self) {
        self.in_jail = true;
    }
    
    pub fn set_turn(&mut self, turn: bool) {
        self.has_turn = turn;
    }
    
    pub fn print_assets(&self) {
        for asset in &(self.properties) {
            println!("{}", asset.borrow().get_name());
        }
    }

    pub fn collect_rent(&mut self, other: &mut Player, property: &Property) {
        let rent = property.get_rent().unwrap() as i32;
        self.cash += rent;
        other.cash -= rent;
    }
}

impl Render for Player {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        println!("Drew a player");
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Player) -> bool {
        self.name == other.name
    }
}
