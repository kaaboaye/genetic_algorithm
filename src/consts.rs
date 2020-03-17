use std::error::Error;

pub type Number = i32;
pub type DynResult<T> = Result<T, Box<dyn Error>>;
