use std::ops::DerefMut;

use crate::MalType;

pub fn pr_str(input: &MalType, print_readably: bool) -> String {
    match input {
        MalType::MalSymbol(x) => String::from(x),
        MalType::MalList(x) => "(".to_string() + &get_list(x, print_readably) + &")".to_string(),
        MalType::MalHashmap(x) => "{".to_string() + &get_list(x, print_readably) + &"}".to_string(),
        MalType::MalVec(x) => "[".to_string() + &get_list(x, print_readably) + &"]".to_string(),
        MalType::MalNil => String::from("nil"),
        MalType::MalString(x) => get_string(x, print_readably),
        MalType::MalFalse => String::from("false"),
        MalType::MalTrue => String::from("true"),
        MalType::MalKeyword(x) => ":".to_string() + x,
    }
}

fn get_list(input: &Vec<MalType>, print_readably: bool) -> String {
    let mut string_list = vec![];
    for item in input {
        string_list.push(pr_str(item, print_readably));
    }
    string_list.join(" ")
}

fn get_string(input: &String, print_readably: bool) -> String {
    if !print_readably {
        return input.into();
    } else {
        let mut string: Vec<String> = vec![];
        string.push("\"".to_string());
        for character in input.as_str().chars() {
            match character {
                '\n' => {
                    string.push("\\n".to_string());
                }
                '\\' => {
                    string.push("\\\\".to_string());
                }
                '"' => {
                    string.push("\\\"".to_string());
                }
                _ => {
                    string.push(character.to_string());
                }
            }
        }
        string.push("\"".to_string());
        return string.join("")
    }
}
