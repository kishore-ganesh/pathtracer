use std::fs::File;
use std::io::{Error, Write};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Sub};
use std::path::Path;
#[derive(Copy, Clone, Debug)]
pub struct RGB {
    r: f32,
    g: f32,
    b: f32,
}
//We'll floor it later

//RGB grid
//For light accumulation: +, for rest, multiply?
impl RGB {
    pub fn create(r: f32, g: f32, b: f32) -> Self {
        RGB { r, g, b }
    }
    pub fn black() -> Self {
        RGB {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
    pub fn is_black(&self) -> bool {
        self.r <= 0.0 && self.g <= 0.0 && self.b <= 0.0
    }

    pub fn is_nan(&self) -> bool {
        self.r.is_nan() || self.g.is_nan() || self.b.is_nan()
    }
}

fn clamp(x: f32, l: f32, r: f32) -> f32 {
    if x < l {
        return l;
    }
    if x > r {
        return r;
    }
    x
}

pub fn clamp_rgb(x: RGB, l: f32, r: f32) -> RGB {
    RGB::create(clamp(x.r, l, r), clamp(x.g, l, r), clamp(x.b, l, r))
}
impl Add for RGB {
    type Output = RGB;
    fn add(self, other: Self) -> Self {
        Self {
            r: (self.r + other.r),
            g: (self.g + other.g),
            b: (self.b + other.b),
        }
    }
}

impl Sub<f32> for RGB {
    type Output = RGB;
    fn sub(self, other: f32) -> Self {
        Self {
            r: (self.r - other),
            g: (self.g - other),
            b: (self.b - other),
        }
    }
}

impl AddAssign for RGB {
    fn add_assign(&mut self, other: Self) {
        //TODO: handle overflow
        //TODO: check this syntax
        *self = Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        };
    }
}

impl Div<f32> for RGB {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl DivAssign<f32> for RGB {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        };
    }
}

impl Mul<f32> for RGB {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}
impl Mul<RGB> for RGB {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            r: (self.r * rhs.r) / 255.0,
            g: (self.g * rhs.g) / 255.0,
            b: (self.b * rhs.b) / 255.0,
        }
    }
}

impl Mul<RGB> for f32 {
    type Output = RGB;
    fn mul(self, rhs: RGB) -> RGB {
        RGB {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

impl Default for RGB {
    fn default() -> Self {
        Self::black()
    }
}

//TODO: Find more idiomatic way to do this
//Should give black
pub fn write_ppm(buf: &Vec<RGB>, xres: usize, yres: usize, s: String) -> Result<(), Error> {
    //P6 width height 255 \n
    //R G B
    //TODO: better error handling
    let path = Path::new(&s);
    let mut file = File::create(path)?;
    let header = format!("P6 {} {} 255\n", xres, yres);
    file.write_all(header.as_bytes())?;
    let buf: Vec<u8> = buf
        .into_iter()
        .flat_map(|x| [x.r as u8, x.g as u8, x.b as u8])
        .collect();
    file.write_all(&buf)?;
    Ok(())
}

//Implement Mut
