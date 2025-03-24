use std::error::Error;

pub trait Downloader {
    fn download(&self, url: &str, path: &str) -> Result<(), Box<dyn Error>>;
}
