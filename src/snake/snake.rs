use std::io::Stdout;
use crossterm::{
    queue,
    style::{Print, Color, SetForegroundColor}
};
use crate::snake::cell::Display;

pub struct Snake{
    next: Option<Box<Snake>>,
    color: Color,
    character: char 
}

impl Snake{
    fn new() -> Snake{
        Snake{
            next: None,
            color: Color::White,
            character: '\u{25A0}' 
        }
    }
    
    fn with_color(color: Color) -> Snake{
        Snake{
            next: None,
            color: color,
            character: '\u{25A0}'
        }
    }
}

impl Display for Snake{
    fn display(&self, stdout: &mut Stdout){
        queue!(
            stdout,
            SetForegroundColor(self.color),
            Print(self.character)
        ).expect("Error while displaying snake character");
    }
}