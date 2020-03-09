use std::error::Error;

pub type Number = u16;
pub type DynResult<T> = Result<T, Box<dyn Error>>;
