//tutorial-read-01.rs
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct InputRow {
    cpf: String,
    num_chpras: String,
    dat_abta_cta: String,
    final_cartao: String,
}

#[derive(Serialize, Deserialize)]
struct OutputRow {
    cpf_hasheado: String,
    cpf: String,
    num_chpras: String,
    dat_abta_cta: String,
    final_cartao: String,
}

fn run() -> Result<(), Box<dyn Error>> {
    let input = File::open(get_arg(1)?)?;
    let output = get_arg(2)?;
    
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .double_quote(true)
        .from_reader(input);

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .double_quote(false)
        .from_path(output)?;
    
    for result in rdr.deserialize() {
        let record: InputRow = result?;
        let mut hasher = Sha256::new();
        hasher.update(record.cpf.clone());
        wtr.serialize(&[OutputRow {
            cpf_hasheado: format!("{:x}", hasher.finalize()), 
            cpf: record.cpf, 
            num_chpras: record.num_chpras, 
            dat_abta_cta: record.dat_abta_cta, 
            final_cartao: record.final_cartao
        }])?;
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