use clap::Parser;
use regex::Regex;
use std::fs;
use std::io::{BufWriter, Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Route of file with the urls to filter
    #[arg(short, long)]
    file: String,

    /// Route of the output file with correct urls
    #[arg(short, long, default_value_t = String::from("correct_urls_output.txt"))]
    output_file: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut count = 0;

    let file_text = fs::read_to_string(args.file).unwrap();

    let text_vector: Vec<&str> = file_text.split('\n').collect();

    let output_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&args.output_file)
        .unwrap();

    let mut writer = BufWriter::new(output_file);

    let links_vector: Vec<&&str> = text_vector
        .iter()
        .filter(|&text| is_valid_url(text))
        .collect();

    for url in links_vector {
        if let Ok(message) = checker((*url).to_string()).await {
            writer
                .write_all(format!("{message} \n").as_bytes())
                .unwrap();

            count += 1;
        }
    }

    let _ = writer.flush();

    println!("It is ready, total of correct urls: {count}");
    println!("Open file in {}", args.output_file);
}

fn is_valid_url(text: &str) -> bool {
    let url_regex = Regex::new(r"^https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)$").unwrap();
    url_regex.is_match(text)
}

async fn checker(url: String) -> Result<String, String> {
    let response = reqwest::get(&url).await.unwrap();
    let status_code = response.status();

    if status_code == 200 || status_code == 301 {
        Ok(format!("{url} (Ok)"))
    } else if status_code == 301 || status_code == 300 {
        Ok(format!("{url} (Redirect)"))
    } else {
        Err(String::from("Some happened"))
    }
}
