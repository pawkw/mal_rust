#[derive(Debug)]
pub enum MalError {
    ParseError,
    TokenizingError,
    ParenMismatch,
    TypeMismatch,
}