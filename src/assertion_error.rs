use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Default)]
pub struct AssertionError {
    pub message: String,
}

impl AssertionError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for AssertionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Debug for AssertionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AssertionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
