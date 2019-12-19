use std::env;
use std::process;
mod lib;
use lib::InputParams;
fn main() {
    let args: Vec<String> = env::args().collect();
    let input: InputParams = InputParams::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = lib::run(input) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
