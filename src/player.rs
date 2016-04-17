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

pub const WHITE:  [f32; 4] = [1.0; 4];
pub const RED:    [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const ORANGE: [f32; 4] = [1.0, 153.0/255.0, 0.0, 1.0];
pub const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
pub const GREEN:  [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const BLUE:   [f32; 4] = [0.0, 0.0, 1.0, 1.0];
pub const PURPLE: [f32; 4] = [102.0/255.0, 0.0, 51.0/255.0, 1.0];


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
pub enum PlayerToken {
    Red,
    Orange
}

#[derive(Debug)]
pub struct Player {
    name: String,
    cash: i32,
    in_jail: bool,
    has_turn: bool,
    space: usize, // index into spaces array in Board
    properties: Vec<Rc<RefCell<Property>>>,
    token_color: [f32; 4],
    x: i32,
    y: i32,
}

impl Player {
    pub fn new(name: String, start_space: usize,
                token_color: [f32; 4]) -> Player {
        Player {
            name: name,
            cash: STARTING_CASH,
            in_jail: false,
            has_turn: false,
            space: start_space,
            properties: Vec::new(),
            token_color: token_color,
            x: 0,
            y: 0,
        }
    }

    pub fn land(&mut self, spaces: 
                &Vec<Rc<RefCell<Space>>>, index: usize) -> LandAction {
                
        let space = spaces[index].borrow();
        self.space = index;
        self.x = spaces[index].borrow().get_x();
        self.y = spaces[index].borrow().get_y();
        match *(space.get_type()) {
            SpaceEnum::Prop(ref property) => {
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
            SpaceEnum::Go(salary) => {
                println!("You landed on GO! Collect ${}.", salary);
                self.cash += salary;
            },
            SpaceEnum::Chance => {println!("Landed on Chance");},
            SpaceEnum::CommunityChest => {println!("Landed on CC");},
            SpaceEnum::Jail => println!("Just visiting..."),
            SpaceEnum::FreeParking => {
                println!("Landed on Free Parking");
            },
            SpaceEnum::GoToJail => {
                println!("Go to jail! Go directly to jail! Do not pass\
                          GO! Do not collect ${}!", GO_SALARY);
                self.space = 10;
                self.jail();
                return LandAction::Space;
            },
            SpaceEnum::IncomeTax(tax) => {
                println!("INCOME TAX! Pay 10% or ${}", tax);
                self.cash -= tax;  // TODO: choose between 10% or tax
            },
            SpaceEnum::LuxuryTax(tax) => {
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
        use graphics::*;
        
        let token = rectangle::square(self.x as f64, self.y as f64, 5.0);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            rectangle(self.token_color, token, transform, gl);
        });
        //println!("Drew a player");
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Player) -> bool {
        self.name == other.name
    }
}
