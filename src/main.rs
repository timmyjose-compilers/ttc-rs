use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use ttc_rs::lexer::Lexer;
use ttc_rs::parser::Parser;

type GenError = Box<dyn Error>;
type GenResult<T> = Result<T, GenError>;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.len() != 1 {
        usage();
    }

    match read_source(&args[0]) {
        Ok(source) => {
            let mut parser = Parser::new(Lexer::new(&source));
            parser.parse();
            println!("Program parsed successfully");
        }

        Err(err) => eprintln!(
            "Error while trying to open source file {}: {}",
            args[0], err
        ),
    }
}

fn read_source(infile: &str) -> GenResult<String> {
    let mut reader = BufReader::new(File::open(infile)?);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn usage() {
    eprintln!("Usage: ttc source-file");
    std::process::exit(0);
}
