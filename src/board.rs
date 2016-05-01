//
//! Board keeps track of most of the game state local to the Monopoly
//! board, including the spaces (and properties contained within them),
//! the players, the card decks, and the background board image.
//!
//! Most of the methods in Board do not take in a current player
//! argument since this is already determined by the player_turn
//! field in Board.
//!


extern crate rand;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;
extern crate core;

use std::io::{self, Write};
use std::collections::BTreeMap;
use std::rc::Rc;
use std::cell::RefCell;
use rand::Rng;
use piston::input::*;
use opengl_graphics::{GlGraphics, Texture};
use std::path::Path;

use super::player::*;
use super::property::*;
use super::game::*;
use super::cards::*;
use super::space::*;

const NUM_SPACES: usize = 40;
const MAX_NUM_PLAYERS: i32 = 6;

pub struct Board {
    spaces: Vec<Rc<RefCell<Space>>>,
    players: Vec<Rc<RefCell<Player>>>,
    player_turn: usize, // index into playerss
    image: Texture,
    chance_cards: Vec<Chance>,
    comm_chest_cards: Vec<CommunityChest>,
    passed_go: bool,
}

impl Board {
    pub fn new() -> Board {
        Board {
            spaces: Vec::with_capacity(NUM_SPACES),
            players: Vec::new(),
            player_turn: 0,
            image: Texture::from_path(Path::new("res/board.png")).unwrap(),
            chance_cards: Vec::new(),
            comm_chest_cards: Vec::new(),
            passed_go: false,
        }
    }
    

    
    pub fn shuffle_chance(&mut self) {
        self.chance_cards = Vec::new();
        let mut rng = rand::thread_rng();
        let mut order_to_card: BTreeMap<i32, Chance> = BTreeMap::new();
        
        // Yes, this is not guaranteed to always insert every card.
        // In real life though, we sometimes forget to shuffle a card :)
        order_to_card.insert(rng.gen_range(1, 10000000), Chance::AdvanceToGo);
        order_to_card.insert(rng.gen_range(1, 10000000), Chance::AdvanceToNearestUtility);
        order_to_card.insert(rng.gen_range(1, 10000000), Chance::AdvanceToNearestRailroad);
        order_to_card.insert(rng.gen_range(1, 10000000), Chance::GoBack3Spaces);
        order_to_card.insert(rng.gen_range(1, 10000000), Chance::AdvanceToBoardwalk);
        
        for (_, card) in order_to_card {
            self.chance_cards.push(card);
        }
    }
    
    pub fn shuffle_comm_chest(&mut self) {
        self.comm_chest_cards = Vec::new();
        let mut rng = rand::thread_rng();
        let mut order_to_card: BTreeMap<i32, CommunityChest> = BTreeMap::new();
        
        // Yes, this is not guaranteed to always insert every card.
        // In real life though, we sometimes forget to shuffle a card :)
        order_to_card.insert(rng.gen_range(1, 10000000), CommunityChest::AdvanceToGo);
        order_to_card.insert(rng.gen_range(1, 10000000), CommunityChest::BankErrorInYourFavor);
        order_to_card.insert(rng.gen_range(1, 10000000), CommunityChest::GoToJail);
        order_to_card.insert(rng.gen_range(1, 10000000), CommunityChest::PaySchoolFees);
        
        for (_, card) in order_to_card {
            self.comm_chest_cards.push(card);
        }
    }
    
    pub fn start_turn(&mut self) {
        let mut player = self.get_current_player();
        while player.borrow().is_bankrupt() { // skip bankrupt players
            self.end_turn();
            player = self.get_current_player();
        }

        println!("It is {}'s turn. You have ${}.",
                 player.borrow().get_name(),
                 player.borrow().get_cash());
        
        player.borrow_mut().set_creditor(None);
        player.borrow_mut().set_turn(true);
    }
    
    /// Debtor is assumed to be the current player
    pub fn on_rent_collected(&mut self, owner: Rc<RefCell<Player>>,
                             prop: Rc<RefCell<Property>>) {
        let debtor = self.players[self.player_turn].clone();
        let rent = self.get_rent(prop.clone());
        owner.borrow_mut().collect_rent(debtor.clone(), rent);
        debtor.borrow_mut().set_creditor(Some(owner.clone()));
    }
    
