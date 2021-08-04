//tutorial-read-01.rs
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;
use sha2::{Sha256, Digest};

fn run() -> Result<(), Box<dyn Error>> {
    let input = File::open(get_arg(1)?)?;
    let output = get_arg(2)?;
    
    let mut rdr = csv::ReaderBuilder::new()
    .has_headers(true)
    .delimiter(b',')
    .double_quote(true)
    .from_reader(input);

    let mut wtr = csv::WriterBuilder::new().from_path(output)?;
    
    for result in rdr.deserialize() {
        let record: String = result?;
        let mut hasher = Sha256::new();
        hasher.update(record);
        wtr.serialize(format!("{:x}", hasher.finalize()))?;
    }

    wtr.flush()?;
    Ok(())
}

/// Returns the positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_arg(arg_num: usize) -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(arg_num) {
        None => Err(From::from("expected 2 arguments, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}