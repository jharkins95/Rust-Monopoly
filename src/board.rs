extern crate rand;
extern crate opengl_graphics;
extern crate piston;
extern crate core;

use std;
use std::io::{self, Read, Write};
use std::collections::VecDeque;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::process;
use std::fmt;
use std::cell::RefCell;
use rand::Rng;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;

use super::player::{Player, LandAction};
use super::property::{Property, ColorGroup};

const TOTAL_NUM_HOUSES: i32 = 32;
const TOTAL_NUM_HOTELS: i32 = 12;
const NUM_SPACES: usize = 40;
const MAX_NUM_PLAYERS: i32 = 6;
pub const GO_SALARY: i32 = 200;

/// Represents a player's choice on their turn
pub enum Turn {
    Roll,
    Quit,
    GetAssets,
    // TODO: add more types of actions (trades, buy/sell houses)
}

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

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

/// A Board contains all useful game state (players, spaces, properties)
#[derive(Debug)]
pub struct Board {
    unclaimed_houses: i32,
    unclaimed_hotels: i32,
    spaces: Vec<Rc<RefCell<Space>>>,
    players: Vec<Rc<RefCell<Player>>>,
    player_turn: usize, // index into playerss
}

impl Board {
    pub fn new() -> Board {
        Board {
            unclaimed_houses: TOTAL_NUM_HOUSES,
            unclaimed_hotels: TOTAL_NUM_HOTELS,
            spaces: Vec::with_capacity(NUM_SPACES),
            players: Vec::new(),
            player_turn: 0,
        }
    }
    
    /// Update the current state of the game
    pub fn update_game_state(&mut self) {
        println!("It is {}'s turn", self.players[self.player_turn]
                                    .borrow().get_name());
        println!("Please enter a command: roll, quit, assets");
        print!(">> ");
        let action = get_turn_action();
        match action {
            Turn::Roll => {
                let mut player = self.players[self.player_turn].clone();
                let dice_roll = get_dice_roll() as usize;
                let new_index = self.get_index(player.borrow()
                    .get_space() + dice_roll);
                println!("{} rolled a {}.",
                         player.borrow().get_name(),
                         dice_roll);
                let result = player.borrow_mut().land(&self.spaces, new_index);
                match result {
                    LandAction::Purchase(ref prop) => {
                        let mut property = prop.borrow_mut();
                        property.purchase(player);
                    },
                    _ => (),
                }
                self.advance_to_next_turn();
            },
            Turn::Quit => {
                print!("Are you sure you want to quit? ");
                if confirm_prompt() {
                    process::exit(0);
                }
            },
            Turn::GetAssets => {
                let player = self.players[self.player_turn].borrow();
                println!("{} has ${} and the following assets:",
                        player.get_name(), player.get_cash());
                player.print_assets();
            },
        };
        println!("");
        
    }
    
    fn advance_to_next_turn(&mut self) {
        self.player_turn = self.get_next_turn(self.player_turn + 1);
    }
    
