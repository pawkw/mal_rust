use crate::malerror::MalError;

#[derive(Clone, PartialEq, Debug)]
pub enum MalType {
    MalList(Vec<MalType>),
    MalSymbol(String),
}

impl MalType {
    pub fn push(&mut self, item: MalType) -> Result<(), MalError> {
        match self {
            Self::MalList(x) => {
                x.push(item);
                Ok(())
            }
            _ => Err(MalError::TypeMismatch),
        }
    }
}
