use std::io;
use regex::Regex;
use itertools::Itertools;

pub enum Command {
    Reroll(Vec<usize>),
    Pick,
    Quit,
    Help,
    Reset,
    ShowScores
}

pub fn get_command() -> Command {
    loop {
        let input = get_input("");

        // Matches 'r' followed by spaces, then numbers with spaces preserved
        let re = Regex::new(r"^r\s+([\d\s]+)$").unwrap(); 
        if let Some(caps) = re.captures(&input) {
            let indices = caps.get(1).unwrap().as_str();
            let numbers = indices.split_whitespace()
            .map(|s| s.parse::<usize>())
            .filter_map(Result::ok)
            .filter(|&n| (1..6).contains(&n))
            .map(|n| n - 1)
            .unique()
            .collect();
            return Command::Reroll(numbers);
        }

        match input.as_str() {
            "p" | "pick" => return Command::Pick,
            "q" | "quit" => return Command::Quit,
            "h" | "help" => return Command::Help,
            "s" | "scores" => return Command::ShowScores,
            "reset" => return Command::Reset,
            _ => ()
        }

        println!("Invalid command. Try again.");
    }
}

pub fn get_pick() -> usize {
    loop {
        let input = get_input("");
        match input.parse::<usize>() {
            Ok(n) => return n - 1,
            Err(_) => println!("Invalid input. Try again."),
        }
    }
}

pub fn get_input(prompt: &str) -> String {
    println!("{}",prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}