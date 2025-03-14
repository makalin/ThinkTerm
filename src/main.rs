mod ai;
mod commands;
mod theme;

use std::collections::VecDeque;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use tokio;

const HISTORY_SIZE: usize = 100;

#[tokio::main]
async fn main() {
    let mut rl = Editor::<()>::new().unwrap();
    let mut history: VecDeque<String> = VecDeque::new();
    let theme_color = theme::get_prompt_color();
    
    println!("Welcome to ThinkTerm - The Terminal That Thinks With You!");
    loop {
        let prompt = format!("{}ThinkTerm> \x1b[0m", theme_color);
        match rl.readline(&prompt) {
            Ok(input) => {
                let input = input.trim().to_string();
                if input.is_empty() {
                    continue;
                }

                rl.add_history_entry(&input);
                history.push_back(input.clone());
                if history.len() > HISTORY_SIZE {
                    history.pop_front();
                }

                if input == "exit" {
                    println!("Goodbye!");
                    break;
                }

                if input.starts_with("ai ") {
                    match ai::ask_ai(&history, &input[3..]).await {
                        Ok(response) => println!("AI: {}", response),
                        Err(e) => eprintln!("Error calling AI: {}", e),
                    }
                } else {
                    commands::execute_command(&input);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("EOF");
                break;
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        }
    }
}
