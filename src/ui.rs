extern crate opengl_graphics;
extern crate piston;
extern crate glutin_window;

use glutin_window::GlutinWindow;
use std::process;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL};

use super::board::*;


pub const WINDOW_WIDTH: u32 = 600;
pub const WINDOW_HEIGHT: u32 = 600;

/// Represents the different points in a player's turn
#[derive(Debug, Clone)]
pub enum TurnState {
    ExecutingCommand,
    AfterCommand,
}

/// Represents a player's choice on their turn
#[derive(Debug, Clone)]
pub enum TurnCommand {
    CommandNotReady,
    Roll,
    Quit,
    Assets,
    // TODO: add more types of actions (trades, buy/sell houses)
}

/// Master game state: is the game running/set up/over?
#[derive(Debug, Clone)]
pub enum GameState {
    GameSetup,
    GameRun,
    GameOver,
}

pub struct Ui {
    main_window: GlutinWindow,
    gl: GlGraphics,
    board: Board,
    game_state: GameState,
    turn_state: TurnState,
    turn_command: Option<TurnCommand>,
}

impl Ui {
    pub fn new() -> Ui {
        let opengl = OpenGL::V3_2;
        let mut window: GlutinWindow = WindowSettings::new(
            "Rust Monopoly",
            [WINDOW_WIDTH, WINDOW_HEIGHT]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
        Ui {
            main_window: window,
            gl: GlGraphics::new(opengl),
            board: Board::new(),
            game_state: GameState::GameSetup,
            turn_state: TurnState::ExecutingCommand,
            turn_command: None,
        }
    }
    
    pub fn set_turn_command_none(&mut self) {
        self.turn_command = None;
    }

    pub fn handle_key_input(&mut self, key: keyboard::Key) {
        match key {
            Key::R => self.turn_command = Some(TurnCommand::Roll),
            Key::Q => self.turn_command = Some(TurnCommand::Quit),
            Key::A => self.turn_command = Some(TurnCommand::Assets),
            _ => self.turn_command = None,
        }
    }
    
    pub fn run(&mut self) {
        let mut events = self.main_window.events();
        while let Some(e) = events.next(&mut self.main_window) {
            match self.game_state {
                GameState::GameSetup => {
                    // TODO: clear game window
                    self.board.setup_game();
                    self.game_state = GameState::GameRun;
                },
                
                GameState::GameRun => {
                    match self.turn_state {
                        TurnState::ExecutingCommand => {
                            match self.turn_command {
                                None => (),
                                Some(_) => {
                                    let command = self.turn_command.clone();
                                    match command.unwrap() {
                                        TurnCommand::Roll => {
                                            self.board.roll_and_land();
                                            self.turn_state = TurnState::AfterCommand;
                                        },
                                        
                                        TurnCommand::Quit => {
                                            print!("Are you sure you want to quit? ");
                                            if super::board::confirm_prompt() {
                                                process::exit(0);
                                            }
                                            self.set_turn_command_none(); // if user didn't exit
                                        },
                                        
                                        TurnCommand::Assets => {
                                            self.board.print_player_assets();
                                            self.set_turn_command_none();
                                        },

                                        _ => (),
                                    };   
                                },
                            };
                        },
                        
                        TurnState::AfterCommand => {
                            // TODO: handle bankruptcy here
                            self.board.advance_to_next_turn();
                            self.turn_state = TurnState::ExecutingCommand;
                            self.turn_command = None;
                        },
                    };
                },
                
                GameState::GameOver => {
                    // TODO: prompt players here for new game; display assets of each
                },
            }
            
            if let Some(r) = e.render_args() {
                self.board.render(&mut self.gl, &r);
            }
            
            if let Some(Button::Keyboard(key)) = e.press_args() {
                self.handle_key_input(key);
            };
        }
    }
}
