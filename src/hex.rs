use std::str::FromStr;
use std::str;

use error::{Result, Error};


#[derive(Debug, PartialEq)]
pub struct Hex(String, String, String);

impl FromStr for Hex {
    type Err = Error;

    fn from_str(s: &str) -> Result<Hex> {
        let s = if s.starts_with("#") {
            s.split_at(1).1
        } else { s };

        let size = match s.len() {
            6 => 2,
            3 => { return Err(Error::NotImplemented); },
            _ => { return Err(Error::Parse); }
        };


        let rgb: Vec<_> = s.as_bytes()
                           .chunks(size)
                           .collect();


        Ok(Hex(str::from_utf8(rgb[0])?.into(),
               str::from_utf8(rgb[1])?.into(),
               str::from_utf8(rgb[2])?.into()))
    }
}

impl Hex {
    pub fn raw(&self) -> (&str, &str, &str) {
        (&self.0, &self.1, &self.2)
    }
}