    pub fn handle_bankruptcy(&mut self) {
        let debtor = self.get_current_player();
        if debtor.borrow().is_bankrupt() {
            println!("{} is bankrupt!", debtor.borrow().get_name());
            let creditor = {
                let debtor = debtor.borrow();
                debtor.get_creditor().clone()
            };
            match creditor {
                Some(ref creditor) => {
                    println!("{}'s assets will be transferred to \
                              {}, the creditor.",
                              debtor.borrow().get_name(),
                              creditor.borrow().get_name());
                    self.acquire_assets(creditor.clone());
                },
                //TODO: return assets to bank
                None => {
                    println!("{}'s assets will be transferred back to the bank.",
                             debtor.borrow().get_name());
                    self.return_assets();
                },
            }
        }
    }
    
    pub fn acquire_assets(&mut self, creditor: Rc<RefCell<Player>>) {
        let debtor = self.get_current_player();
        for property in debtor.borrow().get_properties() {
            creditor.borrow_mut().add_property(property.clone());
            property.borrow_mut().set_owner(Some(creditor.clone()));
        }
    }
    
    pub fn handle_pass_go(&mut self) {
        let player = self.get_current_player();
        println!("{} passed GO and collected {}.",
                 player.borrow().get_name(),
                 GO_SALARY);
        player.borrow_mut().salary(GO_SALARY);
        self.passed_go = false;
    }
    
    pub fn return_assets(&mut self) {
        let debtor = self.get_current_player();
        for property in debtor.borrow().get_properties() {
            property.borrow_mut().set_owner(None);
        }
    }
    
    pub fn on_purchase(&mut self, prop: Rc<RefCell<Property>>) {
        let buyer = self.get_current_player();
        buyer.borrow_mut().purchase(prop.clone());
        prop.borrow_mut().set_owner(Some(buyer.clone()));
    }
    
    pub fn on_land_go(&mut self, salary: i32) {
        println!("You landed on GO! Collect ${}.", salary);
        self.players[self.player_turn].borrow_mut().salary(salary);
    }
    
    pub fn get_num_remaining_players(&self) -> i32 {
        let mut cnt: i32 = 0;
        for player in &self.players {
            if !player.borrow().is_bankrupt() {
                cnt += 1;
            }
        }
        cnt
    }
    
    pub fn get_winner(&self) -> Option<Rc<RefCell<Player>>> {
        if self.get_num_remaining_players() > 1 {
            None
        } else {
            for player in &self.players {
                if !player.borrow().is_bankrupt() {
                    return Some(player.clone())
                }
            }
            unreachable!();
        }
    }
    
    pub fn get_nearest_utility(&self) -> Rc<RefCell<Space>> {
        let index = self.get_player_index();
        if index < ELEC_UTIL {
            self.get_space(ELEC_UTIL).clone()
        } else if index < WATER_UTIL {
            self.get_space(WATER_UTIL).clone()
        } else {
            self.get_space(ELEC_UTIL).clone()
        }
    }
    
    pub fn get_nearest_railroad(&self) -> Rc<RefCell<Space>> {
        let index = self.get_player_index();
        if index < READING_RR {
            self.get_space(READING_RR).clone()
        } else if index < PA_RR {
            self.get_space(PA_RR).clone()
        } else if index < BO_RR {
            self.get_space(BO_RR).clone()
        } else if index < SL_RR {
            self.get_space(SL_RR).clone()
        } else {
            self.get_space(READING_RR).clone()
        }
    }
    
    pub fn on_land_chance(&mut self) -> LandAction {
        if self.chance_cards.len() == 0 {
            self.shuffle_chance();
        }
        println!("Landed on Chance");
        let card = self.chance_cards.pop().unwrap();
        
        match card {
            Chance::AdvanceToGo => {
                let space = self.get_space(GO);
                println!("Advance to GO!");
                self.advance_to(space.clone())
            },
            Chance::AdvanceToNearestUtility => {
                let space = self.get_nearest_utility();
                println!("Advance to nearest utility!");
                self.advance_to(space.clone())
            },
            Chance::AdvanceToNearestRailroad => {
                let space = self.get_nearest_railroad();
                println!("Advance to nearest railroad!");
                self.advance_to(space.clone())
            },
            Chance::GoBack3Spaces => {
                let current_index = self.get_player_index() as i32;
                let new_index = current_index - 3;
                let new_index = {
                    if new_index >= 0 { 
                        new_index as usize
                    } else {
                        self.spaces.len() + new_index as usize
                    }
                };
                let new_space = self.get_space(new_index);
                println!("Go back 3 spaces!");
                self.advance_to(new_space)
            },
            Chance::AdvanceToBoardwalk => {
                let space = self.get_space(BDWK);
                println!("Advance to Boardwalk!");
                self.advance_to(space.clone())
            },
        }
        
    }
    
