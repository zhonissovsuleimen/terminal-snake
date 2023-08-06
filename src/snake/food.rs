use std::io::Stdout;
use crossterm::{
    queue,
    Result,
    style::{ Print, Color, SetForegroundColor}
};
use crate::snake::cell::Display;

struct Food{}

impl Display for Food{
    fn display(&self, stdout: &mut Stdout) -> Result<()>{
        queue!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print('+')
        )
    }
} 