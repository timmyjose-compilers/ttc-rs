//! The Emitter module

use crate::GenResult;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct Emitter {
    outfile: &'static str,
    header: String,
    code: String,
}

impl Emitter {
    pub fn new(outfile: &'static str) -> Self {
        Emitter {
            outfile: outfile,
            header: String::new(),
            code: String::new(),
        }
    }

    pub fn header_line(&mut self, code: &str) {
        self.header.push_str(code);
        self.header.push('\n');
    }

    pub fn emit_line(&mut self, code: &str) {
        self.code.push_str(code);
        self.code.push('\n');
    }

    pub fn emit(&mut self, code: &str) {
        self.code.push_str(code);
    }

    pub fn write_file(&mut self) -> GenResult<()> {
        let mut writer = BufWriter::new(File::create(self.outfile)?);
        writer.write_all(self.header.as_bytes())?;
        writer.write_all(self.code.as_bytes())?;

        Ok(())
    }
}
