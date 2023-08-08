use std::io::Stdout;
use crossterm::{
    queue,
    style::{Print, Color, SetForegroundColor, ResetColor}
};

use super::direction::Direction;

pub struct Snake{
    pub x: u16,
    pub y: u16,
    x_limit: u16,
    y_limit: u16,
    direction: Direction,
    color: Color
}

impl Snake{
    pub fn new(x: u16, y: u16, x_limit: u16, y_limit: u16) -> Snake{
        Snake{
            x: x,
            y: y,
            x_limit: x_limit,
            y_limit: y_limit,
            direction: Direction::Right,
            color: Color::White
        }
    }
    
    pub fn with_color(x: u16, y: u16, x_limit: u16, y_limit: u16, color: Color) -> Snake{
        Snake{
            x: x,
            y: y,
            x_limit: x_limit,
            y_limit: y_limit,
            direction: Direction::Right,
            color: color
        }
    }

    fn move_to(&mut self, x: u16, y: u16){
        let start_x = self.x;
        let start_y = self.y;

        self.x = x;
        self.y = y; 
    }

    pub fn change_direction(&mut self, new_direction: Direction){
        match new_direction{
            Direction::Up => {
                if self.direction != Direction::Down{
                    self.direction = new_direction;
                }
            }
            Direction::Right => {
                if self.direction != Direction::Left{
                    self.direction = new_direction;
                }
            }
            Direction::Down => {
                if self.direction != Direction::Up{
                    self.direction = new_direction;
                }
            }
            Direction::Left => {
                if self.direction != Direction::Right{
                    self.direction = new_direction;
                }
            }
        }
    }
}

impl Display for Snake{
    fn display(&self, stdout: &mut Stdout){
        queue!(
            stdout,
            SetForegroundColor(self.color),
            Print('\u{25A0}'),
            ResetColor
        ).expect("Error while displaying snake character");
    }
}

pub trait Display{
    fn display(&self, stdout: &mut Stdout);
}
