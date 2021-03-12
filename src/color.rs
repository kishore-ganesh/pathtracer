use std::fs::File;
use std::io::{Write, Error};
use std::ops::{Add,Sub, AddAssign, Div, DivAssign, Mul};
use std::path::Path;
#[derive(Copy, Clone, Debug)]
pub struct RGB{
    r: f32, 
    g: f32, 
    b: f32
}
//We'll floor it later 

//RGB grid
//For light accumulation: +, for rest, multiply?
impl RGB {
    
    pub fn create(r: f32, g: f32, b: f32) -> Self{
        return RGB{r: r, g: g, b: b};
    }
    pub fn black() -> Self {
        return RGB{r: 0.0, g: 0.0,b: 0.0};
    }
    pub fn is_black(&self) -> bool {
        return self.r<=0.0 && self.g<=0.0 && self.b <= 0.0;
    }
     
}

fn clamp(x: f32, l: f32, r: f32) -> f32{
    return x;
    if(x < l){
        return l;
    } 
    if(x > r){
        return r;
    } 
    return x;

}

fn clamp_rgb(x: RGB, l: f32, r: f32) -> RGB{
    return RGB::create(
        clamp(x.r, l, r),
        clamp(x.g, l, r),
        clamp(x.b, l, r)
    )
}
impl Add for RGB{
    type Output = RGB;
    fn add(self, other: Self) -> Self{
        return Self{
            r: ((self.r+other.r)),
            g: ((self.g+other.g)),
            b: ((self.b+other.b)),
        };
    }
}

impl Sub<f32> for RGB{
    type Output = RGB;
    fn sub(self, other: f32) -> Self{
        return clamp_rgb(Self{
            r: ((self.r-other)),
            g: ((self.g-other)),
            b: ((self.b-other)),
        }, 0.0,255.0);
    }
}

impl AddAssign for RGB{
    fn add_assign(&mut self, other: Self){
        //TODO: handle overflow
        //TODO: check this syntax
        *self = clamp_rgb(Self{
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b
        }, 0.0,255.0);
    }
}


impl Div<f32> for RGB {
    type Output = Self;
    fn div(self, rhs: f32) -> Self{
        return clamp_rgb(Self{
            r: self.r/rhs,
            g: self.g/rhs,
            b: self.b/rhs
        }, 0.0, 255.0);
    }
}


impl DivAssign<f32> for RGB {
    fn div_assign(&mut self, rhs: f32){
        *self = clamp_rgb(Self{
            r: self.r/rhs,
            g: self.g/rhs,
            b: self.b/rhs
        }, 0.0, 255.0);
    }
}

impl Mul<f32> for RGB{
    type Output = Self;
    fn mul(self, rhs: f32) -> Self{
        return clamp_rgb(Self{
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs
        }, 0.0, 255.0);
    }
}
impl Mul<RGB> for RGB{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self{
        return clamp_rgb(Self{
            r: (self.r * rhs.r)/255.0,
            g: (self.g * rhs.g)/255.0,
            b: (self.b * rhs.b)/255.0
        }, 0.0, 255.0);
    }
}
//TODO: Find more idiomatic way to do this 
//Should give black 
pub fn write_ppm(v: &Vec<Vec<RGB>>,s: String) -> Result<String, Error>{
    //P6 width height 255 \n 
    //R G B 
    //TODO: better error handling
    let path = Path::new(&s);
    let mut file = File::create(&path)?;
    let header = format!("P6 {} {} 255\n", v[0].len(), v.len());
    file.write_all(header.as_bytes())?;
    for row in v {
        let mut col_str = String::from("");
        for col in row {
            file.write_all(&[col.r as u8]);
            //file.write_all(" ".as_bytes());
            file.write_all(&[col.g as u8]);
            //file.write_all(" ".as_bytes());
            file.write_all(&[col.b as u8]);
            //file.write_all(" ".as_bytes());
        }
        //file.write_all("\n".as_bytes());
    }
    Ok("Successful".to_string())
}

//Implement Mut
