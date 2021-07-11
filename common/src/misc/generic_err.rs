use std::fmt::{Display, Formatter};

/// One error to rule them all!
#[derive(Debug)]
pub struct GenericErr(pub String);
impl Default for GenericErr {
    fn default() -> Self {
        ().into()
    }
}
impl Display for GenericErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for GenericErr {
    fn from(text: String) -> Self {
        GenericErr(text)
    }
}
impl From<&str> for GenericErr {
    fn from(text: &str) -> Self {
        GenericErr(text.into())
    }
}
impl From<()> for GenericErr {
    fn from(_: ()) -> Self {
        GenericErr("Unknown Error".into())
    }
}

macro_rules! impl_generic_err {
    ($t: ty) => {
        impl From<$t> for GenericErr {
            fn from(err: $t) -> Self {
                format!("{}: {}", stringify!($t), err).into()
            }
        }
    };
}
impl_generic_err!(serde_json::error::Error);
impl_generic_err!(std::io::Error);
impl_generic_err!(std::fmt::Error);
impl_generic_err!(std::num::ParseIntError);
impl_generic_err!(std::string::FromUtf8Error);
