mod sunat;

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: {} <nombre>", args[0]);
        std::process::exit(1);
    }

    if args[1] == "cronograma" {
        sunat::cronograma::handle(&args[2]).await?;
    } else if args[1] == "pcge" {
        sunat::pcge::handle(&args[2]).await?;
    }

    Ok(())
}
