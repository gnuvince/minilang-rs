use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Void,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Void => write!(f, "void"),
        }
    }
}

impl Type {
    pub fn format_letter(&self) -> char {
        match *self {
            Type::Int => 'd',
            Type::Float => 'f',
            Type::Void => 'v',
        }
    }
}
