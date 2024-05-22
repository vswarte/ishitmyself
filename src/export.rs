use std::fs;
use std::io;
use std::io::Write;

use crate::util::singleton::SingletonMap;

#[derive(Debug)]
pub enum ExportError {
    FileCreation(io::Error),
    FileWrite(io::Error),
}

pub trait Export {
    fn export(&self) -> Result<(), ExportError>;
}

impl Export for SingletonMap {
    fn export(&self) -> Result<(), ExportError> {
        let mut fh = fs::File::create("./singletons.csv")
            .map_err(ExportError::FileCreation)?;

        for entry in self.iter() {
            writeln!(fh, "\"{}\", {:x}", entry.0, entry.1)
                .map_err(ExportError::FileWrite)?;
        }

        Ok(())
    }
}
