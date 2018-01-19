use std::ops::{Add, Mul, Div};
use std::fmt::{self, Formatter, Display};
use std::cmp::max;
use std::error::Error;

#[derive(Clone, Copy)]
pub struct RGBColor {
    r: f64,
    g: f64,
    b: f64
}

impl RGBColor {
    pub fn new(r: f64, g: f64, b: f64) -> RGBColor {
        RGBColor { r: r, g: g, b: b }
    }

    pub fn new_u8(r: u8, g: u8, b: u8) -> RGBColor {
        RGBColor::new(r as f64 / 255., g as f64 / 255., b as f64 / 255.)
    }

    pub fn from_hex(hex: &str) -> Result<RGBColor, Box<Error>> {
        let mut hex_chars = hex.chars().peekable();
        if hex_chars.peek().unwrap() == &'#' { 
            hex_chars.next();
        }
        let err_msg = "Failed to parse color";

        let r = hex_chars.next().ok_or(err_msg)?.to_digit(16).ok_or(err_msg)? * 16 + hex_chars.next().ok_or(err_msg)?.to_digit(16).ok_or(err_msg)?;
        let g = hex_chars.next().ok_or(err_msg)?.to_digit(16).ok_or(err_msg)? * 16 + hex_chars.next().ok_or(err_msg)?.to_digit(16).ok_or(err_msg)?;
        let b = hex_chars.next().ok_or(err_msg)?.to_digit(16).ok_or(err_msg)? * 16 + hex_chars.next().ok_or(err_msg)?.to_digit(16).ok_or(err_msg)?;

        Ok(RGBColor::new(r as f64 / 255., g as f64 / 255., b as f64 / 255.))
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn g(&self) -> f64 {
        self.g
    }

    pub fn b(&self) -> f64 {
        self.b
    }

    pub fn r_u8(&self) -> u8 {
        (self.r * 255.) as u8
    }

    pub fn g_u8(&self) -> u8 {
        (self.g * 255.) as u8
    }

    pub fn b_u8(&self) -> u8 {
        (self.b * 255.) as u8
    }

    pub fn max_to_one(&self) -> RGBColor {
        let max_value = if self.r > self.g {
            if self.r > self.b {
                self.r
            } else {
                self.b
            }
        } else {
            if self.g > self.b {
                self.g
            } else {
                self.b
            }
        };

        if max_value > 1. {
            *self / max_value
        } else {
            *self
        }
    }
}

impl Display for RGBColor {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "(r {}, g {}, b {})", self.r, self.g, self.b)
    }
}

impl Mul<f64> for RGBColor {
    type Output = RGBColor;
    
    fn mul(self, rhs: f64) -> RGBColor {
        RGBColor::new(rhs * self.r, rhs * self.g, rhs * self.b)
    }
}

impl Div<f64> for RGBColor {
    type Output = RGBColor;
    
    fn div(self, rhs: f64) -> RGBColor {
        RGBColor::new(self.r / rhs, self.g / rhs, self.b / rhs)
    }
}

impl Mul<RGBColor> for f64 {
    type Output = RGBColor;
    
    fn mul(self, rhs: RGBColor) -> RGBColor {
        RGBColor::new(self * rhs.r, self * rhs.g, self * rhs.b)
    }
}

impl Mul<RGBColor> for RGBColor {
    type Output = RGBColor;

    fn mul(self, rhs: RGBColor) -> RGBColor {
        RGBColor::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl Add<RGBColor> for RGBColor {
    type Output = RGBColor;

    fn add(self, rhs: RGBColor) -> RGBColor {
        RGBColor::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

pub const BLACK: RGBColor = RGBColor { r: 0., g: 0., b: 0. };
pub const RED: RGBColor = RGBColor { r: 1., g: 0., b: 0. };
pub const GREEN: RGBColor = RGBColor { r: 0., g: 1., b: 0. };
pub const BLUE: RGBColor = RGBColor { r: 0., g: 0., b: 1. };
pub const GRAY: RGBColor = RGBColor { r: 0.5, g: 0.5, b: 0.5 };
pub const WHITE: RGBColor = RGBColor { r: 1., g: 1., b: 1. };
