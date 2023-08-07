use std::io::Stdout;
use crossterm::{
    queue,
    style::{Print, ResetColor},
    cursor::{SavePosition, RestorePosition, MoveTo, self}
};
use crate::snake::{
    snake::Snake,
    food::Food,
};

use super::snake::Display;

pub struct Game{
    pub max_width: u16,
    pub max_height: u16,
    snake_pieces: Vec<Snake>,
    food_pieces: Vec<Food>
}

impl Game{
    pub fn new(max_width: u16, max_height: u16) -> Game{

        //creating base snake
        let center_x = max_height/2;
        let center_y = max_width/2;
        let head = Snake::new(center_x, center_y, max_height, max_width);

        let mut snake_vec = Vec::<Snake>::new();
        snake_vec.push(head);

        let mut food_vec = Vec::<Food>::new();
        food_vec.push(Food::new(0,0));

        Game{
            max_width: max_width,
            max_height: max_height,
            snake_pieces: snake_vec,
            food_pieces: food_vec
        }
    }

    pub fn display(&self, stdout: &mut Stdout){
        queue!(stdout, SavePosition).expect("Error while saving cursor position");

        //printing game border
        //top bar
        Self::print_character(stdout, '+');
        for _ in 0..self.max_width{
            Self::print_character(stdout, '-');
        }
        Self::print_character(stdout, '+');

        //sides
        for _ in 0..self.max_height{
            Self::print_character(stdout, '|');
            for _ in 0..self.max_width{
                Self::print_character(stdout, ' ');
            }
            Self::print_character(stdout, '|');
            Self::print_character(stdout, '\n');
        }

        //bottom bar
        Self::print_character(stdout, '+');
        for _ in 0..self.max_width{
            Self::print_character(stdout, '-');
        }
        Self::print_character(stdout, '+');
        
        //adding food        
        for piece in self.food_pieces.iter(){
            queue!(stdout, RestorePosition).expect("Error while restoring position");
            //moving cursor to food pos and adjusting for border 
            queue!(stdout, MoveTo(piece.x + 1, piece.y + 1)).expect("Error while moving cursor");
            piece.display(stdout);
        }

        //adding snake        
        for piece in self.snake_pieces.iter(){
            queue!(stdout, RestorePosition).expect("Error while restoring position");
            //moving cursor to snake pos and adjusting for border 
            queue!(stdout, MoveTo(piece.x + 1, piece.y + 1)).expect("Error while moving cursor");
            piece.display(stdout);
        }
    }

    fn print_character(stdout: &mut Stdout, character: char){
        queue!(
            stdout,
            Print(character)
        ).expect("Error while printing fill character");
    }

    fn reset_color(stdout: &mut Stdout){
        queue!(
            stdout,
            ResetColor
        ).expect("Error while reseting terminal color");
    }

} 