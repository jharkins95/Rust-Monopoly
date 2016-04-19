extern crate rand;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;
extern crate core;

use std::io::{self, Write};
use std::collections::BTreeMap;
use std::rc::Rc;
use std::fmt;
use std::cell::RefCell;
use rand::Rng;
use piston::input::*;
use opengl_graphics::{GlGraphics, Texture};
use std::path::Path;

use super::player::*;
use super::property::{Property, ColorGroup};
use super::game::*;

const TOTAL_NUM_HOUSES: i32 = 32;
const TOTAL_NUM_HOTELS: i32 = 12;
const NUM_SPACES: usize = 40;
const MAX_NUM_PLAYERS: i32 = 6;

static mut space_cnt: usize = 0;

/// Objects that can be drawn to the screen with
/// the Piston/OpenGL framework
pub trait Render {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs);
}

#[derive(Debug)]
pub enum SpaceEnum {
    Prop(Rc<RefCell<Property>>),
    Go,
    Chance,
    CommunityChest,
    Jail,
    FreeParking,
    GoToJail,
    IncomeTax,
    LuxuryTax,
}

/// Represents a space on the board that players can land on
/// (not necessarily a property)
#[derive(Debug)]
pub struct Space {
    s_type: SpaceEnum,
    x: i32,
    y: i32,
    index: usize,
}

impl Space {
    pub fn new(prop: SpaceEnum, x: i32, y: i32) -> Space {
        let space = Space {
            s_type: prop,
            x: x,
            y: y,
            index: unsafe { space_cnt },
        };
        unsafe { space_cnt += 1; }
        space
    }
    
    pub fn get_index(&self) -> usize {
        self.index
    }
    
    pub fn get_type(&self) -> &SpaceEnum {
        &(self.s_type)
    }
    
    pub fn get_x(&self) -> i32 {
        self.x
    }
    
    pub fn get_y(&self) -> i32 {
        self.y
    }
}

impl Render for Space {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
    /*
        match *self {
            Space::Prop(ref property) => {
                property.borrow().render(gl, args);
            },
            _ => (),
        };
        */
        //println!("Drew a space");
    }
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

/// A Board contains all useful game state (players, spaces, properties)
//#[derive(Debug)]
pub struct Board {
    unclaimed_houses: i32,
    unclaimed_hotels: i32,
    spaces: Vec<Rc<RefCell<Space>>>,
    players: Vec<Rc<RefCell<Player>>>,
    player_turn: usize, // index into playerss
    image: Texture,
}

impl Board {
    pub fn new() -> Board {
        Board {
            unclaimed_houses: TOTAL_NUM_HOUSES,
            unclaimed_hotels: TOTAL_NUM_HOTELS,
            spaces: Vec::with_capacity(NUM_SPACES),
            players: Vec::new(),
            player_turn: 0,
            image: Texture::from_path(Path::new("res/board.png")).unwrap()
        }
    }
    
    pub fn start_turn(&mut self) {
        println!("It is {}'s turn. Please enter a command: roll, quit, assets", 
            self.players[self.player_turn].borrow().get_name());
        self.players[self.player_turn].borrow_mut().set_turn(true);
    }
    
    /// Debtor is assumed to be the current player
    pub fn on_rent_collected(&mut self, owner: &mut Player,
                             prop: &Property) {
        let mut debtor = self.players[self.player_turn].borrow_mut();
        owner.collect_rent(&mut *debtor, prop);
    }
    
    pub fn on_purchase(&mut self, prop: Rc<RefCell<Property>>) {
        self.players[self.player_turn].borrow_mut().purchase(prop.clone());
        prop.borrow_mut().purchase(self.players[self.player_turn].clone());
    }
    
    pub fn on_land_go(&mut self, salary: u32) {
        println!("You landed on GO! Collect ${}.", salary);
        self.players[self.player_turn].borrow_mut().salary(salary);
    }
    
    pub fn on_land_chance(&mut self) {
        println!("Landed on Chance");
        // TODO: handle chance
    }
    
    pub fn on_land_comm_chest(&mut self) {
        println!("Landed on Community Chest");
        // TODO: handle comm. chest
    }
    
    pub fn on_land_jail(&mut self) {
        println!("Just visiting...");
    }
    
