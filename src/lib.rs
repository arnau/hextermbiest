extern crate palette;
extern crate termion;


pub mod error;
pub use error::{Error, Result};
pub mod hex;
pub mod xterm;
pub use xterm::XTerm;
pub use hex::Hex;
