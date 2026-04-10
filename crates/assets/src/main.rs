mod sunat;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: {} <nombre>", args[0]);
        std::process::exit(1);
    }

    match args[1] {
        "cronograma": sunat::cronograma::handle(args[2]),
        _ => std::process::exit(1),
    }
}
