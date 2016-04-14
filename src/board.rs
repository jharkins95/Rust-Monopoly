extern crate rand;
extern crate opengl_graphics;
extern crate piston;
extern crate core;

use std;
use std::io::{self, Read, Write};
use std::collections::VecDeque;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::cell::RefCell;
use rand::Rng;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;

use super::player::Player;
use super::property::{Property, ColorGroup};

const TOTAL_NUM_HOUSES: i32 = 32;
const TOTAL_NUM_HOTELS: i32 = 12;
const NUM_SPACES: usize = 40;
const MAX_NUM_PLAYERS: u32 = 6;
pub const GO_SALARY: i32 = 200;

/// Objects that can be drawn to the screen with
/// the Piston/OpenGL framework
pub trait Draw {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs);
}

/// Represents a space on the board that players can land on
/// (not necessarily a property)
#[derive(Debug)]
pub enum Space {
    Prop(Rc<RefCell<Property>>),
    Go(i32),
    Chance,
    CommunityChest,
    Jail,
    FreeParking,
    GoToJail,
    IncomeTax(i32),
    LuxuryTax(i32),
}

/// A Board contains all useful game state (players, spaces, properties)
#[derive(Debug)]
pub struct Board {
    unclaimed_houses: i32,
    unclaimed_hotels: i32,
    spaces: VecDeque<Rc<RefCell<Space>>>,
    players: Vec<Rc<RefCell<Player>>>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            unclaimed_houses: TOTAL_NUM_HOUSES,
            unclaimed_hotels: TOTAL_NUM_HOTELS,
            spaces: VecDeque::with_capacity(NUM_SPACES),
            players: Vec::new(),
        }
    }
    
    /// Update the current state of the game
    pub fn update_game_state(&mut self) {
        
    }

    fn fill_spaces(&mut self) {
        let go = Space::Go(GO_SALARY);
        let chance_bot = Space::Chance;
        let chance_top = Space::Chance;
        let chance_right = Space::Chance;
        let comm_chest_bot = Space::CommunityChest;
        let comm_chest_left = Space::CommunityChest;
        let comm_chest_right = Space::CommunityChest;
        let jail = Space::Jail;
        let free_parking = Space::FreeParking;
        let go_to_jail = Space::GoToJail;
        let income_tax = Space::IncomeTax(200);
        let luxury_tax = Space::LuxuryTax(75);
        
        let med_ave = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Mediterranean Avenue".to_string(),
                                  60,
                                  2,
                                  ColorGroup::DarkPurple))));
        let balt_ave = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Baltic Avenue".to_string(),
                                  60,
                                  4,
                                  ColorGroup::DarkPurple))));    
        let reading_rr = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Reading Railroad".to_string(),
                                  150,
                                  25,
                                  ColorGroup::Railroad))));    
        let orient_ave = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Oriental Avenue".to_string(),
                                  100,
                                  6,
                                  ColorGroup::LightBlue))));     
        let verm_ave = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Vermont Avenue".to_string(),
                                  100,
                                  6,
                                  ColorGroup::LightBlue))));          
        let conn_ave = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Connecticut Avenue".to_string(),
                                  120,
                                  8,
                                  ColorGroup::LightBlue))));   
        let st_char_pl = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "St. Charles Place".to_string(),
                                  140,
                                  10,
                                  ColorGroup::LightPurple))));         
        let states_ave = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "States Avenue".to_string(),
                                  140,
                                  10,
                                  ColorGroup::LightPurple))));   
        let va_ave = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Virginia Avenue".to_string(),
                                  160,
                                  12,
                                  ColorGroup::LightPurple))));       
        let pa_rr = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Pennsylvania Railroad".to_string(),
                                  150,
                                  25,
                                  ColorGroup::Railroad))));   
        let st_james_pl = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "St. James Place".to_string(),
                                  180,
                                  14,
                                  ColorGroup::Orange))));      
        let tn_ave = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Tennessee Avenue".to_string(),
                                  180,
                                  14,
                                  ColorGroup::Orange))));       
        let ny_ave = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "New York Avenue".to_string(),
                                  200,
                                  16,
                                  ColorGroup::Orange))));     
        let ky_ave = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Kentucky Avenue".to_string(),
                                  220,
                                  18,
                                  ColorGroup::Red))));     
        let in_ave = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Indiana Avenue".to_string(),
                                  220,
                                  18,
                                  ColorGroup::Red))));       
        let il_ave = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Illinois Avenue".to_string(),
                                  240,
                                  20,
                                  ColorGroup::Red))));       
        let bo_rr = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "B&O Railroad".to_string(),
                                  150,
                                  25,
                                  ColorGroup::Railroad))));                                   
        let atl_ave = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Atlantic Avenue".to_string(),
                                  260,
                                  22,
                                  ColorGroup::DarkPurple))));      
        let ventnor_ave = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Ventnor Avenue".to_string(),
                                  260,
                                  22,
                                  ColorGroup::Yellow))));      
        let mar_gard = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Marvin Gardens".to_string(),
                                  280,
                                  22,
                                  ColorGroup::Yellow))));      
        let pac_ave = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Pacific Avenue".to_string(),
                                  300,
                                  26,
                                  ColorGroup::Green))));     
        let nc_ave = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "North Carolina Avenue".to_string(),
                                  300,
                                  26,
                                  ColorGroup::Green))));       
        let pa_ave = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Pennsylvania Avenue".to_string(),
                                  320,
                                  28,
                                  ColorGroup::Green))));    
        let sl_rr = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Short Line".to_string(),
                                  150,
                                  25,
                                  ColorGroup::Railroad))));      
        let park_pl = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Park Place".to_string(),
                                  350,
                                  35,
                                  ColorGroup::DarkBlue)))); 
        let bdwk = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Boardwalk".to_string(),
                                  400,
                                  50,
                                  ColorGroup::DarkBlue))));      
        let elec_util = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Electric Company".to_string(),
                                  150,
                                  8,
                                  ColorGroup::Utility))));   
        let water_util = Space::Prop(Rc::new(RefCell::new(Property::new(
                                  "Water Works".to_string(),
                                  150,
                                  8,
                                  ColorGroup::Utility))));                                   
                                   
                                   
        
        self.spaces.push_back(Rc::new(RefCell::new(go)));
        self.spaces.push_back(Rc::new(RefCell::new(med_ave)));
        self.spaces.push_back(Rc::new(RefCell::new(comm_chest_bot)));
        self.spaces.push_back(Rc::new(RefCell::new(balt_ave)));
        self.spaces.push_back(Rc::new(RefCell::new(income_tax)));
        self.spaces.push_back(Rc::new(RefCell::new(reading_rr)));
        self.spaces.push_back(Rc::new(RefCell::new(orient_ave)));
        self.spaces.push_back(Rc::new(RefCell::new(chance_bot)));
        self.spaces.push_back(Rc::new(RefCell::new(verm_ave)));
        self.spaces.push_back(Rc::new(RefCell::new(conn_ave)));
        self.spaces.push_back(Rc::new(RefCell::new(jail)));
        self.spaces.push_back(Rc::new(RefCell::new(st_char_pl)));
        self.spaces.push_back(Rc::new(RefCell::new(elec_util)));
        self.spaces.push_back(Rc::new(RefCell::new(states_ave)));
        self.spaces.push_back(Rc::new(RefCell::new(va_ave)));
        self.spaces.push_back(Rc::new(RefCell::new(pa_rr)));
        self.spaces.push_back(Rc::new(RefCell::new(st_james_pl)));
        self.spaces.push_back(Rc::new(RefCell::new(comm_chest_left)));
        self.spaces.push_back(Rc::new(RefCell::new(tn_ave)));
        self.spaces.push_back(Rc::new(RefCell::new(ny_ave)));
        self.spaces.push_back(Rc::new(RefCell::new(free_parking)));
        self.spaces.push_back(Rc::new(RefCell::new(ky_ave)));
        self.spaces.push_back(Rc::new(RefCell::new(chance_top)));
        self.spaces.push_back(Rc::new(RefCell::new(in_ave)));
        self.spaces.push_back(Rc::new(RefCell::new(il_ave)));
        self.spaces.push_back(Rc::new(RefCell::new(bo_rr)));
        self.spaces.push_back(Rc::new(RefCell::new(atl_ave)));
        self.spaces.push_back(Rc::new(RefCell::new(ventnor_ave)));
        self.spaces.push_back(Rc::new(RefCell::new(water_util)));
        self.spaces.push_back(Rc::new(RefCell::new(mar_gard)));
        self.spaces.push_back(Rc::new(RefCell::new(go_to_jail)));
        self.spaces.push_back(Rc::new(RefCell::new(pac_ave)));
        self.spaces.push_back(Rc::new(RefCell::new(nc_ave)));
        self.spaces.push_back(Rc::new(RefCell::new(comm_chest_right)));
        self.spaces.push_back(Rc::new(RefCell::new(pa_ave)));
        self.spaces.push_back(Rc::new(RefCell::new(sl_rr)));
        self.spaces.push_back(Rc::new(RefCell::new(chance_right)));
        self.spaces.push_back(Rc::new(RefCell::new(park_pl)));
        self.spaces.push_back(Rc::new(RefCell::new(luxury_tax)));
        self.spaces.push_back(Rc::new(RefCell::new(bdwk)));
        
        
    }

    pub fn setup_game(&mut self) {
        let mut input = String::new();
        let mut num_players = 2; // default
    
        println!("Welcome to Monopoly!");
        print!("How many players today? ");
        
        let num_players = get_num_players();
        
        let mut turns_to_names: BTreeMap<i32, String> = BTreeMap::new();

        for i in 0..num_players {
            println!("Please enter Player {}'s name: ", i + 1);
            let mut name = String::new();
            io::stdin().read_line(&mut name).unwrap();
            
            let mut n = get_dice_roll();
            while turns_to_names.contains_key(&n) {
                n = get_dice_roll();
            }
            turns_to_names.insert(n, name.trim().to_string());
        }

        for (_, name) in turns_to_names {
            self.players.push(Rc::new(RefCell::new(
                              Player::new(name, Space::Go(GO_SALARY)))));
        }
        
        self.fill_spaces();
        self.players[0].borrow_mut().set_turn(true);
        
        println!("Game setup complete.");
        
    }
}

