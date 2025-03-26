use std::error::Error;

pub trait Executor<T> {
    fn execute(&self) -> Result<T, Box<dyn Error>>;
}
