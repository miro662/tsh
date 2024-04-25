pub mod parser;

#[derive(Debug, PartialEq, Eq)]
pub struct Command(pub Vec<String>);
