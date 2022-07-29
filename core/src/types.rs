use std::fmt::{Display, Error, Formatter};

pub enum Format {
    Byte,
    Halfword,
    Word,
}

impl Display for Format {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Format::Byte => write!(f, "Byte"),
            Format::Halfword => write!(f, "Halfword"),
            Format::Word => write!(f, "Word"),
        }
    }
}
