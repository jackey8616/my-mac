use reqwest::{blocking::Client, header::CONTENT_LENGTH};
use std::fs::File;
use std::io::{self, Read, Write};

use crate::traits::Downloader;

pub struct HttpDownloader;

impl Downloader for HttpDownloader {
    fn download(&self, url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        let mut response = client.get(url).send()?;

        let mut file = File::create(path)?;

        let total_size = response
            .headers()
            .get(CONTENT_LENGTH)
            .and_then(|ct_len| ct_len.to_str().ok())
            .and_then(|ct_len| ct_len.parse::<u64>().ok())
            .unwrap_or(0);

        let mut downloaded: u64 = 0;
        let mut buffer = [0; 8192];

        loop {
            let bytes_read = response.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }

            file.write_all(&buffer[..bytes_read])?;

            downloaded += bytes_read as u64;

            if total_size > 0 {
                print!(
                    "\rDownloading... {:.1}%",
                    (downloaded as f64 / total_size as f64) * 100.0
                );
                io::stdout().flush()?;
            }
        }

        println!("\rDownload complete!");

        Ok(())
    }
}

#[cfg(test)]
mod http_downloader_tests {
    use super::*;

    #[test]
    fn test_download() {
        use crate::core::HttpDownloader;
        use std::fs;
        use std::path::Path;

        let downloader = HttpDownloader;
        let test_url = "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh";
        let test_path = "./test_download.sh";

        let result = downloader.download(test_url, test_path);
        assert!(result.is_ok(), "Download should succeed");

        assert!(
            Path::new(test_path).exists(),
            "Downloaded file should exist"
        );

        fs::remove_file(test_path).expect("Should remove test download");
    }
}
