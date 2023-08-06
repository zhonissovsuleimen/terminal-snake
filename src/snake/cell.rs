use std::io::Stdout;

#[derive(Default)]
pub struct Cell{
    pub x: u32,
    pub y: u32,
    pub object: Option<Box<dyn Display>>
}

impl Cell{
    pub fn new(x: u32, y:u32) -> Cell{
       Cell{
        x: x,
        y: y,
        object: None
       } 
    }
}

pub trait Display{
    fn display(&self, stdout: &mut Stdout);
}
