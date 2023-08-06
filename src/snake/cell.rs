use std::io::Stdout;
use crossterm::Result;

pub struct Cell{
    pub x: u32,
    pub y: u32,
    pub object: Option<Box<dyn Display>>
}

pub trait Display{
    fn display(&self, stdout: &mut Stdout) -> Result<()>;
}
