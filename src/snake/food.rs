use std::io::Stdout;
use crossterm::{
    queue,
    style::{ Print, Color, SetForegroundColor}
};
use crate::snake::snake::Display;

pub struct Food{
    pub x: u16,
    pub y: u16
}

impl Food{
    pub fn new(x: u16, y: u16) -> Food{
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