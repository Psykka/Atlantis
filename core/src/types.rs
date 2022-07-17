use std::fmt::{Display, Formatter, Error};

pub enum Format {
    Byte,
    HalfWord,
    Word,
}

impl Display for Format {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Format::Byte => write!(f, "Byte"),
            Format::HalfWord => write!(f, "HalfWord"),
            Format::Word => write!(f, "Word"),
        }
    }
}