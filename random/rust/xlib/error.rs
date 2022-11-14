use std::result::Result as StdResult;
//use crate::display::error::init_error_handler;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub reason: &'static str
}

pub fn init() {
    //init_error_handler();
}

impl Error {
    pub fn new(reason: &'static str) -> Self {
        Self { reason }
    }
}
