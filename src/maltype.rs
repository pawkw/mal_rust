use crate::malerror::MalError;

#[derive(Clone, PartialEq, Debug)]
pub enum MalType {
    MalList(Vec<MalType>),
    MalSymbol(String),
    MalNil,
    MalVec(Vec<MalType>),
    MalHashmap(Vec<MalType>),
    MalString(String),
    MalTrue,
    MalFalse,
    MalKeyword(String),
}

