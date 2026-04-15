pub async fn handle(year: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

async fn download(year: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://www.mef.gob.pe/contenidos/conta_publ/pcge/PCGE_{year}.pdf");

    Ok(())
}
