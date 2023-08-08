use std::{io::Stdout, borrow::BorrowMut};
use crossterm::{
    queue,
    style::Print,
    cursor::{SavePosition, RestorePosition, MoveTo}
};
use crate::snake::{
    snake::{Snake, Display},
    food::Food,
};


pub struct Game{
    pub max_width: u16,
    pub max_height: u16,
    pub snake_pieces: Vec<Snake>,
    pub food_piece: Food
}

impl Game{
    pub fn new(max_width: u16, max_height: u16) -> Game{

        //creating base snake
        let center_x = max_width/2;
        let center_y = max_height/2;
        let mut snake_head = Snake::new(center_x, center_y, max_width, max_height);
        let mut snake_body1 = Snake::new(center_x-1, center_y, max_width, max_height);
        let mut snake_body2 = Snake::new(center_x-2, center_y, max_width, max_height);
        
        let mut snake_pieces = vec![snake_head, snake_body1, snake_body2];

        let mut food = Food::new(0,0);

        Game{
            max_width: max_width,
            max_height: max_height,
            snake_pieces,
            food_piece: food
        }
    }

    pub fn display(&self, stdout: &mut Stdout){
        queue!(stdout, SavePosition).expect("Error while saving cursor position");

        //displaying game border
        //top bar
        Self::print_character(stdout, '+');
        for _ in 0..self.max_width{
            Self::print_character(stdout, '-');
        }
        Self::print_character(stdout, '+');
        Self::print_character(stdout, '\n');

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
        
        //displaying food to console        
        queue!(stdout, RestorePosition).expect("Error while restoring position");
        queue!(stdout, MoveTo(self.food_piece.x + 1, self.food_piece.y + 1)).expect("Error while moving cursor");
        self.food_piece.display(stdout);

        //displaying snake to console
        todo!();
    }

    fn print_character(stdout: &mut Stdout, character: char){
        queue!(
            stdout,
            Print(character)
        ).expect("Error while printing fill character");
    }
} 