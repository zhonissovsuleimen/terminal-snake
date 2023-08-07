use std::io::Stdout;
use crossterm::{
    queue,
    style::{ Print, Color, SetForegroundColor}
};
use crate::snake::snake::Display;

pub struct Food{
    x: u32,
    y: u32
}

impl Food{
    pub fn new(x: u32, y: u32) -> Food{
        Food{
            x: x,
            y: y
        }
    }
}

impl Display for Food{
    fn display(&self, stdout: &mut Stdout){
        queue!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print('+')
        ).expect("Error while displaying food character")
    }
} 