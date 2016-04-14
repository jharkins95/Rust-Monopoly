use std;
use std::rc::Rc;
use std::cell::RefCell;
use super::property::Property;
use super::board::*;

const STARTING_CASH: i32 = 1500;

#[derive(Debug)]
pub struct Player {
    name: String,
    cash: i32,
    space: Space,
    properties: Vec<Rc<RefCell<Property>>>,
}

impl Player {
    pub fn new(name: String, start_space: Space) -> Player {
        Player {
            name: name,
            cash: STARTING_CASH,
            space: start_space,
            properties: Vec::new(),
        }
    }

    pub fn land(&mut self, space: Space) {
        match space {
            Space::Prop(ref property) => {
                if property.borrow().is_owned() {
                    let owner = property.borrow().get_owner().unwrap();
                    let mut owner = owner.borrow_mut();
                    if *self == *owner {
                        println!("You already own {}.",
                                 property.borrow().get_name());
                    } else {
                        println!("{} is owned by {}. Pay ${}!", 
                                 property.borrow().get_name(),
                                 owner.get_name(), 
                                 property.borrow().get_rent().unwrap());
                        owner.collect_rent(self, 
                                                        &*property.borrow());
                    }
                } else {
                    if self.cash < property.borrow().get_purchase_price() as i32 {
                        println!("You don't have enough money for that!");
                    } else {
                        println!("{} is not owned. Would you like to buy it?",
                                 property.borrow().get_name());
                        if confirm_prompt() {
                            println!("{} purchased {} for ${}!",
                                    self.name,
                                    property.borrow().get_name(),
                                    property.borrow().get_purchase_price());
                            self.properties.push(property.clone());
                        }
                    }
                }
            },
            Space::Go(salary) => {
                println!("You landed on GO! Collect ${}.", salary);
                self.cash += salary;
            },
            Space::Chance => unimplemented!(),
            Space::CommunityChest => unimplemented!(),
            Space::Jail => println!("Just visiting..."),
            Space::FreeParking => unimplemented!(),
            Space::GoToJail => {
                println!("Go to jail! Go directly to jail! Do not pass\
                          GO! Do not collect ${}!", GO_SALARY);
                self.space = Space::GoToJail;
                return;
            },
            Space::IncomeTax(tax) => {
                println!("INCOME TAX! Pay 10% or ${}", tax);
                self.cash -= tax;  // TODO: choose between 10% or tax
            },
            Space::LuxuryTax(tax) => {
                println!("Pay LUXURY TAX of {}!", tax);
                self.cash -= tax;
            },
        };
        self.space = space;
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_cash(&self) -> i32 {
        self.cash
    }

    pub fn is_bankrupt(&self) -> bool {
        self.cash <= 0
    }

    pub fn collect_rent(&mut self, other: &mut Player, property: &Property) {
        let rent = property.get_rent().unwrap() as i32;
        self.cash += rent;
        other.cash -= rent;
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Player) -> bool {
        self.name == other.name
    }
}
