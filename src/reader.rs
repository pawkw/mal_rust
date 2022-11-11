use crate::malerror::MalError;
use crate::maltype::MalType;
use regex::{Error, Regex};

struct Reader {
    token_vector: Vec<String>,
    cursor: usize,
}

impl Reader {
    fn new(data: Vec<String>) -> Self {
        Self {
            token_vector: data.clone(),
            cursor: 0,
        }
    }

    fn peek(&self) -> Option<&String> {
        self.token_vector.get(self.cursor)
    }

    fn read(&mut self) -> Option<&String> {
        // let result = self.peek();
        let result = self.token_vector.get(self.cursor);
        self.cursor += 1;
        result
    }
}

pub fn read_str(input: &String) -> Result<MalType, MalError> {
    let result = tokenize(&input);
    match result {
        Ok(x) => {
            let result = read_form(&mut Reader::new(x));
            Ok(result?)
        }
        _ => Err(MalError::TokenizingError),
    }
}

fn tokenize(input: &String) -> Result<Vec<String>, MalError> {
    const PATTERN: &str =
        r#"[,\s]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#;

    // let input_no_commas = &input.replace(",", " ");
    let re = Regex::new(PATTERN).unwrap();
    let tokens = re
        .captures_iter(input)
        .map(|x| x[0].trim().to_string())
        .collect();
    // dbg!(&tokens);
    match tokens {
        Vec { .. } => Ok(tokens),
        _ => Err(MalError::TokenizingError),
    }
}

#[test]
fn test_tokenize() -> Result<(), MalError> {
    let input = "(+ 1 2)".to_string();
    let result = tokenize(&input)?;
    let expect: Vec<String> = vec!["(", "+", "1", "2", ")"]
        .into_iter()
        .map(|x| x.to_string())
        .collect();
    assert_eq!(result, expect);
    Ok(())
}

fn read_form(token_reader: &mut Reader) -> Result<MalType, MalError> {
    match token_reader.peek() {
        Some(x) => {
            if x.starts_with("(") {
                token_reader.read();
                Ok(MalType::MalList(read_list(token_reader, &")")?))
            } else if x.starts_with("{") {
                token_reader.read();
                Ok(MalType::MalHashmap(read_list(token_reader, &"}")?))
            } else if x.starts_with("[") {
                token_reader.read();
                Ok(MalType::MalVec(read_list(token_reader, &"]")?))
            } else {
                Ok(read_atom(token_reader)?)
            }
        }
        None => Err(MalError::ParseError),
    }
}

fn read_list(token_reader: &mut Reader, delim: &str) -> Result<Vec<MalType>, MalError> {
    let mut token_list = vec![];
    'dave: loop {
        match token_reader.peek() {
            Some(x) => {
                if x.starts_with(delim) {
                    token_reader.read();
                    break 'dave;
                }
                let item: MalType = read_form(token_reader).unwrap();
                token_list.push(item);
            }
            None => return Err(MalError::ParenMismatch),
        }
    }
    Ok(token_list)
}

// #[test]
// fn test_read_list() -> Result<(), MalError> {
//     let input = "(+ 1 2)".to_string();

//     let token_list = tokenize(&input)?;
//     let reader = &mut Reader::new(token_list);
//     let result = read_list(reader)?;

//     let mut expected = MalType::MalList(vec![]);

//     println!("{:?} :: {:?}", result, expected);
//     Ok(())
// }

fn read_atom(token_reader: &mut Reader) -> Result<MalType, MalError> {
    loop {
        let token = token_reader.read();
        match token {
            Some(x) => {
                if x.starts_with(":") {
                    return Ok(MalType::MalKeyword(x[1..].to_string()));
                } else if x.starts_with('"') {
                    return Ok(MalType::MalString(get_string(&x)))
                }
                return Ok(MalType::MalSymbol(x.to_string()));
            }
            None => { return Err(MalError::ParseError); }
        }
    }
}

fn get_string(input: &String) -> String {
    let mut string: Vec<String> = vec![];
    let mut escaped: bool = false;
    for character in input.as_str().chars().skip(1) {
        if character == '\\' && !escaped {
            escaped = true;
            continue;
        }
        if escaped {
            match character {
                'n' => {
                    string.push("\n".to_string());
                },
                '\\' => {
                    string.push("\\".to_string());
                },
                '"' => {
                    string.push("\"".to_string());
                },
                _ => {
                    string.push("\\".to_string()+&character.to_string());
                }
            }
            escaped = false;
            continue;
        }
        string.push(character.to_string());
    }

    string.pop();
    string.join("")
}

#[test]
fn test_read_atom() -> Result<(), MalError> {
    let input = "+ 1 2)".to_string();
    let token_list = tokenize(&input)?;
    let reader = &mut Reader::new(token_list);
    let result = read_atom(reader)?;
    assert_eq!(result, MalType::MalSymbol("+".to_string()));
    Ok(())
}