    fn get_index(&mut self, index: usize) -> usize {
        index % NUM_SPACES
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
                                   
                                   
        
        self.spaces.push(Rc::new(RefCell::new(go)));
        self.spaces.push(Rc::new(RefCell::new(med_ave)));
        self.spaces.push(Rc::new(RefCell::new(comm_chest_bot)));
        self.spaces.push(Rc::new(RefCell::new(balt_ave)));
        self.spaces.push(Rc::new(RefCell::new(income_tax)));
        self.spaces.push(Rc::new(RefCell::new(reading_rr)));
        self.spaces.push(Rc::new(RefCell::new(orient_ave)));
        self.spaces.push(Rc::new(RefCell::new(chance_bot)));
        self.spaces.push(Rc::new(RefCell::new(verm_ave)));
        self.spaces.push(Rc::new(RefCell::new(conn_ave)));
        self.spaces.push(Rc::new(RefCell::new(jail)));
        self.spaces.push(Rc::new(RefCell::new(st_char_pl)));
        self.spaces.push(Rc::new(RefCell::new(elec_util)));
        self.spaces.push(Rc::new(RefCell::new(states_ave)));
        self.spaces.push(Rc::new(RefCell::new(va_ave)));
        self.spaces.push(Rc::new(RefCell::new(pa_rr)));
        self.spaces.push(Rc::new(RefCell::new(st_james_pl)));
        self.spaces.push(Rc::new(RefCell::new(comm_chest_left)));
        self.spaces.push(Rc::new(RefCell::new(tn_ave)));
        self.spaces.push(Rc::new(RefCell::new(ny_ave)));
        self.spaces.push(Rc::new(RefCell::new(free_parking)));
        self.spaces.push(Rc::new(RefCell::new(ky_ave)));
        self.spaces.push(Rc::new(RefCell::new(chance_top)));
        self.spaces.push(Rc::new(RefCell::new(in_ave)));
        self.spaces.push(Rc::new(RefCell::new(il_ave)));
        self.spaces.push(Rc::new(RefCell::new(bo_rr)));
        self.spaces.push(Rc::new(RefCell::new(atl_ave)));
        self.spaces.push(Rc::new(RefCell::new(ventnor_ave)));
        self.spaces.push(Rc::new(RefCell::new(water_util)));
        self.spaces.push(Rc::new(RefCell::new(mar_gard)));
        self.spaces.push(Rc::new(RefCell::new(go_to_jail)));
        self.spaces.push(Rc::new(RefCell::new(pac_ave)));
        self.spaces.push(Rc::new(RefCell::new(nc_ave)));
        self.spaces.push(Rc::new(RefCell::new(comm_chest_right)));
        self.spaces.push(Rc::new(RefCell::new(pa_ave)));
        self.spaces.push(Rc::new(RefCell::new(sl_rr)));
        self.spaces.push(Rc::new(RefCell::new(chance_right)));
        self.spaces.push(Rc::new(RefCell::new(park_pl)));
        self.spaces.push(Rc::new(RefCell::new(luxury_tax)));
        self.spaces.push(Rc::new(RefCell::new(bdwk)));
        
        
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
                              Player::new(name, 0))));
        }
        
        self.fill_spaces();
        self.players[0].borrow_mut().set_turn(true);
        
        println!("Game setup complete.\n");
        
    }
    
    /// Returns the index of the next turn
    pub fn get_next_turn(&self, index: usize) -> usize {
        index % self.players.len()
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

 pub fn get_string() -> String {
    let mut input = String::new();
    io::stdout().flush();
    io::stdin().read_line(&mut input).unwrap();
    input
 }
 
/// Get a yes/no answer from stdin
pub fn confirm_prompt() -> bool {
    loop {
        let mut input = get_string();
        match &(*input.trim().to_lowercase()) {
            "yes" => return true,
            "no"  => return false,
            _ => print!("Please enter yes or no: "),
        }
    }
}

pub fn get_turn_action() -> Turn {
    loop {
        let mut input = get_string();
        match &(*input.trim().to_lowercase()) {
            "roll" => return Turn::Roll,
            "quit"  => return Turn::Quit,
            "assets" => return Turn::GetAssets,
            _ => print!("Please enter a valid command: "),
        }
    }
}

/// Get an integer from stdin
pub fn get_int() -> i32 {
    use std::str::FromStr;
    
    loop {
        let mut input = String::new();
        io::stdout().flush();
        io::stdin().read_line(&mut input).unwrap();
        match i32::from_str(&(input.trim())) {
            Ok(n) => return n,
            Err(e) => print!("Please enter an integer: "),
        }
    }
}

/// Get the desired number of players
pub fn get_num_players() -> i32 {
    loop {
        let n = get_int();
        if n < 2 || n > MAX_NUM_PLAYERS {
            print!("Number of players must be >= 2 and <= {}: ",
                     MAX_NUM_PLAYERS);
        } else {
            return n;
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