    pub fn on_land_comm_chest(&mut self) {
        if self.comm_chest_cards.len() == 0 {
            self.shuffle_comm_chest();
        }
        println!("Landed on Community Chest");
        let card = self.comm_chest_cards.pop().unwrap();
        let player = self.get_current_player();
        
        match card {
            CommunityChest::AdvanceToGo => {
                let space = self.get_space(GO);
                println!("Advance to GO!");
                self.advance_to(space.clone());
            },
            CommunityChest::BankErrorInYourFavor => {
                println!("Bank error in your favor! Collect $200.");
                player.borrow_mut().salary(200);
            },
            CommunityChest::GoToJail => {
                println!("Go to jail!");
                let gtj = player.borrow().get_space();
                let jail = self.get_space(10).clone();
                gtj.borrow_mut().remove_player(player.clone());
                jail.borrow_mut().add_player(player.clone());
                player.borrow_mut().jail(self.spaces[JAIL].clone());
            },
            CommunityChest::PaySchoolFees => {
                println!("Pay school fees of $50!");
                player.borrow_mut().tax(50);
            },
        };
    }
    
    pub fn on_land_jail(&mut self) {
        println!("Just visiting...");
    }
    
    pub fn on_land_free_parking(&mut self) {
        println!("Landed on Free Parking");
        // TODO: add free parking salary??
    }
    
    pub fn get_rent(&self, property: Rc<RefCell<Property>>) -> i32 {
        let color_group = property.borrow().get_color_group();
        let base_rent = property.borrow().get_base_rent();
        let owner = {
            let property = property.borrow();
            property.get_owner().clone()
        };    
        let num_props = owner.borrow().get_num_props(&color_group);
        let has_monopoly = owner.borrow().has_monopoly(color_group.clone());
        let num_houses = property.borrow().get_num_houses();
        let num_hotels = property.borrow().get_num_hotels();
        match color_group {
            ColorGroup::Railroad => {
                match num_props {
                    1 => base_rent,
                    2 => base_rent * 2,
                    3 => base_rent * 4,
                    4 => base_rent * 8,
                    _ => unreachable!(),
                }
            },
            ColorGroup::Utility => {
                match num_props {
                    1 => base_rent * 4,
                    2 => base_rent * 10,
                    _ => unreachable!(),
                }
            },
            _ => {
                if has_monopoly {
                    if num_hotels >= 1 {
                        base_rent * num_hotels as i32 * 40
                    } else if num_houses >= 1 {
                        base_rent * num_houses as i32 * 5
                    } else {
                        base_rent * 3
                    }
                } else {
                    base_rent
                }
            },
        }
    }
    
    pub fn on_land_go_to_jail(&mut self, go_salary: i32) {
        println!("Go to jail! Go directly to jail! Do not pass \
                  GO! Do not collect ${}!", go_salary);   
        let player = self.get_current_player();
        let gtj = player.borrow().get_space();
        let jail = self.get_space(10).clone();
        gtj.borrow_mut().remove_player(player.clone());
        jail.borrow_mut().add_player(player.clone());
        player.borrow_mut().jail(self.spaces[JAIL].clone());
    }
    
    pub fn on_land_income_tax(&mut self, tax: i32) {
        println!("Income tax! Pay ${}.", tax);
        let player = self.get_current_player();
        player.borrow_mut().tax(tax);
    }
    
    pub fn on_land_luxury_tax(&mut self, tax: i32) {
        println!("Luxury tax! Pay ${}.", tax);
        let player = self.get_current_player();
        player.borrow_mut().tax(tax);
    }
    
    pub fn get_space(&self, index: usize) -> Rc<RefCell<Space>> {
        let index = self.clip_player_index(index);
        self.spaces[index].clone()
    }
    
