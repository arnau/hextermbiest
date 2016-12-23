use std::fmt;
use termion::color::{Bg, AnsiValue};
use termion::style::Reset;


#[derive(Debug)]
pub struct SystemColors;

impl fmt::Display for SystemColors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let normal: String = (0..8).map(|x| format!("{}  ", Bg(AnsiValue(x))))
                                   .collect();
        let bright: String = (8..16).map(|x| format!("{}  ", Bg(AnsiValue(x))))
                                   .collect();

        write!(f, "{}\n{}\n{}{}", "System colors:", normal, bright, Reset)
    }
}

#[derive(Debug)]
pub struct GreyscaleRamp;

impl fmt::Display for GreyscaleRamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ramp = (0..24).map(|x| format!("{}  ", Bg(AnsiValue::grayscale(x))))
                          .collect::<String>();

        write!(f, "{}\n{}{}", "Greyscale ramp:", ramp, Reset)
    }
}

#[derive(Debug)]
pub struct ColorCube;

impl fmt::Display for ColorCube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cube = String::new();

        for green in 0..6 {
            for red in 0..6 {
                for blue in 0..6 {
                    cube.push_str(&format!("{}  ", Bg(AnsiValue::rgb(red, green, blue))));
                }

                cube.push_str(&format!("{} ", Reset));
            }

            cube.push_str(&format!("{}\n", Reset));
        }

        write!(f, "{}\n{}{}", "Color cube:", cube, Reset)
    }
}
