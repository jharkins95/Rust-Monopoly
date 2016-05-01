//
//! The Game struct stores the entirety of the game state, including the
//! game board, whether the game is running or not, 
//! the current state of a player's turn, and the player's command.
//! It also stores the window in which game objects are drawn.
//!
//! The game proceeds as follows:
//! 1. Player prompted for action.
//! 2. Player enters key corresponding to action in the console.
//! 3. Game executes the player's action.
//! 4. If the game requires a response to the action (such as landing on
//!    unowned property), the player will be prompted to respond by
//!    typing a key in the window.
//! 5. After the player's turn is finished, the game will check if they
//!    are bankrupt. If so, the player's assets will be transferred to
//!    the creditor (or the bank if there is no creditor). The bankrupt
//!    player will no longer be able to trade properties, collect rents,
//!    or otherwise participate in the game.
//! 6. The last remaining player wins the game.
//!
//! During the course of the game, the Game will notify its Board that 
//! the next player is ready to begin his turn via board.start_turn().
//! When the player lands, the Board will return a LandAction to the Game,
//! representing the type of action taken upon landing (buying/selling/
//! renting/etc); see LandAction for more details.
//!

extern crate opengl_graphics;
extern crate piston;
extern crate glutin_window;

use std::collections::BTreeMap;
use glutin_window::GlutinWindow;
use std::process;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use std::rc::Rc;
use std::cell::RefCell;
use opengl_graphics::{GlGraphics, OpenGL};

use super::board::*;
use super::player::*;
use super::property::*;
use super::space::*;


pub const WINDOW_WIDTH: i32 = 600;
pub const WINDOW_HEIGHT: i32 = 600;

pub const GO_SALARY: i32 = 200;
pub const INCOME_TAX_AMT: i32 = 200;
pub const LUXURY_TAX_AMT: i32 = 75;

/// Objects that can be drawn to the screen with
/// the Piston/OpenGL framework
pub trait Render {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs);
}

/// Represents the different stages in a player's turn
#[derive(Debug, PartialEq, Clone)]
pub enum TurnState {
    StartTurn,
    WaitingForCommand,
    StartWaitingForCommand,
    InJail,
    ExecutingCommand,
    AfterCommand,
    ConfirmQuit,
    ConfirmPurchase(Rc<RefCell<Property>>),
    ConfirmPlayAgain,
    ConfirmBuySellHouseHotel,
    EnterPropIndex,
    ValidatePropIndex,
    BuyHouseHotel,
    SellHouseHotel,
}

/// Represents a player's choice of action during their turn
#[derive(Debug, PartialEq, Clone)]
pub enum TurnCommand {
    Roll,
    Quit,
    Assets,
    PayJailFine,
    UseJailCard,
    HouseHotel,
    Trade,
    // TODO: add more types of actions (trades, buy/sell houses)
}

/// Master game state: is the game running/set up/over?
#[derive(Debug, PartialEq, Clone)]
pub enum GameState {
    GameGUISetup,
    GameStateSetup,
    GameRun,
    GameOver,
}

pub struct Game {
    main_window: GlutinWindow,
    gl: GlGraphics,
    board: Board,
    game_state: GameState,
    turn_state: TurnState,
    turn_command: Option<TurnCommand>,
    key_queue: Vec<u8>,
}

