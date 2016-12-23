extern crate hextermbiest;

use hextermbiest::xterm::{SystemColors, GreyscaleRamp, ColorCube};

fn main() {
    println!("{}", SystemColors);
    println!("\n{}", ColorCube);
    println!("\n{}", GreyscaleRamp);
}
