use chrono::{Datelike, Utc};
use lopdf::Document;
use regex::Regex;
use std::{fs::OpenOptions, io::Write, path::Path};

pub async fn handle(year: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_path_str = format!("./assets/pcge_{year}.pdf");
    let file_path = Path::new(&file_path_str);

    if !file_path.exists() {
        download(year, &file_path).await?;
    }

    let csv_path_str = format!("./assets/pcge_{year}.csv");
    let csv_path = Path::new(&csv_path_str);

    if !csv_path.exists() {
        make_csv(&file_path, &csv_path)?;
    }

    Ok(())
}

async fn download(year: &str, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://www.mef.gob.pe/contenidos/conta_publ/pcge/PCGE_{year}.pdf");
    let response = reqwest::get(url).await?.bytes().await?;

    std::fs::write(file_path, response)?;

    Ok(())
}

fn make_csv(file_path: &Path, csv_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let doc = Document::load(&file_path)?;
    let pages = doc.get_pages();
    let re = Regex::new(r"\s{2,}").unwrap();
    let year_now = Utc::now().year();

    let mut csv = OpenOptions::new()
        .append(true)
        .create(true)
        .open(csv_path)?;

    for page_num in 21..=62 {
        if let Some(_page_id) = pages.get(&page_num) {
            let text = doc.extract_text(&[page_num])?;
            let lines: Vec<&str> = re.split(&text).collect();

            for line in lines {
                if let Some((code, desc)) = line.split_once(" ") {
                    if code.len() > 2 && code.chars().all(|c| c.is_ascii_digit()) {
                        writeln!(csv, "{year_now}0101|{code}|{}|01||||1", desc.trim())?;
                    }
                }
            }
        }
    }

    Ok(())
}