impl Draw for Board {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        unimplemented!();
    }
}


/*
 *  UTILITY FUNCTIONS
 */

/// Get a yes/no answer from stdin
pub fn confirm_prompt() -> bool {
    loop {
        let mut input = String::new();
        io::stdout().flush();
        io::stdin().read_line(&mut input).unwrap();
        match &(*input.trim().to_lowercase()) {
            "yes" => return true,
            "no"  => return false,
            _ => println!("Please enter yes or no"),
        }
    }
}

/// Get an integer from stdin
pub fn get_num_players() -> u32 {
    use std::str::FromStr;
    
    loop {
        let mut input = String::new();
        io::stdout().flush();
        io::stdin().read_line(&mut input).unwrap();
        match u32::from_str(&(input.trim())) {
            Ok(n) => {
                if n < 2 || n > MAX_NUM_PLAYERS {
                    println!("Number of players must be >= 2 and <= {}",
                             MAX_NUM_PLAYERS);
                } else {
                    return n;
                }
            },
            Err(e) => println!("Please enter an integer"),
        }
    }
}

/// Simulate a dice roll: return an integer between 2 and 12, inclusive
pub fn get_dice_roll() -> i32 {
    let mut rng = rand::thread_rng();
    let first: i32 = rng.gen_range(1, 7); // [1, 7)
    let second: i32 = rng.gen_range(1, 7);
    first + second
}
