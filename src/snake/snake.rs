use std::io::Stdout;
use crossterm::{
    queue,
    style::{Print, Color, SetForegroundColor}
};

use super::direction::Direction;

pub struct Snake{
    x: u32,
    y: u32,
    x_limit: u32,
    y_limit: u32,
    direction: Direction,
    next: Option<Box<Snake>>,
    color: Color,
}

impl Snake{
    pub fn new(x: u32, y: u32, x_limit: u32, y_limit: u32) -> Snake{
        Snake{
            x: x,
            y: y,
            x_limit: x_limit,
            y_limit: y_limit,
            direction: Direction::Right,
            next: None,
            color: Color::White
        }
    }
    
    pub fn with_color(x: u32, y: u32, x_limit: u32, y_limit: u32, color: Color) -> Snake{
        Snake{
            x: x,
            y: y,
            x_limit: x_limit,
            y_limit: y_limit,
            direction: Direction::Right,
            next: None,
            color: color
        }
    }

    pub fn make_move(&mut self){
        let start_x = self.x;
        let start_y = self.y;

        match self.direction{
            Direction::Up => {
                self.x = (self.x - 1) % self.x_limit;
            }
            Direction::Right => {
                self.y = (self.y + 1) % self.y_limit;
            }
            Direction::Down => {
                self.x = (self.x + 1) % self.x_limit;
            }
            Direction::Left => {
                self.y = (self.y - 1) % self.y_limit;
            }
        }

        if let Some(next_body) = self.next.as_mut() {
            next_body.move_to(start_x, start_y);
        }
    }

    fn move_to(&mut self, x: u32, y: u32){
        let start_x = self.x;
        let start_y = self.y;

        self.x = x;
        self.y = y; 

        if let Some(next_body) = self.next.as_mut() {
            next_body.move_to(start_x, start_y);
        }
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
            Print('\u{25A0}')
        ).expect("Error while displaying snake character");
    }
}

pub trait Display{
    fn display(&self, stdout: &mut Stdout);
}