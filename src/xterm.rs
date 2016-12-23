use std::fmt;
use termion::color::{Bg, AnsiValue};
use termion::style::Reset;

use hex::Hex;


#[derive(Debug)]
pub struct C256(Zone);

impl C256 {
    pub fn rgb(r: u32, g: u32, b: u32) -> C256 {
        // a * 0 + b = 0    (1)
        // a * 5 + b = 255  (2)
        //
        // a = (255 - 0) / (5 + 0) = 51
        let r8 = r / 51;
        let g8 = g / 51;
        let b8 = b / 51;

        let ansi = r8 * 36 + g8 * 6 + b8 + 16;
        let zone = if ansi < 16 {
            Zone::System(ansi)
        } else {
            if (r8 == g8) && r8 == b8 {
                Zone::Grayscale(r8)
            } else {
                Zone::Cube(ansi)
            }
        };

        C256(zone)
    }


    pub fn sample(&self) -> String {
        let value = match self.0 {
            Zone::System(x) => AnsiValue(x as u8),
            Zone::Cube(x) => AnsiValue(x as u8),
            Zone::Grayscale(x) => AnsiValue::grayscale(x as u8),
        };

        format!("{}    {}", Bg(value), Reset)
    }

}

#[derive(Debug)]
pub enum Zone {
    System(u32),
    Cube(u32),
    Grayscale(u32),
}



#[derive(Debug)]
pub struct XTerm(u32, Zone);

impl XTerm {
    pub fn rgb(r: u32, g: u32, b: u32) -> XTerm {
        // a * 0 + b = 0    (1)
        // a * 5 + b = 255  (2)
        //
        // a = (255 - 0) / (5 + 0) = 51
        let r8 = r / 51;
        let g8 = g / 51;
        let b8 = b / 51;

        let ansi = r8 * 36 + g8 * 6 + b8 + 16;
        let zone = if ansi < 16 {
            Zone::System(ansi)
        } else {
            if (r == g) && r == b {
                Zone::Grayscale(r)
            } else {
                Zone::Cube(ansi)
            }
        };

        XTerm(ansi, zone)
    }

    pub fn sample(&self) -> String {
        format!("{}    {}", Bg(AnsiValue(self.0 as u8)), Reset)
    }
}


impl From<Hex> for XTerm {
    fn from(color: Hex) -> XTerm {
        let (r, g, b) = color.raw();
        let vr = u32::from_str_radix(r, 16).unwrap();
        let vg = u32::from_str_radix(g, 16).unwrap();
        let vb = u32::from_str_radix(b, 16).unwrap();

        XTerm::rgb(vr, vg, vb)
    }
}

impl fmt::Display for XTerm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:03}", self.0)
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
