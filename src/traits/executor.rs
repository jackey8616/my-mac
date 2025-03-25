use std::error::Error;

pub trait Executor {
    fn execute(&self) -> Result<bool, Box<dyn Error>>;
}
