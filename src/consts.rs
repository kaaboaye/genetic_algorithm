use std::error::Error;

pub type Number = u32;
pub type DynResult<T> = Result<T, Box<dyn Error>>;
