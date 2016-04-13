use std;
use std::rc::Rc;
use std::cell::RefCell;
use super::player::Player;
use super::property::Property;

const TOTAL_NUM_HOUSES: i32 = 32;
const TOTAL_NUM_HOTELS: i32 = 12;

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

#[derive(Debug)]
pub struct Board {
    unclaimed_houses: i32,
    unclaimed_hotels: i32,
    spaces: Rc<RefCell<Vec<Space>>>,
    players: Rc<RefCell<Vec<Player>>>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            unclaimed_houses: TOTAL_NUM_HOUSES,
            unclaimed_hotels: TOTAL_NUM_HOTELS,
            spaces: Rc::new(RefCell::new(Vec::new())),
            players: Rc::new(RefCell::new(Vec::new())),
        }
    }
}
