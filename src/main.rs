mod reader;
mod maltype;
mod malerror;
mod printer;
use malerror::MalError;
use maltype::MalType;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
use reader::read_str;
use printer::pr_str;

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("Line history will be stored in history.txt.");
    }

    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("{}",rep(&line));
                // println!("{}", read_str(&line));
            },
            Err(ReadlineError::Interrupted) => {
                println!("Keyboard interrupt. Exiting.");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("Exiting.");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
        
    }
    rl.save_history("history.txt")
}


fn mal_read(input: &String) -> MalType {
    let output = read_str(input);
    match output {
        Ok(x) => x,
        Err(x) => {
            match x {
                MalError::ParseError => {
                    println!("A parse error occured: {}", &input);
                },
                MalError::ParenMismatch => {
                    println!("Expected a closing paren.");
                },
                MalError::TokenizingError => {
                    println!("An error occured while tokenizing: {}", &input);
                },
                MalError::TypeMismatch => {
                    println!("A type mismatch occured: {}", &input);
                },
            }
            MalType::MalNil
        }
    }
}

fn mal_eval(input: &MalType) -> MalType {
    let output: MalType = input.clone();
    output
}

fn mal_print(input: &MalType) -> String {
    pr_str(input, true)
}

fn rep(input: &String) -> String {
    let output: String = mal_print(&mal_eval(&mal_read(&input)));
    output
}