impl Game {
    pub fn new() -> Game {
        let opengl = OpenGL::V3_2;
        let window: GlutinWindow = WindowSettings::new(
            "Rust Monopoly",
            [WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
        Game {
            main_window: window,
            gl: GlGraphics::new(opengl),
            board: Board::new(),
            game_state: GameState::GameGUISetup,
            turn_state: TurnState::StartTurn,
            turn_command: None,
            key_queue: Vec::new(),
        }
    }
    
    fn reset_state(&mut self) {
        self.board = Board::new();
        self.game_state = GameState::GameGUISetup;
        self.turn_state = TurnState::StartTurn;
        self.turn_command = None;
        self.key_queue = Vec::new();
    }
    
    pub fn setup_game(&mut self) {
        self.reset_state();
        self.board.reset_spaces();
        
        println!("Welcome to Monopoly!");
        print!("How many players today? ");
        
        let num_players = get_num_players();
        let mut available_colors = vec![true, true, true, true, true, true];
        let colors = vec![RED, ORANGE, YELLOW, GREEN, BLUE, PURPLE];
        
        let mut turns_to_players: BTreeMap<i32, Rc<RefCell<Player>>> 
            = BTreeMap::new();
        for i in 0..num_players {
            print!("Please enter Player {}'s name: ", i + 1); 
            let mut name;

            loop {
                name = get_string();
                let mut valid_name: bool = true;
                for (_, player) in &turns_to_players {
                    if name == player.borrow().get_name() {
                        println!("That name is already chosen! Pick another name");
                        valid_name = false;
                    }
                }
                if valid_name {
                    break;
                }
               
            }
                
            
            let mut color;
            print!("Choose a color (ROYGBV): ");
            loop {
                color = get_token_color();
                if available_colors[color] {
                    available_colors[color] = false;
                    break;
                } else {
                    print!("That color is already chosen! Pick another color: ");
                }
            }
            let mut n = get_dice_roll_12();
            while turns_to_players.contains_key(&n) {
                n = get_dice_roll_12();
            }
            
            
            let go = self.board.get_space(GO);
            let player = Rc::new(RefCell::new(
                Player::new(name.trim().to_string(), 
                            go.clone(), 
                            colors[color])));
            go.borrow_mut().add_player(player.clone());
            turns_to_players.insert(n, player.clone());
        }

        for (_, player) in turns_to_players {
            self.board.add_player(player);
        }
        
        println!("Game setup complete.\n");
        
    }
    
    /// Clear the screen
    fn clear(&mut self, args: &RenderArgs) {
        use graphics::*;
        let ref mut gl = self.gl;
        const WHITE: [f32; 4] = [1.0; 4];
    
        gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);
        });
    }

    // Update the game state based on the key pressed
    fn handle_key_input(&mut self, key: keyboard::Key) {
        //println!("Key pressed = {:?}", key);
        match key {
            Key::R => {
                if self.turn_state == TurnState::WaitingForCommand {
                    self.turn_state = TurnState::ExecutingCommand;
                    self.turn_command = Some(TurnCommand::Roll);
                } else if self.turn_state == TurnState::InJail {
                    self.turn_command = Some(TurnCommand::Roll);
                }
            },
            Key::C => {
                if self.turn_state == TurnState::InJail {
                    unimplemented!();
                }
            },
            Key::Q => {
                if self.turn_state == TurnState::WaitingForCommand {
                    self.turn_state = TurnState::ExecutingCommand;
                    self.turn_command = Some(TurnCommand::Quit);
                }
            },
            Key::B => {
                if self.turn_state == TurnState::ConfirmBuySellHouseHotel {
                    self.turn_state = TurnState::BuyHouseHotel;
                }
            },
            Key::S => {
                if self.turn_state == TurnState::ConfirmBuySellHouseHotel {
                    self.turn_state = TurnState::SellHouseHotel;
                }
            },
            Key::Y => {
                match self.game_state.clone() {
                    GameState::GameOver => {
                        self.game_state = GameState::GameStateSetup;
                        self.turn_state = TurnState::StartTurn;
                        self.turn_command = None;
                    },
                    GameState::GameRun => {
                        match self.turn_state.clone() {
                            TurnState::ConfirmQuit => {
                                println!("Goodbye!");
                                process::exit(0);
                            },
                            TurnState::ConfirmPurchase(ref mut prop) => {
                                self.board.on_purchase(prop.clone());
                                self.turn_state = TurnState::AfterCommand;
                                self.turn_command = None;
                            },
                            _ => (),
                        };
                    },
                    _ => (),
                };
            },
            Key::N => {
                match self.game_state.clone() {
                    GameState::GameOver => {
                        process::exit(0);
                    },
                    GameState::GameRun => {
                        match self.turn_state {
                            TurnState::ConfirmQuit => {
                                self.turn_state = TurnState::StartWaitingForCommand;
                                self.turn_command = None;
                            },
                            TurnState::ConfirmPurchase(_) => {
                                self.turn_state = TurnState::AfterCommand;
                                self.turn_command = None;
                            },
                            _ => (),
                        };
                    },
                    _ => (),
                };
            },
            Key::A => {
                if self.turn_state == TurnState::WaitingForCommand {
                    self.turn_state = TurnState::ExecutingCommand;
                    self.turn_command = Some(TurnCommand::Assets);
                }
            },
            Key::P => {
                if self.turn_state == TurnState::InJail {
                    let player = self.board.get_current_player();
                    let cash = player.borrow().get_cash();
                    if cash >= 50 {
                        self.turn_command = Some(TurnCommand::PayJailFine);
                    } else {
                        println!("You don't have enough money! Choose another option.");
                    }
                }
            },
            Key::H => {
                if self.turn_state == TurnState::WaitingForCommand {
                    self.turn_state = TurnState::ExecutingCommand;
                    self.turn_command = Some(TurnCommand::HouseHotel);
                }
            },
            Key::D0 |
            Key::D1 |
            Key::D2 |
            Key::D3 |
            Key::D4 |
            Key::D5 |
            Key::D6 |
            Key::D7 |
            Key::D8 |
            Key::D9   => {
                if self.turn_state == TurnState::EnterPropIndex {
                    self.key_queue.push(key as u8);
                }
            },
            Key::Return => {
                if self.turn_state == TurnState::EnterPropIndex {
                    self.turn_state = TurnState::ValidatePropIndex;
                }
            },
            _ => self.turn_command = None,
        }
    }
    
    pub fn handle_land_space(&mut self, space: Rc<RefCell<Space>>) {
        let t = { // due to Rust's pedantic borrowing system...
            let space = space.borrow();
            space.get_type().clone()
        };
            
        match t {
            SpaceEnum::Prop(_) => unreachable!(),
            SpaceEnum::Go => self.board.on_land_go(GO_SALARY),
            SpaceEnum::Chance => {
                let action = self.board.on_land_chance();
                //self.turn_state = TurnState::ExecutingCommand;
                println!("turn_state = {:?}", self.turn_state);
                println!("turn_command = {:?}", self.turn_command);
                self.handle_land(action);
            },
            SpaceEnum::CommunityChest => self.board.on_land_comm_chest(),
            SpaceEnum::Jail => self.board.on_land_jail(),
            SpaceEnum::FreeParking => self.board.on_land_free_parking(),
            SpaceEnum::GoToJail => self.board.on_land_go_to_jail(GO_SALARY),
            SpaceEnum::IncomeTax => self.board.on_land_income_tax(INCOME_TAX_AMT),
            SpaceEnum::LuxuryTax => self.board.on_land_luxury_tax(LUXURY_TAX_AMT),
        }
    }
    
    pub fn handle_land(&mut self, action: LandAction) {
        match action {
            LandAction::Rent(ref prop) => {
                let owner = {
                    let property = prop.borrow();
                    property.get_owner().clone()
                };    
                
                println!("{} is owned by {}. Pay rent of ${}!", 
                         prop.borrow().get_name(),
                         owner.borrow().get_name(), 
                         self.board.get_rent(prop.clone()));
                self.board.on_rent_collected(owner.clone(), prop.clone());
                
                self.turn_state = TurnState::AfterCommand;
                self.turn_command = None;
            },
            LandAction::Own(ref prop) => {
                println!("You already own {}.", prop.borrow().get_name());
                
                self.turn_state = TurnState::AfterCommand;
                self.turn_command = None;
            },
            LandAction::InsFunds(ref prop) => {
                println!("You don't have enough money to purchase {}!",
                            prop.borrow().get_name());
                            
                self.turn_state = TurnState::AfterCommand;
                self.turn_command = None;
            },
            LandAction::MightPurchase(ref prop) => {
                println!("{} is not owned. Would you like to buy it for ${}?",
                                 prop.borrow().get_name(),
                                 prop.borrow().get_purchase_price());
                
                self.turn_state = TurnState::ConfirmPurchase(prop.clone());
                self.turn_command = None;
            },
            LandAction::Space(ref space) => {
                self.turn_state = TurnState::AfterCommand;
                self.turn_command = None;
                self.handle_land_space(space.clone());
            },
        }
    }
    
    /// The main event loop
    pub fn run(&mut self) {
        let mut events = self.main_window.events();
        while let Some(e) = events.next(&mut self.main_window) {
            //println!("Updated game state");
            match self.game_state {
                GameState::GameGUISetup => {},
            
                GameState::GameStateSetup => {
                    // TODO: clear game window
                    self.setup_game();
                    self.game_state = GameState::GameRun;
                },
                
                GameState::GameRun => {
                    match self.turn_state {
                        TurnState::StartTurn => {
                            print!("{}[2J", 27 as char); // clear screen
                            self.board.start_turn();
                            let player = self.board.get_current_player();
                            let player = player.borrow();
                            if player.is_in_jail() {
                                println!("You are in jail! You can try to roll doubles(R) or \
                                    pay $50(P).");
                                self.turn_state = TurnState::InJail;
                            } else {
                                self.turn_state = TurnState::StartWaitingForCommand;
                            }
                        },
                        TurnState::StartWaitingForCommand => {
                            println!("");
                            println!("**************************************************");
                            println!("Please enter a command in the drawing window:");
                            println!("roll(R)");
                            println!("quit(Q)");
                            println!("assets(A)");
                            println!("houses(H)");
                            println!("**************************************************");
                            println!(">> ");
                            self.turn_state = TurnState::WaitingForCommand;
                        },
                        TurnState::WaitingForCommand => {
                            // do nothing while waiting
                        },
                        TurnState::ExecutingCommand => {
                            if let Some(command) = self.turn_command.clone() {;
                                match command {
                                    TurnCommand::Roll => {  
                                        let action = self.board.roll_and_land();
                                        self.handle_land(action);
                                    },
                                    
                                    TurnCommand::Quit => {
                                        println!("Are you sure you want to quit? ");
                                        self.turn_state = TurnState::ConfirmQuit;
                                    },
                                    
                                    TurnCommand::Assets => {
                                        self.board.print_player_assets();
                                        self.turn_state = TurnState::StartWaitingForCommand;
                                    },
                                    
                                    TurnCommand::HouseHotel => {
                                        let player = self.board.get_current_player();
                                        let monopolies = player.borrow().get_monopolies();
                                        if monopolies.len() == 0 {
                                            println!("You have no monopolies on which you \
                                                      can place houses/hotels.");
                                            self.turn_state = TurnState::StartWaitingForCommand;
                                        } else {
                                            println!("Enter property index, then press ENTER:");
                                            let mut index = 0;
                                            for prop in monopolies {
                                                println!("{}: {}", index, prop.borrow().get_name());
                                                index += 1;
                                            }
                                            self.key_queue = Vec::new();
                                            self.turn_state = TurnState::EnterPropIndex;
                                        }
                                    },
                                    
                                    _ => (),
                                };   
                                self.turn_command = None;
                            };
                        },
                        
                        TurnState::InJail => {
                            if let Some(command) = self.turn_command.clone() {;
                                match command {
                                    TurnCommand::PayJailFine => {  
                                        let player = self.board.get_current_player();
                                        println!("{} paid $50.", player.borrow().get_name());
                                        player.borrow_mut().tax(50);
                                        player.borrow_mut().unjail();
                                        self.turn_state = TurnState::StartWaitingForCommand;
                                        self.turn_command = None;
                                    },
                                    
                                    TurnCommand::Roll => {
                                        let player = self.board.get_current_player();
                                        let first = get_dice_roll_6();
                                        let second = get_dice_roll_6();
                                        if first == second {
                                            println!("{} rolled doubles and is now free!", player.borrow().get_name());
                                            player.borrow_mut().unjail();
                                            self.turn_state = TurnState::StartWaitingForCommand;
                                            self.turn_command = None;
                                        } else {
                                            println!("{} did not roll doubles and remains in jail!", player.borrow().get_name());
                                            self.turn_state = TurnState::AfterCommand;
                                            self.turn_command = None;
                                        }
                                    },
                                    
                                    TurnCommand::UseJailCard => {
                                        let player = self.board.get_current_player();
                                        println!("{} used a GOOJFC.", player.borrow().get_name());
                                        player.borrow_mut().unjail();
                                        self.turn_state = TurnState::StartWaitingForCommand;
                                        self.turn_command = None;
                                    },
                                    
                                    _ => (),
                                };   
                                self.turn_command = None;
                            };
                        },
                        
                        TurnState::AfterCommand => {
                            self.board.handle_bankruptcy();
                            if self.board.get_num_remaining_players() == 1 {
                                self.game_state = GameState::GameOver;
                            }
                            self.board.end_turn();
                            self.turn_state = TurnState::StartTurn;
                            self.turn_command = None;
                        },
                        
                        TurnState::ValidatePropIndex => {
                            let index_str = String::from_utf8(self.key_queue.clone()).unwrap();
                            if let Ok(index) = index_str.parse::<usize>() {
                                let player = self.board.get_current_player();
                                let monopolies = player.borrow().get_monopolies();
                                if index >= monopolies.len() {
                                    println!("Index must be within range!");
                                    self.turn_state = TurnState::StartWaitingForCommand;
                                    self.turn_command = None;
                                } else {
                                    println!("Buy(B) or sell(S)?");
                                    self.turn_state = TurnState::ConfirmBuySellHouseHotel;
                                }
                            } else {
                                println!("Index must be an integer!");
                                self.turn_state = TurnState::StartWaitingForCommand;
                                self.turn_command = None;
                            }
                        },
                        
                        TurnState::BuyHouseHotel => {
                            let index_str = String::from_utf8(self.key_queue.clone()).unwrap();
                            let index = index_str.parse::<usize>().unwrap();
                            let player = self.board.get_current_player();
                            let monopolies = player.borrow().get_monopolies();
                            let prop = monopolies[index].clone();
                            
                            let num_houses = {
                                let prop = prop.borrow();
                                prop.get_num_houses()
                            };
                            let num_hotels = {
                                let prop = prop.borrow();
                                prop.get_num_hotels()
                            };
                            let cash = {
                                let player = player.borrow();
                                player.get_cash()
                            };
                            
                            if num_hotels >= MAX_NUM_HOTELS {
                                println!("{} cannot be further improved!",
                                          prop.borrow().get_name());
                            } else if num_houses >= MAX_NUM_HOUSES {
                                if cash < HOTEL_COST as i32 {
                                    println!("You cannot afford another hotel!");
                                } else {
                                    println!("Bought a hotel on {}!",
                                    prop.borrow().get_name());
                                    for _ in 0..4 {
                                        prop.borrow_mut().remove_house();
                                    }
                                    prop.borrow_mut().add_hotel();
                                    player.borrow_mut().tax(HOTEL_COST);
                                }
                            } else {
                                if cash < HOUSE_COST as i32 {
                                    println!("You cannot afford another house!");
                                } else {
                                    println!("Bought a house on {}!",
                                              prop.borrow().get_name());
                                    prop.borrow_mut().add_house();
                                    player.borrow_mut().tax(HOUSE_COST);
                                }          
                            }
                            self.turn_state = TurnState::StartWaitingForCommand;
                            self.turn_command = None;
                        },
                        
                        TurnState::SellHouseHotel => {
                            let index_str = String::from_utf8(self.key_queue.clone()).unwrap();
                            let index = index_str.parse::<usize>().unwrap();
                            let player = self.board.get_current_player();
                            let monopolies = player.borrow().get_monopolies();
                            let prop = monopolies[index].clone();
                            
                            let num_hotels = {
                                let prop = prop.borrow();
                                prop.get_num_hotels()
                            };
                            let num_houses = {
                                let prop = prop.borrow();
                                prop.get_num_houses()
                            };
                            
                            if num_hotels >= 1 {
                                println!("Sold a hotel on {}!", prop.borrow().get_name());
                                prop.borrow_mut().remove_hotel();
                                player.borrow_mut().salary(HOTEL_COST / 2);
                                let new_num_hotels = {
                                    prop.borrow().get_num_hotels()
                                };
                                if new_num_hotels == 0 {
                                    for _ in 0..4 {
                                        prop.borrow_mut().add_house();
                                    }
                                }
                            } else if num_houses >= 1 {
                                println!("Sold a house on {}!", prop.borrow().get_name());
                                prop.borrow_mut().remove_house();
                                let player = self.board.get_current_player();
                                player.borrow_mut().salary(HOUSE_COST / 2);
                            } else {
                                println!("No houses to remove on {}!", prop.borrow().get_name());
                            }
                            self.turn_state = TurnState::StartWaitingForCommand;
                            self.turn_command = None;
                        },
                        
                        _ => (),
                    };
                },
                
                GameState::GameOver => {
                    match self.turn_state {
                        TurnState::ConfirmPlayAgain => (),
                        _ => {
                            let winner = self.board.get_winner().unwrap();
                            println!("{} has won the game!",
                                winner.borrow().get_name());
                            println!("{} has ${} and the following assets:",
                                winner.borrow().get_name(),
                                winner.borrow().get_cash());
                            winner.borrow().print_assets();
                    
                            println!("Play again?");
                            self.turn_state = TurnState::ConfirmPlayAgain;
                    
                        },
                    };
                },
            }
            
            if let Some(r) = e.render_args() {
                self.clear(&r);
                if self.game_state == GameState::GameGUISetup {
                    self.game_state = GameState::GameStateSetup;
                } else {
                    self.board.render(&mut self.gl, &r);
                }
            }
            
            if let Some(Button::Keyboard(key)) = e.press_args() {
                self.handle_key_input(key);
            };
        }
    }
}
