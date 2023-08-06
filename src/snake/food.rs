use std::io::Stdout;
use crossterm::{
    queue,
    style::{ Print, Color, SetForegroundColor}
};
use crate::snake::cell::Display;

pub struct Food{}

impl Display for Food{
    fn display(&self, stdout: &mut Stdout){
        queue!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print('+')
        ).expect("Error while displaying food character")
    }
} 