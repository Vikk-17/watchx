use hickory_net::NetError;
use std::io;

#[derive(Debug)]
pub enum AppError {
    Dns(NetError),
    Io(io::Error),
}

impl From<NetError> for AppError {
    fn from(err: NetError) -> Self {
        AppError::Dns(err)
    }
}
