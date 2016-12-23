extern crate termion;


pub mod error;
pub mod color;
pub mod xterm;

pub use error::{Error, Result};
pub use color::{Rgb, XTerm};
