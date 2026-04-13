use chrono::NaiveDate;
use scraper::{Html, Selector};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub async fn handle(year: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_path_str = format!("./assets/cronograma_{year}.txt");
    let file_path = Path::new(&file_path_str);

    if !file_path.exists() {
        download(year, &file_path).await?;
    }

    let csv_path_str = format!("./assets/cronograma_{year}.csv");
    let csv_path = Path::new(&csv_path_str);

    if !csv_path.exists() {
        make_csv(&year, &file_path, &csv_path)?;
    }

    Ok(())
}

async fn download(year: &str, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://www.sunat.gob.pe/orientacion/cronogramas/{year}/cObligacionMensual{year}.html"
    );
    let response = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&response);
    let table_selector = Selector::parse("table")?;

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;

    if let Some(first_table) = document.select(&table_selector).next() {
        let tr_selector = Selector::parse("tr")?;

        for row in first_table.select(&tr_selector) {
            let td_selector = Selector::parse("td")?;

            for cell in row.select(&td_selector) {
                write!(file, "\t{}", cell.text().collect::<String>())?;
            }

            write!(file, "\n")?;
        }
    }

    Ok(())
}

fn make_csv(
    year: &str,
    file_path: &Path,
    csv_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut year: i32 = year.parse::<i32>()?;
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut csv = OpenOptions::new()
        .append(true)
        .create(true)
        .open(csv_path)?;

    writeln!(csv, "Subject,Start Date,All Day Event")?;

    let subjects = ["0", "1", "2 y 3", "4 y 5", "6 y 7", "8 y 9", "BCU"];
    let mut counter: usize = 0;
    let mut month = 1;

    for (index, line) in reader.lines().enumerate() {
        if index < 2 || counter == 7 {
            counter = 0;
            continue;
        }

        if counter == 0 {
            month += 1;
        }

        if let Some(last) = line?.split('\t').last() {
            let day: u32 = last.parse::<u32>()?;
            if month == 13 {
                month = 1;
                year += 1;
            }
            let date_event = NaiveDate::from_ymd_opt(year, month, day).unwrap();
            let date_str = date_event.format("%m/%d/%Y").to_string();

            writeln!(csv, "RUC {}, {}, True", subjects[counter], date_str)?;
            counter += 1;
        }
    }

    Ok(())
}