    pub fn get_next_space(&mut self) -> Rc<RefCell<Space>> {
        let player = self.get_current_player();
        let dice_roll = get_dice_roll_12() as usize;
        println!("{} rolled a {}.",
                 player.borrow().get_name(),
                 dice_roll);
        let old_player_index = self.get_player_index();
        let new_raw_index = old_player_index + dice_roll;
        if new_raw_index >= self.spaces.len() {
            self.passed_go = true;
        }
        let new_player_index = self.clip_player_index(new_raw_index);
        let new_space = self.get_space(new_player_index);
        new_space.clone()
    }
    
    pub fn advance_to(&mut self, new_space: Rc<RefCell<Space>>) -> LandAction {
        let player = self.get_current_player();
        let old_space = player.borrow().get_space();
        old_space.borrow_mut().remove_player(player.clone());
        new_space.borrow_mut().add_player(player.clone());
        
        if self.passed_go {
            self.handle_pass_go();
        }
        let mut player = player.borrow_mut();
        player.land(new_space)
    }
    
    pub fn roll_and_land(&mut self) -> LandAction {
        let space = self.get_next_space();
        self.advance_to(space.clone())
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
    
    pub fn clip_player_index(&self, index: usize) -> usize {
        index % self.spaces.len()
    }
    
    pub fn get_player_index(&self) -> usize {
        let player = self.get_current_player();
        let space = player.borrow().get_space();
        let space = space.borrow(); // why Rust?? why??
        space.get_index()
    }
    
    pub fn reset_spaces(&mut self) {  
        let go = Space::new(SpaceEnum::Go, 522, 520, GO);
        let med_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Mediterranean Avenue".to_string(),
                                  60,
                                  2,
                                  ColorGroup::DarkPurple)))), 472, 520, MED_AVE);
        let comm_chest_bot = Space::new(SpaceEnum::CommunityChest, 426, 520, COMM_CHEST_BOT);
        let balt_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Baltic Avenue".to_string(),
                                  60,
                                  4,
                                  ColorGroup::DarkPurple)))), 376, 520, BALT_AVE);  
        let income_tax = Space::new(SpaceEnum::IncomeTax, 328, 520, INCOME_TAX);
        let reading_rr = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Reading Railroad".to_string(),
                                  150,
                                  25,
                                  ColorGroup::Railroad)))), 280, 520, READING_RR);    
        let orient_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Oriental Avenue".to_string(),
                                  100,
                                  6,
                                  ColorGroup::LightBlue)))), 231, 520, ORIENT_AVE);
        let chance_bot = Space::new(SpaceEnum::Chance, 183, 520, CHANCE_BOT);
        let verm_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Vermont Avenue".to_string(),
                                  100,
                                  6,
                                  ColorGroup::LightBlue)))), 134, 520, VERM_AVE);          
        let conn_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Connecticut Avenue".to_string(),
                                  120,
                                  8,
                                  ColorGroup::LightBlue)))), 88, 520, CONN_AVE);  
        let jail = Space::new(SpaceEnum::Jail, 4, 520, JAIL);                                  
        let st_char_pl = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "St. Charles Place".to_string(),
                                  140,
                                  10,
                                  ColorGroup::LightPurple)))), 4, 472, ST_CHAR_PL);
        let elec_util = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Electric Company".to_string(),
                                  150,
                                  8,
                                  ColorGroup::Utility)))), 4, 424, ELEC_UTIL);                                  
        let states_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "States Avenue".to_string(),
                                  140,
                                  10,
                                  ColorGroup::LightPurple)))), 4, 376, STATES_AVE);   
        let va_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Virginia Avenue".to_string(),
                                  160,
                                  12,
                                  ColorGroup::LightPurple)))), 4, 327, VA_AVE);       
        let pa_rr = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Pennsylvania Railroad".to_string(),
                                  150,
                                  25,
                                  ColorGroup::Railroad)))), 4, 280, PA_RR);   
        let st_james_pl = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "St. James Place".to_string(),
                                  180,
                                  14,
                                  ColorGroup::Orange)))), 4, 230, ST_JAMES_PL);      
        let comm_chest_left = Space::new(SpaceEnum::CommunityChest, 4, 182, COMM_CHEST_LEFT);
        let tn_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Tennessee Avenue".to_string(),
                                  180,
                                  14,
                                  ColorGroup::Orange)))), 4, 133, TN_AVE);       
        let ny_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "New York Avenue".to_string(),
                                  200,
                                  16,
                                  ColorGroup::Orange)))), 4, 85, NY_AVE);     
        let free_parking = Space::new(SpaceEnum::FreeParking, 4, 4, FREE_PARKING);
        let ky_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Kentucky Avenue".to_string(),
                                  220,
                                  18,
                                  ColorGroup::Red)))), 88, 4, KY_AVE);     
        let chance_top = Space::new(SpaceEnum::Chance, 135, 4, CHANCE_TOP);
        let in_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Indiana Avenue".to_string(),
                                  220,
                                  18,
                                  ColorGroup::Red)))), 184, 4, IN_AVE);       
        let il_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Illinois Avenue".to_string(),
                                  240,
                                  20,
                                  ColorGroup::Red)))), 232, 4, IL_AVE);       
        let bo_rr = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "B&O Railroad".to_string(),
                                  150,
                                  25,
                                  ColorGroup::Railroad)))), 280, 4, BO_RR);                                   
        let atl_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Atlantic Avenue".to_string(),
                                  260,
                                  22,
                                  ColorGroup::Yellow)))), 328, 4, ATL_AVE);      
        let ventnor_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Ventnor Avenue".to_string(),
                                  260,
                                  22,
                                  ColorGroup::Yellow)))), 377, 4, VENTNOR_AVE);      
        let water_util = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Water Works".to_string(),
                                  150,
                                  8,
                                  ColorGroup::Utility)))), 425, 4, WATER_UTIL);  
        let mar_gard = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Marvin Gardens".to_string(),
                                  280,
                                  22,
                                  ColorGroup::Yellow)))), 474, 4, MAR_GARD);      
        let go_to_jail = Space::new(SpaceEnum::GoToJail, 522, 4, GO_TO_JAIL);
        let pac_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Pacific Avenue".to_string(),
                                  300,
                                  26,
                                  ColorGroup::Green)))), 522, 85, PAC_AVE);     
        let nc_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "North Carolina Avenue".to_string(),
                                  300,
                                  26,
                                  ColorGroup::Green)))), 522, 133, NC_AVE);       
        let comm_chest_right = Space::new(SpaceEnum::CommunityChest, 522, 181, COMM_CHEST_RIGHT);
        let pa_ave = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Pennsylvania Avenue".to_string(),
                                  320,
                                  28,
                                  ColorGroup::Green)))), 522, 230, PA_AVE);    
        let sl_rr = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Short Line".to_string(),
                                  150,
                                  25,
                                  ColorGroup::Railroad)))), 522, 279, SL_RR);      
        let chance_right = Space::new(SpaceEnum::Chance, 522, 327, CHANCE_RIGHT);
        let park_pl = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Park Place".to_string(),
                                  350,
                                  35,
                                  ColorGroup::DarkBlue)))), 522, 375, PARK_PL); 
        let luxury_tax = Space::new(SpaceEnum::LuxuryTax, 522, 424, LUXURY_TAX);
        let bdwk = Space::new(SpaceEnum::Prop ( Rc::new(RefCell::new(Property::new(
                                  "Boardwalk".to_string(),
                                  400,
                                  50,
                                  ColorGroup::DarkBlue)))), 522, 472, BDWK);      

        self.spaces = Vec::new();
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
    
    pub fn add_player(&mut self, player: Rc<RefCell<Player>>) {
        self.players.push(player.clone());
    }
    
    /// Returns the player whose turn is currently up
    pub fn get_current_player(&self) -> Rc<RefCell<Player>> {
        self.players[self.player_turn].clone()
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
    }
}


/*
 *  UTILITY FUNCTIONS
 */

pub fn get_token_color() -> usize {
    loop {
        let color = get_string();
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
        let input = get_string();
        match &(*input.trim().to_lowercase()) {
            "yes" => return true,
            "no"  => return false,
            _ => print!("Please enter yes or no: "),
        }
    }
}

pub fn get_turn_action() -> TurnCommand {
    loop {
        let input = get_string();
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
            Err(_) => print!("Please enter an integer: "),
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

pub fn get_dice_roll_6() -> i32 {
    let mut rng = rand::thread_rng();
    let result: i32 = rng.gen_range(1, 7);
    result
}

/// Simulate a dice roll: return an integer between 2 and 12, inclusive
pub fn get_dice_roll_12() -> i32 {
    let mut rng = rand::thread_rng();
    let first: i32 = rng.gen_range(1, 7); // [1, 7)
    let second: i32 = rng.gen_range(1, 7);
    first + second
}