    pub fn on_land_free_parking(&mut self) {
        println!("Landed on Free Parking");
    }
    
    pub fn on_land_go_to_jail(&mut self, go_salary: u32) {
        println!("Go to jail! Go directly to jail! Do not pass \
                  GO! Do not collect ${}!", go_salary);   
        self.players[self.player_turn].borrow_mut().jail(
            self.spaces[10].clone());
    }
    
    pub fn on_land_income_tax(&mut self, tax: u32) {
        println!("Income tax! Pay ${}.", tax);
        self.players[self.player_turn].borrow_mut().tax(tax);
    }
    
    pub fn on_land_luxury_tax(&mut self, tax: u32) {
        println!("Luxury tax! Pay ${}.", tax);
        self.players[self.player_turn].borrow_mut().tax(tax);
    }
    
    pub fn get_space(&self, index: usize) -> Rc<RefCell<Space>> {
        let index = self.get_player_index(index);
        self.spaces[index].clone()
    }
    
    pub fn roll_and_land(&mut self) -> LandAction {
        let mut player = self.players[self.player_turn].clone();
        let dice_roll = get_dice_roll() as usize;
        let old_space = player.borrow().get_space();
        let old_player_index = old_space.borrow().get_index();
        let new_player_index = self.get_player_index(old_player_index + dice_roll);
        
        let new_space = self.spaces[new_player_index].clone();
        println!("{} rolled a {}.",
                 player.borrow().get_name(),
                 dice_roll);
        self.players[self.player_turn].borrow_mut().land(new_space)
    }
    
    pub fn print_player_assets(&self) {
        let player = self.players[self.player_turn].borrow();
        println!("{} has ${} and the following assets:",
                player.get_name(), player.get_cash());
        player.print_assets();
    }
    
    pub fn end_turn(&mut self) {
        self.players[self.player_turn].borrow_mut()
            .set_turn(false);
        self.player_turn = self.get_next_turn(self.player_turn + 1);
    }
    
    pub fn get_player_index(&self, index: usize) -> usize {
        index % self.spaces.len()
    }

