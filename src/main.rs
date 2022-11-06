use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

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
                rep(&line);
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


fn READ(input: &String) -> String {
    let output: String = input.clone();
    output
}

fn EVAL(input: &String) -> String {
    let output: String = input.clone();
    output
}

fn PRINT(input: &String) -> String {
    let output: String = input.clone();
    println!("{}", &input);
    output
}

fn rep(input: &String) -> String {
    let output: String = PRINT(&EVAL(&READ(&input)));
    output
}
