use clap::Parser;
use std::process;
use grop;

fn main() {
    // let args: Vec<String> = env::args().collect();

    let args = grop::Args::parse();
     
    let config = grop::Config::build(args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = grop::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
