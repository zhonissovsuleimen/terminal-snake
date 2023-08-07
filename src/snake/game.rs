use std::io::Stdout;
use crossterm::{
    queue,
    style::{Print, ResetColor}
};
use crate::snake::{
    snake::Snake,
    food::Food,
};

pub struct Game{
    pub max_width: u32,
    pub max_height: u32,
    snake_pieces: Vec<Snake>,
    food_pieces: Vec<Food>
}

impl Game{
    pub fn new(max_width: u32, max_height: u32) -> Game{

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
        for i in 0..self.max_height{
            for j in 0..self.max_width{
                todo!();
                Self::reset_color(stdout);
            }
            Self::print_character(stdout, '\n');
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