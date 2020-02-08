extern crate minigrep;

use std::env;
use std::error::Error;
use std::process;

use minigrep::Config;

fn main() -> Result<(), Box<Error>> {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args)?;

  println!("Searching for {}", config.query);
  println!("In file {}", config.filename);

  if let Err(e) = minigrep::run(config) {
    println!("Application error: {}", e);
    process::exit(1);
  }

  Ok(())
}
