use std::rc::Rc;
use std::cell::RefCell;
use player::Player;

const MAX_N_HOUSES_PER: i32 = 4;
const MAX_N_HOTELS_PER: i32 = 1;

#[derive(Debug)]
pub enum ColorGroup {
    DARK_PURPLE,
    LIGHT_BLUE,
    LIGHT_PURPLE,
    ORANGE,
    RED,
    YELLOW,
    GREEN,
    DARK_BLUE,
    RAILROAD,
    UTILITY,
}

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

    pub fn get_owner(&self) -> Option<Player> {
        None
    }
}
