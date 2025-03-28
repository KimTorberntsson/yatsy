use std::process::exit;

use super::dice;
use super::input;
use super::dice_result;
use super::scores;

pub struct Game {
    pub rerolls: i32,
    pub dice: dice::DiceRoll,
    pub score_card: scores::ScoreCard
}

impl Game {
    pub fn new() -> Game {
        Game {
            rerolls: 2,
            dice: dice::DiceRoll::new(),
            score_card: scores::ScoreCard::new()
        }
    }

    pub fn start(&mut self) {
        self.print_welcome();
        self.start_round();
    }

    pub fn print_welcome(&self) {
        println!("\n--- Welcome to Command Line Yatsy!---");
        self.print_help();
    }

    fn print_help(&self) {
        println!("Enter \"r\" followed by indices to reroll dice. Indices are 1 based and separated by spaces.");
        println!("Enter \"p\" to pick a result from the available options.");
        println!("Enter \"s\" to show the current score card.");
        println!("Enter \"q\" to quit or \"reset\" to start a new game.");
        println!("Enter \"h\" to show this help message.")
    }

    fn print_state(&self) {
        println!("\nDice: {}, Rerolls left: {}", self.dice, self.rerolls);
    }

    fn start_round(&mut self) {
        self.check_end();
        self.reset_round();
        self.get_command();
    }

    fn check_end(&self) {
        if self.score_card.is_complete() {
            println!("\n--- Game over! Thanks for playing! ---");
            exit(0);
        }
    }

    fn reset_round(&mut self) {
        self.rerolls = 2;
        self.dice = dice::DiceRoll::new();
    }

    fn reset_game(&mut self) {
        // Reset terminal
        print!("{}[2J", 27 as char);

        self.score_card = scores::ScoreCard::new();
        self.reset_round();

        self.start();
    }

    fn get_command(&mut self) {
        self.print_state();

        match input::get_command() {
            input::Command::Reroll(indices) => self.handle_reroll(indices),
            input::Command::Pick => self.handle_pick(),
            input::Command::Quit => exit(0),
            input::Command::Reset => self.reset_game(),
            input::Command::ShowScores => {
                self.score_card.print_scores(false);
                self.get_command();
            }
            input::Command::Help => {
                self.print_help();
                self.get_command();
            },
        }
    }

    fn handle_pick(&mut self) {
        let available_types = self.score_card.get_available_types();
        let results = dice_result::get_results( self.dice.clone() );
        let available_results = results.iter().filter(|&&r| available_types.contains(&r.result_type)).collect::<Vec<_>>();
        if available_results.len() == 0 {
            self.handle_strike();
        }
        
        println!("Pick a result:");
        for i in 0..available_results.len() {
            println!("{}: {}", i + 1, available_results[i]);
        }
        println!("{}: Strike row", available_results.len() + 1);

        let pick = input::get_pick();
        if pick > available_results.len() {
            println!("Invalid selection. Try again.");
            self.handle_pick();
        }

        if pick == available_results.len() {
            self.handle_strike();
            return;
        }

        self.score_card.add_result(available_results[pick].clone());

        self.score_card.print_scores(true);
        self.start_round();
    }

    fn handle_strike(&mut self) {
        println!("Strike a result:");
        let available_types = self.score_card.get_available_types();
        for i in 0..available_types.len() {
            println!("{}: {}", i, available_types[i]);
        }

        let index = input::get_pick();
        if index >= available_types.len() {
            println!("Invalid selection. Try again.");
            self.handle_strike();
        }

        self.score_card.strike(available_types[index]);

        self.score_card.print_scores(true);
        self.start_round();
    }

    fn handle_reroll(&mut self, indices: Vec<usize>) {
        match self.get_reroll(indices) {
            Ok(indices) => {
                let new_dice = self.dice.reroll(indices);
                self.rerolls = self.rerolls - 1;
                self.dice = new_dice;
            },
            Err(e) => {
                println!("{}", e);
            }
        }
        self.get_command();
    }

    fn get_reroll(&self, indices: Vec<usize>) -> Result<Vec<usize>, String> {
        if self.rerolls == 0 {
            return Err("You have no rerolls left!".to_string())
        }

        if (indices.len() == 0) || (indices.len() > 5) {
            return Err("You must reroll between 1 and 5 dice!".to_string())
        }

        return Ok(indices);
    }
}