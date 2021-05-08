use std::fmt::{Debug, Display, Formatter};

macro_rules! impl_from_using_debug {
    ($t:ty) => {
        impl From<$t> for GenericErr {
            fn from(err: $t) -> Self {
                GenericErr::new(format!("{:?}", err))
            }
        }
    };
}

#[derive(Debug)]
/// One error to rule them all!
pub struct GenericErr(String);
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
impl_from_using_debug!(std::io::Error);
impl_from_using_debug!(std::fmt::Error);

impl Display for GenericErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: '{}'", self.0)
    }
}
