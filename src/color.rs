use std::str::FromStr;
use std::str;
use std::fmt;
use termion::color::{Bg, AnsiValue};
use termion::style::Reset;

use error::{Result, Error};


/// System colors.
pub const SYSTEM: [u32; 16] = [
    0x000000, 0x800000, 0x008000, 0x808000,
    0x000080, 0x800080, 0x008080, 0xc0c0c0,

    0x808080, 0xff0000, 0x00ff00, 0xffff00,
    0x0000ff, 0xff00ff, 0x00ffff, 0xffffff,
];


/// Cube (6x6x6) intensities.
pub const INTENSITIES: [u32; 6] = [
    0x00, 0x5F, 0x87,
    0xAF, 0xD7, 0xFF
];


/// Shades of grey.
pub const SHADES: [u32; 24] = [
    0x08, 0x12, 0x1C, 0x26,
    0x30, 0x3A, 0x44, 0x4E,
    0x58, 0x62, 0x6C, 0x76,
    0x80, 0x8A, 0x94, 0x9E,
    0xA8, 0xB2, 0xBC, 0xC6,
    0xD0, 0xDA, 0xE4, 0xEE,
];


/// RGB triple encoded as three unsigned integers.
#[derive(Debug, Clone, PartialEq)]
pub struct Rgb(pub u32, pub u32, pub u32);

impl Rgb {
    pub fn triple(&self) -> (u32, u32, u32) {
        (self.red(), self.green(), self.blue())
    }

    pub fn red(&self) -> u32 {
        self.0
    }

    pub fn green(&self) -> u32 {
        self.1
    }

    pub fn blue(&self) -> u32 {
        self.2
    }

    pub fn is_grey(&self) -> bool {
        self.red() == self.green() && self.red() == self.blue()
    }

    pub fn sample(&self) -> String {
        format!("{}     {}", Bg(AnsiValue::rgb(self.red() as u8,
                                               self.green() as u8,
                                               self.blue() as u8)), Reset)
    }
}


/// Converting from a number is assumed to be a Hexadecimal triple.
impl From<u32> for Rgb {
    fn from(hex: u32) -> Rgb {
        let red = hex >> 16;
        let green = (hex >> 8) & !(0xff << 8);
        let blue = hex & !(0xffff << 8);

        Rgb(red, green, blue)
    }
}


/// Converting from a string is assumed to be a Hexadecimal triple in the form
/// of `"#rrggbb"` or `"rgb"`.
impl FromStr for Rgb {
    type Err = Error;

    fn from_str(s: &str) -> Result<Rgb> {
        let s = s.trim_left_matches('#');
        let hex = u32::from_str_radix(s, 16)?;

        // TODO: This check should go to `From<u32>`. Move it when rust
        // stabilises `TryFrom`.
        if hex > 0xffffff {
            return Err(Error::Parse);
        }

        Ok(From::from(hex))
    }
}


impl From<Rgb> for u32 {
    fn from(rgb: Rgb) -> u32 {
        let mut res = rgb.red() << 16;
        res &= !(0xff << 8);
        res |= rgb.green() << 8;
        res = res & !0xff;
        res = res | rgb.blue();

        res
    }
}

impl fmt::LowerHex for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:06x}", u32::from(self.clone()))
    }
}

impl fmt::UpperHex for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:06X}", u32::from(self.clone()))
    }
}

