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


pub const WINDOW_WIDTH: u32 = 600;
pub const WINDOW_HEIGHT: u32 = 600;

const GO_SALARY: u32 = 200;
const INCOME_TAX: u32 = 200;
const LUXURY_TAX: u32 = 75;

/// Represents the different stages in a player's turn
#[derive(Debug, PartialEq, Clone)]
pub enum TurnState {
    StartTurn,
    WaitingForCommand,
    ExecutingCommand,
    AfterCommand,
    ConfirmQuit,
    ConfirmPurchase(Rc<RefCell<Property>>),
    ConfirmPlayAgain,
}

/// Represents a player's choice of action during their turn
#[derive(Debug, PartialEq, Clone)]
pub enum TurnCommand {
    Roll,
    Quit,
    Assets,
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
}

impl Game {
    pub fn new() -> Game {
        let opengl = OpenGL::V3_2;
        let mut window: GlutinWindow = WindowSettings::new(
            "Rust Monopoly",
            [WINDOW_WIDTH, WINDOW_HEIGHT]
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
        }
    }
    
    pub fn setup_game(&mut self) {
        let mut input = String::new();
    
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
            let mut name = get_string();
            
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
            let mut n = get_dice_roll();
            while turns_to_players.contains_key(&n) {
                n = get_dice_roll();
            }
            
            
            let mut go = self.board.get_space(0);
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
                }
            },
            Key::Q => {
                if self.turn_state == TurnState::WaitingForCommand {
                    self.turn_state = TurnState::ExecutingCommand;
                    self.turn_command = Some(TurnCommand::Quit);
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
                                self.turn_state = TurnState::WaitingForCommand;
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
            _ => self.turn_command = None,
        }
    }
    
    pub fn handle_land_space(&mut self, space: Rc<RefCell<Space>>) {
        let t = {
            let space = space.borrow();
            space.get_type().clone()
        };
            
        match t {
            SpaceEnum::Prop(_) => unreachable!(),
            SpaceEnum::Go => self.board.on_land_go(GO_SALARY),
            SpaceEnum::Chance => self.board.on_land_chance(),
            SpaceEnum::CommunityChest => self.board.on_land_comm_chest(),
            SpaceEnum::Jail => self.board.on_land_jail(),
            SpaceEnum::FreeParking => self.board.on_land_free_parking(),
            SpaceEnum::GoToJail => self.board.on_land_go_to_jail(GO_SALARY),
            SpaceEnum::IncomeTax => self.board.on_land_income_tax(INCOME_TAX),
            SpaceEnum::LuxuryTax => self.board.on_land_luxury_tax(LUXURY_TAX),
        }
    }
    
    pub fn handle_land(&mut self, action: LandAction) {
        match action {
            LandAction::Rent(ref prop) => {
                let property = prop.borrow();
                let owner = property.get_owner();
                
                println!("{} is owned by {}. Pay rent of ${}!", 
                         property.get_name(),
                         owner.borrow().get_name(), 
                         property.get_rent().unwrap());
                self.board.on_rent_collected(owner.clone(), &*property);
                
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
                self.handle_land_space(space.clone());
                self.turn_state = TurnState::AfterCommand;
                self.turn_command = None;
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
                            self.board.start_turn();
                            self.turn_state = TurnState::WaitingForCommand
                        },
                        TurnState::WaitingForCommand => {
                        
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
                                        self.turn_state = TurnState::WaitingForCommand;
                                    },
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
