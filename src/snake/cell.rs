use std::io::Stdout;

#[derive(Default)]
pub struct Cell{
    pub x: usize,
    pub y: usize,
    pub object: Option<Box<dyn Display>>
}

impl Cell{
    pub fn new(x: usize, y:usize) -> Cell{
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
