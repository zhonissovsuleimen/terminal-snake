use super::{direction::Direction, game_object::GameObject};
use crossterm::{
    cursor::{MoveTo, RestorePosition, SavePosition},
    queue,
    style::Print,
};
use rand::Rng;
use std::io::Stdout;
pub struct Game {
    pub max_width: u16,
    pub max_height: u16,
    current_direction: Direction,
    pub objects: Vec<GameObject>,
}

impl Game {
    pub fn new(max_width: u16, max_height: u16) -> Game {
        //creating base snake
        let center_x = max_width / 2;
        let center_y = max_height / 2;
        let snake_head = GameObject::Snake(center_x, center_y);
        let snake_body1 = GameObject::Snake(center_x - 1, center_y);
        let snake_body2 = GameObject::Snake(center_x - 2, center_y);

        let food = GameObject::Food(0, 0);
        let objects = vec![snake_head, snake_body1, snake_body2, food];

        Game {
            max_width: max_width,
            max_height: max_height,
            current_direction: Direction::Right,
            objects: objects,
        }
    }

    pub fn display(&self, stdout: &mut Stdout) {
        self.print_game_border(stdout);
        //displaying objects to console
        for obj in &self.objects {
            Self::display_object(stdout, *obj);
        }
    }

    pub fn make_move(&mut self) {
        let mut new_x: u16 = 0;
        let mut new_y: u16 = 0;

        if let GameObject::Snake(x, y) = self.objects.get(0).unwrap() {
            new_x = *x;
            new_y = *y;
        }

        match self.current_direction {
            Direction::Up => new_y = new_y.wrapping_add(self.max_height - 1) % self.max_height,
            Direction::Right => new_x = (new_x + 1) % self.max_width,
            Direction::Down => new_y = (new_y + 1) % self.max_height,
            Direction::Left => new_x = new_x.wrapping_add(self.max_width - 1) % self.max_width,
        };

        for (i, obj) in self.objects.clone().into_iter().enumerate() {
            match obj {
                GameObject::Snake(x, y) => {
                    self.objects[i] = GameObject::Snake(new_x, new_y);
                    new_x = x;
                    new_y = y
                }
                _ => {}
            }
        }
    }

    pub fn change_direction(&mut self, new_dir: Direction) {
        match self.current_direction {
            Direction::Up => {
                if new_dir != Direction::Down {
                    self.current_direction = new_dir;
                }
            }
            Direction::Right => {
                if new_dir != Direction::Left {
                    self.current_direction = new_dir;
                }
            }
            Direction::Down => {
                if new_dir != Direction::Up {
                    self.current_direction = new_dir;
                }
            }
            Direction::Left => {
                if new_dir != Direction::Right {
                    self.current_direction = new_dir;
                }
            }
        }
    }

    pub fn respawn_food(&mut self) {
        let new_x = rand::thread_rng().gen_range(0..self.max_width);
        let new_y = rand::thread_rng().gen_range(0..self.max_height);
        for (i, obj) in self.objects.clone().into_iter().enumerate() {
            match obj {
                GameObject::Food(_, _) => {
                    self.objects[i] = GameObject::Food(new_x, new_y);
                    return;
                }
                _ => {}
            }
        }
    }

    pub fn grow(&mut self) {
        //position of new snake body shouldn't matter,
        //as it will be adjusted in next make_move() call
        let (x, y) = self.get_snake_head_pos();
        self.objects.push(GameObject::Snake(x, y));
    }

    fn print_game_border(&self, stdout: &mut Stdout) {
        //top bar
        Self::print_character(stdout, '+');
        for _ in 0..self.max_width {
            Self::print_character(stdout, '-');
        }
        Self::print_character(stdout, '+');
        Self::print_character(stdout, '\n');

        //sides
        for _ in 0..self.max_height {
            Self::print_character(stdout, '|');
            for _ in 0..self.max_width {
                Self::print_character(stdout, ' ');
            }
            Self::print_character(stdout, '|');
            Self::print_character(stdout, '\n');
        }

        //bottom bar
        Self::print_character(stdout, '+');
        for _ in 0..self.max_width {
            Self::print_character(stdout, '-');
        }
        Self::print_character(stdout, '+');
    }

    fn print_character(stdout: &mut Stdout, character: char) {
        queue!(stdout, Print(character)).expect("Error while printing fill character");
    }

    fn display_object(stdout: &mut Stdout, obj: GameObject) {
        let character: char;
        match obj {
            GameObject::Snake(_, _) => character = '\u{2588}',
            GameObject::Food(_, _) => character = '*',
        }

        match obj {
            GameObject::Snake(x, y) | GameObject::Food(x, y) => {
                queue!(stdout, SavePosition).expect("Error while saving cursor position");
                queue!(stdout, MoveTo(x + 1, y + 1), Print(character))
                    .expect("Error occured while printing character");
                queue!(stdout, RestorePosition).expect("Error while restoring cusror position");
            }
        }
    }

    pub fn consumption_occured(&self) -> bool {
        self.get_food_pos() == self.get_snake_head_pos()
    }

    fn get_food_pos(&self) -> (u16, u16) {
        for obj in self.objects.iter() {
            match obj {
                GameObject::Food(x, y) => {
                    return (*x, *y);
                }
                _ => {}
            }
        }
        (0, 0)
    }

    fn get_snake_head_pos(&self) -> (u16, u16) {
        for obj in self.objects.iter() {
            match obj {
                GameObject::Snake(x, y) => {
                    return (*x, *y);
                }
                _ => {}
            }
        }
        (0, 0)
    }
}
