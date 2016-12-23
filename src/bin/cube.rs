extern crate hextermbiest;

use hextermbiest::xterm::{SystemColors, GrayscaleRamp, ColorCube};

fn main() {
    println!("{}", SystemColors);
    println!("\n{}", ColorCube);
    println!("\n{}", GrayscaleRamp);
}
