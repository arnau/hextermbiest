extern crate hextermbiest;

use hextermbiest::color::codex;

fn main() {
    for color in codex() {
        println!("{} #{:06x}", color, color);
    }
}
