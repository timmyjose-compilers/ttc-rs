use std::fs::File;
use std::io::{BufReader, Read};
use ttc_rs::emitter::Emitter;
use ttc_rs::lexer::Lexer;
use ttc_rs::parser::Parser;
use ttc_rs::GenResult;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.len() != 1 {
        usage();
    }

    match read_source(&args[0]) {
        Ok(source) => {
            let mut emitter = Emitter::new("out.c");
            let mut parser = Parser::new(Lexer::new(&source), &mut emitter);
            parser.parse();
            match emitter.write_file() {
                Ok(_) => println!("Program compiled successfully"),
                Err(err) => eprintln!("Failed to compile to C code: {:?}", err),
            }
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
