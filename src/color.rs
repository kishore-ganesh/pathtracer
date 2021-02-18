pub struct RGB{
    r: f32, 
    g: f32, 
    b: f32
}
//We'll floor it later 

//RGB grid
//For light accumulation: +, for rest, multiply?
impl RGB {
    fn to_file(&self){
        
    }

    pub fn black() -> Self {
        return RGB{r: 0.0, g: 0.0,b: 0.0};
    }
     
}

//Implement Mut