impl From<XTerm> for Rgb {
    /// Decomposing a XTerm code in three 6-bit does not guarantee the result
    /// will be the same 0-255 number from the official codex.
    ///
    /// ```
    /// # use hextermbiest::color::{Rgb, XTerm};
    /// let rgb = Rgb::from(0xffd75f);            // Rgb(255, 215, 95)
    /// let xterm = XTerm::from(rgb);             // XTerm::Cube(221)
    /// ```
    ///
    /// To recover the exact RGB triple, the `approximate` function adjusts
    /// the input as one of the possible intensities: `0x00`, `0x5f`, `0x87`,
    /// `0xAF`, `0xD7`.
    ///
    /// So, the result is the expected approximation
    ///
    /// ```
    /// # use hextermbiest::color::{Rgb, XTerm};
    /// # let rgb = Rgb::from(0xffd75f);
    /// # let xterm = XTerm::from(rgb);
    /// let rgb2 = Rgb::from(xterm);              // Rgb(255, 215, 95)
    /// println!("{:06x}", rgb2);                 // 0xffd75f
    /// ```
    ///
    /// Instead of the exact result:
    ///
    /// ```
    /// # use hextermbiest::color::{Rgb, XTerm};
    /// # let rgb = Rgb::from(0xffd75f);
    /// # let xterm = XTerm::from(rgb);
    /// let rgb2 = Rgb::from(xterm);              // Rgb(255, 204, 51)
    /// println!("{:06x}", rgb2);                 // 0xffcc33
    /// ```
    fn from(color: XTerm) -> Rgb {
        match color {
            XTerm::Greyscale(code) => {
                let shade = (code as usize) - 0xE8;
                let (_, &hex) = SHADES.into_iter().enumerate()
                                      .find(|&(x, _)| x == shade).unwrap();
                Rgb(hex, hex, hex)
            }

            XTerm::System(code) => {
                Rgb::from(SYSTEM[code as usize])
            }

            XTerm::Cube(code) => {
                let base = code - 16;
                let red = base / 36;
                let green = (base % 36) / 6;
                let blue = base % 6;

                Rgb(approximate(red * 51), approximate(green * 51), approximate(blue * 51))
            }
        }
    }
}



// ----------------------------------------------------------------------------
// XTerm
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum XTerm {
    System(u32),
    Cube(u32),
    Greyscale(u32),
}

impl XTerm {
    pub fn code(&self) -> u32 {
        match *self {
            XTerm::System(x) => x,
            XTerm::Greyscale(x) => x,
            XTerm::Cube(x) => x,
        }
    }

    pub fn sample(&self) -> String {
        let code = self.code() as u8;

        format!("{}     {}", Bg(AnsiValue(code)), Reset)
    }
}

impl From<Rgb> for XTerm {
    fn from(rgb: Rgb) -> XTerm {
        let (red, green, blue) = rgb.triple();

        if rgb.is_grey() {
            if let Some((shade, _)) = SHADES.into_iter().enumerate()
                                            .find(|&(_, &hex)| hex == red) {
                return XTerm::Greyscale(0xE8 + (shade as u32));
            }
        }


        let hexref = u32::from(rgb);
        if let Some((code, _)) = SYSTEM.into_iter().enumerate()
                                     .find(|&(_, &hex)| hex == hexref) {
            return XTerm::System(code as u32);
        }


        // a * 0 + b = 0    (1)
        // a * 5 + b = 255  (2)
        //
        // a = (255 - 0) / (5 + 0) = 51
        let r8 = red / 51;
        let g8 = green / 51;
        let b8 = blue / 51;
        let code = r8 * 36 + g8 * 6 + b8 + 16;

        XTerm::Cube(code)
    }
}

impl fmt::Display for XTerm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:03}", self.sample(), self.code())
    }
}

impl fmt::LowerHex for XTerm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:06x}", Rgb::from(self.clone()))
    }
}

impl fmt::UpperHex for XTerm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:06X}", Rgb::from(self.clone()))
    }
}


impl From<u32> for XTerm {
    fn from(x: u32) -> XTerm {
        match x {
            x if x <= 15 => XTerm::System(x),
            x if x >= 232 => XTerm::Greyscale(x),
            x => XTerm::Cube(x),
        }
    }
}

impl From<XTerm> for u32 {
    fn from(color: XTerm) -> u32 {
        color.code()
    }
}


impl FromStr for XTerm {
    type Err = Error;

    fn from_str(s: &str) -> Result<XTerm> {
        let rgb = s.parse::<Rgb>()?;

        Ok(From::from(rgb))
    }
}


/// The only values allowed in a cube are 0x00, 0x5f, 0x87, 0xAF, 0xD7 and
/// 0xFF.  This function makes sure the value is one of these.
fn approximate(x: u32) -> u32 {
    match x {
        x if x > 0x00 && x < 0x5f => 0x5f,
        x if x > 0x5f && x < 0x87 => 0x87,
        x if x > 0x87 && x < 0xAF => 0xAF,
        x if x > 0xAF && x < 0xD7 => 0xD7,
        x => x,
    }
}


pub fn codex() -> Vec<XTerm> {
    (0..256).map(|x| XTerm::from(x)).collect()
}

pub fn grayscale() -> Vec<u32> {
    SHADES.into_iter()
          .map(|x| (x << 16) | (x << 8) | x)
          .collect()
}
