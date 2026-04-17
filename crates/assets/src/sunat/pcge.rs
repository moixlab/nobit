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
    let re = Regex::new(r"(\d{2,})\s+([^\d\n\-]+)").unwrap();
    let year_now = Utc::now().year();

    let mut csv = OpenOptions::new()
        .append(true)
        .create(true)
        .open(csv_path)?;

    for page_num in 21..=62 {
        if let Some(_page_id) = pages.get(&page_num) {
            let text = doc.extract_text(&[page_num])?;

            let mut code_last = "".to_string();

            for cap in re.captures_iter(&text) {
                let code = check_code(&code_last, cap[1].trim());
                let desc = cap[2].trim().chars().take(100).collect::<String>();

                if code.len() > 2 && !desc.contains("ELEMENTO") {
                    writeln!(csv, "{year_now}0101|{code}|{desc}|01||||1|")?;
                    code_last = code;
                }
            }
        }
    }

    Ok(())
}

fn check_code(code_last: &str, code_now: &str) -> String {
    match (code_last, code_now) {
        ("27242", "27233") => "27243".to_string(),
        ("3011", "30221") => "30111".to_string(),
        ("30111", "30224") => "30114".to_string(),
        ("688", "6882") => "6881".to_string(),
        ("7012", "70111") => "70121".to_string(),
        ("70121", "70112") => "70122".to_string(),
        _ => code_now.to_string(),
    }
}
