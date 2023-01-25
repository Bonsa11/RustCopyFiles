use std::error::Error;
use std::io;
use std::process;
use std::fs;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    // read in csv with no headers
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());

    // loop through rows and copy first column to second column
    for result in rdr.records() {
        let record = result?;
        fs::copy(&record[0], &record[1])?;
    }
    Ok(())
}