#[derive(Debug)]
pub enum MalError {
    ParseError,
    TokenizingError(String),
    ParenMismatch,
    TypeMismatch,
}