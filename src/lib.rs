#![feature(step_by)]

extern crate palette;
extern crate termion;


pub mod error;
pub mod color;
pub mod hex;
pub mod xterm;
pub mod codex;

pub use error::{Error, Result};
pub use hex::Hex;
pub use xterm::XTerm;
