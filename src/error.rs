use casbin::error::Error as CasbinError;
use casbin_rb_adapter::casbin;
use rbatis::rbdc;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    E(String),
    #[error("{0}")]
    Internal(String),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Db(#[from] rbdc::Error),
    #[error("{0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("{0}: {1}")]
    Code(i32, String),
    #[error("{0}")]
    Casbin(#[from] CasbinError),
}

impl Error {
    pub fn err<T>(s: impl Into<String>) -> Result<T> {
        Err(Error::E(s.into()))
    }
}

impl Error {
    pub fn to_msg(&self) -> (i32, String) {
        let msg = match self {
            Error::E(s) => (1, s.clone()),
            _ => {
                // tracing::info!("error: {:?}", self);
                (2, "操作失败".to_string())
            },
        };
        (msg.0, msg.1)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::E(s.to_string())
    }
}

#[macro_export]
macro_rules! error_info {
    ($code: expr) => {
        $crate::service::CONTEXT.config.get_error($code)
    };
    ($code: expr, $arg: expr) => {
        $crate::service::CONTEXT.config.get_error_arg($code, $arg)
    };
}
