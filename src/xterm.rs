use std::fmt;
use termion::color::{Bg, AnsiValue};
use termion::style::Reset;

use hex::Hex;


#[derive(Debug)]
pub struct XTerm(u32);

impl From<Hex> for XTerm {
    fn from(color: Hex) -> XTerm {
        let (r, g, b) = color.raw();
        let vr = u32::from_str_radix(r, 16).unwrap();
        let vg = u32::from_str_radix(g, 16).unwrap();
        let vb = u32::from_str_radix(b, 16).unwrap();

        // a * 0 + b = 0
        // a * 5 + b = 255
        // a = (255 - 0) / 5 = 51
        let res = (vr / 51) * 36 + (vg / 51) * 6 + (vb / 51) + 16;

        XTerm(res)
    }
}

impl fmt::Display for XTerm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<XTerm> for u8 {
    fn from(color: XTerm) -> u8 {
        color.0 as u8
    }
}

impl<'a> From<&'a XTerm> for u8 {
    fn from(color: &'a XTerm) -> u8 {
        color.0 as u8
    }
}


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
pub struct GrayscaleRamp;

impl fmt::Display for GrayscaleRamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ramp = (0..24).map(|x| format!("{}  ", Bg(AnsiValue::grayscale(x))))
                          .collect::<String>();

        write!(f, "{}\n{}{}", "Grayscale ramp:", ramp, Reset)
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
