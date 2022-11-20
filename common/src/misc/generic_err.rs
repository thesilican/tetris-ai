use std::{
    error::Error,
    fmt::{Display, Formatter},
};

/// One error to rule them all!
#[derive(Debug)]
pub struct GenericErr {
    message: String,
    source: Option<Box<dyn Error>>,
}
impl GenericErr {
    pub fn with_message(message: &str) -> Self {
        GenericErr {
            message: message.to_string(),
            source: None,
        }
    }
    pub fn with_error(name: &str, err: Box<dyn Error>) -> Self {
        GenericErr {
            message: format!("{}: {}", name, err),
            source: Some(err),
        }
    }
}

impl Default for GenericErr {
    fn default() -> Self {
        ().into()
    }
}
impl Display for GenericErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Error for GenericErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}

impl From<String> for GenericErr {
    fn from(text: String) -> Self {
        GenericErr::with_message(&text)
    }
}
impl From<&str> for GenericErr {
    fn from(text: &str) -> Self {
        GenericErr::with_message(text.into())
    }
}
impl From<()> for GenericErr {
    fn from(_: ()) -> Self {
        GenericErr::with_message("unknown error".into())
    }
}

macro_rules! impl_generic_err {
    ($t: ty) => {
        impl From<$t> for GenericErr {
            fn from(err: $t) -> Self {
                GenericErr::with_error(stringify!($t), Box::new(err))
                // format!("{}: {}", stringify!($t), err).into()
            }
        }
    };
}
impl_generic_err!(serde_json::error::Error);
impl_generic_err!(std::io::Error);
impl_generic_err!(std::fmt::Error);
impl_generic_err!(std::num::ParseIntError);
impl_generic_err!(std::string::FromUtf8Error);
impl_generic_err!(std::array::TryFromSliceError);
impl_generic_err!(redis::RedisError);
impl_generic_err!(base64::DecodeError);
impl_generic_err!(sdl2::video::WindowBuildError);
impl_generic_err!(sdl2::IntegerOrSdlError);

#[macro_export]
macro_rules! generic_err {
    () => {
        generic_err!("unknown error")
    };
    ($($arg:tt)*) => {
        ::std::result::Result::Err($crate::GenericErr::from(format!($($arg)*)))
    };
}

pub type GenericResult<T> = Result<T, GenericErr>;
