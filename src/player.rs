use std::rc::Rc;
use std::cell::RefCell;
use piston::input::*;
use opengl_graphics::GlGraphics;

use super::property::*;
use super::board::*;

const STARTING_CASH: i32 = 1500;

pub const WHITE:  [f32; 4] = [1.0; 4];
pub const RED:    [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const ORANGE: [f32; 4] = [1.0, 153.0/255.0, 0.0, 1.0];
pub const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
pub const GREEN:  [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const BLUE:   [f32; 4] = [0.0, 0.0, 1.0, 1.0];
pub const PURPLE: [f32; 4] = [102.0/255.0, 0.0, 51.0/255.0, 1.0];


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
        }
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
    
    pub fn purchase(&mut self, property: Rc<RefCell<Property>>) {
        println!("{} purchased {} for ${}!",
                self.name,
                property.borrow().get_name(),
                property.borrow().get_purchase_price());
        self.cash -= property.borrow().get_purchase_price() as i32;
        self.properties.push(property.clone());
    }

    pub fn salary(&mut self, salary: u32) {
        self.cash += salary as i32;
    }
    
    pub fn tax(&mut self, tax: u32) {
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
    
    pub fn get_x(&self) -> i32 {
        self.space.borrow().get_x()
    }
    
    pub fn get_y(&self) -> i32 {
        self.space.borrow().get_y()
    }
}

impl Render for Player {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;
        
        let token = rectangle::square(self.space.borrow().get_x() as f64, 
                                      self.space.borrow().get_y() as f64, 
                                      20.0);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            rectangle(self.token_color, token, transform, gl);
        });
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Player) -> bool {
        self.name == other.name
    }
}
