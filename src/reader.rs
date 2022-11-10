use std::result;

// mod maltype;
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
        _ => Err(MalError::TokenizingError(
            "reader::read_str: tokenize failed.".to_string(),
        )),
    }
}

fn tokenize(input: &String) -> Result<Vec<String>, MalError> {
    const PATTERN: &str =
        r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#;

    let re = Regex::new(PATTERN).unwrap();
    let tokens = re
        .captures_iter(input)
        .map(|x| x[0].trim().to_string())
        .collect();
    match tokens {
        Vec { .. } => Ok(tokens),
        _ => Err(MalError::TokenizingError(
            "reader::tokenize error.".to_string(),
        )),
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
                Ok(read_list(token_reader)?)
            } else {
                Ok(read_atom(token_reader)?)
            }
        },
        None => Err(MalError::ParseError),
    }
}

fn read_list(token_reader: &mut Reader) -> Result<MalType, MalError> {
    let mut token_list = MalType::MalList(vec![]);
    'dave: loop {
        match token_reader.peek() {
            Some(x) => {
                if x.starts_with(")") {
                    token_reader.read();
                    break 'dave;
                }
                let item: MalType = read_form(token_reader).unwrap();
                token_list.push(item).unwrap();
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
    let token = &token_reader.read();
    match token {
        Some(x) => {
            Ok(MalType::MalSymbol(x.to_string()))
        }
        None => Err(MalError::ParseError),
    }
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
