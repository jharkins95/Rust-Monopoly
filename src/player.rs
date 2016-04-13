use std;
use std::rc::Rc;
use std::cell::RefCell;
use super::property::Property;
use super::board::Board;

const STARTING_CASH: i32 = 1500;

#[derive(Debug)]
pub struct Player {
    name: String,
    cash: i32,
    properties: Rc<RefCell<Vec<Property>>>,
    is_bankrupt: bool,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name: name,
            cash: STARTING_CASH,
            properties: Rc::new(RefCell::new(Vec::new())),
            is_bankrupt: false,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_cash(&self) -> i32 {
        self.cash
    }

    pub fn is_bankrupt(&self) -> bool {
        self.is_bankrupt
    }
}
