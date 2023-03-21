use std::fmt;

// these will come in handy later
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Addr(pub [u8; 4]);

impl fmt::Display for Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [a, b, c, d] = self.0;
        write!(f, "{}.{}.{}.{}", a, b, c, d)
    }
}

impl fmt::Debug for Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
