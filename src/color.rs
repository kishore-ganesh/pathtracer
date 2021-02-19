use std::fs::File;
use std::io::{Write, Error};
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
    

    pub fn black() -> Self {
        return RGB{r: 0.0, g: 0.0,b: 0.0};
    }
     
}
//TODO: Find more idiomatic way to do this 
//Should give black 
pub fn write_ppm(v: Vec<Vec<RGB>>,s: String) -> Result<String, Error>{
    //P6 width height 255 \n 
    //R G B 
    let path = Path::new(&s);
    let mut file = File::create(&path)?;
    let header = format!("P6 {} {} 255 \n", v[0].len(), s.len());
    file.write_all(header.as_bytes())?;
    for row in v {
        let mut col_str = String::from("");
        for col in row {
            col_str += format!("{} {} {} ", col.r as i32, col.g as i32, col.b as i32).as_str();
        }
        col_str += "\n";
        file.write_all(col_str.as_bytes())?;
    }
    Ok("Successful".to_string())
}

//Implement Mut
