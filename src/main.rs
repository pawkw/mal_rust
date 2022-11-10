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
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
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
    let output = read_str(input).unwrap();
    output
}

fn EVAL(input: &MalType) -> MalType {
    let output: MalType = input.clone();
    output
}

fn PRINT(input: &MalType) -> String {
    match input {
        MalType::MalSymbol(x) => { String::from(x) },
        MalType::MalList(x) => {
            let mut string_list = vec![];
            for item in x {
                string_list.push(PRINT(item));
            }
            ("(".to_string()+&string_list.join(" ")+")").to_string()
        }
    }
}

fn rep(input: &String) -> String {
    let output: String = PRINT(&EVAL(&READ(&input)));
    output
}
