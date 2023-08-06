use std::io::Stdout;
use crossterm::{
    queue,
    style::{Print, ResetColor}
};
use crate::snake::{
    cell::Cell,
    snake::Snake
};

pub struct GameGrid{
    pub max_width: usize,
    pub max_height: usize,
    pub grid: Vec<Vec<Cell>>,
    pub snake_head_pos: (usize, usize)
}

impl GameGrid{
    pub fn new(max_width: usize, max_height: usize) -> GameGrid{

        //creating grid
        let mut grid = Vec::<Vec<Cell>>::new();
        for i in 0..max_height{
            let mut temp_vec = Vec::<Cell>::new();
            for j in 0..max_width{
                temp_vec.push(Cell::new(i, j));
            }
            grid.push(temp_vec);
        };

        //creating base snake
        let head = Snake::new();
        let center_x = max_height/2;
        let center_y = max_width/2;

        grid[center_x][center_y].object = Some(Box::new(head));

        GameGrid {
            max_width: max_width,
            max_height: max_height,
            grid: grid,
            snake_head_pos: (center_x, center_y)
        }
    }

    pub fn display(&self, stdout: &mut Stdout){
        for i in 0..self.max_height{
            for j in 0..self.max_width{
                match &self.grid[i][j].object{
                    Some(b) => b.display(stdout),
                    None => Self::print_character(stdout, '.')
                }
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