    pub fn reset_spaces(&mut self) {  
        let go = Space::new(SpaceEnum::Go, 522, 520);
        let med_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Mediterranean Avenue".to_string(),
                                  60,
                                  2,
                                  ColorGroup::DarkPurple)))), 472, 520);
        let comm_chest_bot = Space::new(SpaceEnum::CommunityChest, 426, 520);
        let balt_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Baltic Avenue".to_string(),
                                  60,
                                  4,
                                  ColorGroup::DarkPurple)))), 376, 520);  
        let income_tax = Space::new(SpaceEnum::IncomeTax, 328, 520);
        let reading_rr = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Reading Railroad".to_string(),
                                  150,
                                  25,
                                  ColorGroup::Railroad)))), 280, 520);    
        let orient_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Oriental Avenue".to_string(),
                                  100,
                                  6,
                                  ColorGroup::LightBlue)))), 231, 520);
        let chance_bot = Space::new(SpaceEnum::Chance, 183, 520);
        let verm_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Vermont Avenue".to_string(),
                                  100,
                                  6,
                                  ColorGroup::LightBlue)))), 134, 520);          
        let conn_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Connecticut Avenue".to_string(),
                                  120,
                                  8,
                                  ColorGroup::LightBlue)))), 88, 520);  
        let jail = Space::new(SpaceEnum::Jail, 4, 520);                                  
        let st_char_pl = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "St. Charles Place".to_string(),
                                  140,
                                  10,
                                  ColorGroup::LightPurple)))), 4, 472);
        let elec_util = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Electric Company".to_string(),
                                  150,
                                  8,
                                  ColorGroup::Utility)))), 4, 424);                                  
        let states_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "States Avenue".to_string(),
                                  140,
                                  10,
                                  ColorGroup::LightPurple)))), 4, 376);   
        let va_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Virginia Avenue".to_string(),
                                  160,
                                  12,
                                  ColorGroup::LightPurple)))), 4, 327);       
        let pa_rr = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Pennsylvania Railroad".to_string(),
                                  150,
                                  25,
                                  ColorGroup::Railroad)))), 4, 280);   
        let st_james_pl = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "St. James Place".to_string(),
                                  180,
                                  14,
                                  ColorGroup::Orange)))), 4, 230);      
        let comm_chest_left = Space::new(SpaceEnum::CommunityChest, 4, 182);
        let tn_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Tennessee Avenue".to_string(),
                                  180,
                                  14,
                                  ColorGroup::Orange)))), 4, 133);       
        let ny_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "New York Avenue".to_string(),
                                  200,
                                  16,
                                  ColorGroup::Orange)))), 4, 85);     
        let free_parking = Space::new(SpaceEnum::FreeParking, 4, 4);
        let ky_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Kentucky Avenue".to_string(),
                                  220,
                                  18,
                                  ColorGroup::Red)))), 88, 4);     
        let chance_top = Space::new(SpaceEnum::Chance, 135, 4);
        let in_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Indiana Avenue".to_string(),
                                  220,
                                  18,
                                  ColorGroup::Red)))), 184, 4);       
        let il_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Illinois Avenue".to_string(),
                                  240,
                                  20,
                                  ColorGroup::Red)))), 232, 4);       
        let bo_rr = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "B&O Railroad".to_string(),
                                  150,
                                  25,
                                  ColorGroup::Railroad)))), 280, 4);                                   
        let atl_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Atlantic Avenue".to_string(),
                                  260,
                                  22,
                                  ColorGroup::DarkPurple)))), 328, 4);      
        let ventnor_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Ventnor Avenue".to_string(),
                                  260,
                                  22,
                                  ColorGroup::Yellow)))), 377, 4);      
        let water_util = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Water Works".to_string(),
                                  150,
                                  8,
                                  ColorGroup::Utility)))), 425, 4);  
        let mar_gard = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Marvin Gardens".to_string(),
                                  280,
                                  22,
                                  ColorGroup::Yellow)))), 474, 4);      
        let go_to_jail = Space::new(SpaceEnum::GoToJail, 522, 4);
        let pac_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Pacific Avenue".to_string(),
                                  300,
                                  26,
                                  ColorGroup::Green)))), 522, 85);     
        let nc_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "North Carolina Avenue".to_string(),
                                  300,
                                  26,
                                  ColorGroup::Green)))), 522, 133);       
        let comm_chest_right = Space::new(SpaceEnum::CommunityChest, 522, 181);
        let pa_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Pennsylvania Avenue".to_string(),
                                  320,
                                  28,
                                  ColorGroup::Green)))), 522, 230);    
        let sl_rr = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Short Line".to_string(),
                                  150,
                                  25,
                                  ColorGroup::Railroad)))), 522, 279);      
        let chance_right = Space::new(SpaceEnum::Chance, 522, 327);
        let park_pl = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Park Place".to_string(),
                                  350,
                                  35,
                                  ColorGroup::DarkBlue)))), 522, 375); 
        let luxury_tax = Space::new(SpaceEnum::LuxuryTax, 522, 424);
        let bdwk = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Boardwalk".to_string(),
                                  400,
                                  50,
                                  ColorGroup::DarkBlue)))), 522, 472);      

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
    
    pub fn add_player(&mut self, player: Player) {
        self.players.push(Rc::new(RefCell::new(player)));
    }
    
    /// Returns the index of the next turn
    fn get_next_turn(&self, index: usize) -> usize {
        index % self.players.len()
    }
}

impl Render for Board {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;
    
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(0.0, 0.0);
            image(&(self.image), transform, gl);
        });
        
        for space in &(self.spaces) {
            space.borrow().render(gl, args);
        }
        
        for player in &(self.players) {
            player.borrow().render(gl, args);
        }
        //println!("Drew the board");
    }
}


/*
 *  UTILITY FUNCTIONS
 */

pub fn get_token_color() -> usize {
    loop {
        let mut color = get_string();
        match &(*color.trim().to_lowercase()) {
            "red" => return 0,
            "orange" => return 1,
            "yellow" => return 2,
            "green" => return 3,
            "blue" => return 4,
            "purple" => return 5,
            _ => print!("Please enter a valid color: "),
        };
    }
}
 
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

pub fn get_turn_action() -> TurnCommand {
    loop {
        let mut input = get_string();
        match &(*input.trim().to_lowercase()) {
            "roll" => return TurnCommand::Roll,
            "quit"  => return TurnCommand::Quit,
            "assets" => return TurnCommand::Assets,
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
