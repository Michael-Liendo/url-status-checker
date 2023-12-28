use clap::Parser;
use regex::Regex;
use reqwest::StatusCode;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

/// cleans URLs from an input file and verifies their status codes. Cleaned URLs and their status codes are stored in an output file.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File containing the URLs to clean and verify.
    #[arg(short, long)]
    file: String,

    /// Output file where valid URLs and their status codes will be saved.
    #[arg(short, long, default_value_t = String::from("correct_urls_output.txt"))]
    output_file: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let input_file_path = PathBuf::from(&args.file);
    let output_file_path = PathBuf::from(&args.output_file);

    let file_text = fs::read_to_string(&input_file_path)?;

    let valid_urls: Vec<&str> = file_text
        .split('\n')
        .filter(|&text| is_valid_url(text))
        .collect();

    let mut writer = BufWriter::new(
        fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&output_file_path)?,
    );

    let mut count = 0;

    for url in valid_urls {
        let message = checker(url).await?;
        writer.write_all(format!("{message} \n").as_bytes())?;
        count += 1;
    }

    writer.flush()?;

    println!("It is ready, total of correct URLs: {count}");
    println!("Open file in {}", output_file_path.display());

    Ok(())
}

fn is_valid_url(text: &str) -> bool {
    let url_regex = Regex::new(r"^https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)$").unwrap();
    url_regex.is_match(text)
}

async fn checker(url: &str) -> Result<String, String> {
    let response = reqwest::get(url).await.map_err(|err| err.to_string())?;
    let status_code = response.status();

    match status_code {
        StatusCode::OK => Ok(format!("{url} (Ok)")),
        StatusCode::MULTIPLE_CHOICES | StatusCode::MOVED_PERMANENTLY => {
            Ok(format!("{url} (Redirect)"))
        }
        _ => Err(format!(
            "Error checking URL: {url} (status code: {status_code})"
        )),
    }
}
