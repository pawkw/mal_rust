mod reader;
mod maltype;
mod malerror;
use malerror::MalError;
use maltype::MalType;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
use reader::read_str;

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


fn READ(input: &String) -> MalType {
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

fn EVAL(input: &MalType) -> MalType {
    let output: MalType = input.clone();
    output
}

fn PRINT(input: &MalType) -> String {
    fn get_list(input: &Vec<MalType>) -> String {
        let mut string_list = vec![];
            for item in input {
                string_list.push(PRINT(item));
            }
            string_list.join(" ")
    }

    match input {
        MalType::MalSymbol(x) => { String::from(x) },
        MalType::MalList(x) => {
            "(".to_string()+&get_list(x)+&")".to_string()
        },
        MalType::MalHashmap(x) => {
            "{".to_string()+&get_list(x)+&"}".to_string()
        },
        MalType::MalVec(x) => {
            "[".to_string()+&get_list(x)+&"]".to_string()
        },
        MalType::MalNil => { String::from("nil") },
        MalType::MalString(x) => {
            x.into()
        }
    }
}

fn rep(input: &String) -> String {
    let output: String = PRINT(&EVAL(&READ(&input)));
    output
}
