use std::fmt::{Debug, Display, Formatter};

/// One error to rule them all!
#[derive(Debug)]
pub struct GenericErr(pub String);
impl GenericErr {
    pub fn new(text: impl Into<String>) -> Self {
        GenericErr(text.into())
    }
}
impl From<String> for GenericErr {
    fn from(text: String) -> Self {
        GenericErr::new(text)
    }
}
impl From<&str> for GenericErr {
    fn from(text: &str) -> Self {
        GenericErr::new(text)
    }
}
impl From<()> for GenericErr {
    fn from(_: ()) -> Self {
        GenericErr::new("Unknown Error")
    }
}
impl From<serde_json::error::Error> for GenericErr {
    fn from(err: serde_json::error::Error) -> Self {
        format!("Serde Error: {}", err).into()
    }
}
impl From<std::io::Error> for GenericErr {
    fn from(err: std::io::Error) -> Self {
        format!("IO Error: {}", err).into()
    }
}
impl From<std::fmt::Error> for GenericErr {
    fn from(err: std::fmt::Error) -> Self {
        format!("std::fmt Error: {}", err).into()
    }
}
impl From<std::num::ParseIntError> for GenericErr {
    fn from(err: std::num::ParseIntError) -> Self {
        format!("std::num::ParseIntError: {}", err).into()
    }
}
impl From<std::string::FromUtf8Error> for GenericErr {
    fn from(err: std::string::FromUtf8Error) -> Self {
        format!("std::string::FromUtf8Error: {}", err).into()
    }
}

impl Display for GenericErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
