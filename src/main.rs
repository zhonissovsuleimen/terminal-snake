use std::io::{stdout, Write, Stdout};
use std::thread;
use std::time::Duration;
use crossterm::{
    queue,
    terminal::{Clear, ClearType},
    style::{Print, ResetColor},
    cursor::{MoveTo, Hide}
};

mod snake;
use snake::cell::Cell;

fn main() {
    let mut stdout = stdout();
    const FIELD_WIDTH: usize = 32;
    const FIELD_HEIGHT: usize = 16;
    const TICK_DELAY_MS: u64 = 250; 
    let mut game_field: [[Cell; FIELD_WIDTH]; FIELD_HEIGHT] = Default::default();

    for i in 0..FIELD_HEIGHT{
        for j in 0..FIELD_WIDTH{
            game_field[i][j] = Cell::new(i as u32, j as u32);
        }
    }

    hide_cursor(&mut stdout);
    //game loop
    loop{
        clear_terminal(&mut stdout);

        for i in 0..FIELD_HEIGHT{
            for j in 0..FIELD_WIDTH{
                match &game_field[i][j].object{
                    Some(b) => b.display(&mut stdout),
                    None => print_character(&mut stdout, '.')
                }
                reset_color(&mut stdout);
            }
            new_line(&mut stdout);
        }

        //executing command queue 
        stdout.flush().expect("Error while flushing stdout");
        thread::sleep(Duration::from_millis(TICK_DELAY_MS));
    }
}

fn clear_terminal(stdout: &mut Stdout){
    queue!(
        stdout,
        Clear(ClearType::FromCursorUp),
        MoveTo(0,0)
    ).expect("Error while clearing the terminal");
}

fn reset_color(stdout: &mut Stdout){
    queue!(
        stdout,
        ResetColor
    ).expect("Error while reseting terminal color");
}

fn new_line(stdout: &mut Stdout){
    queue!(
        stdout,
        Print("\n")
    ).expect("Error while printing new line");
}

fn print_character(stdout: &mut Stdout, character: char){
    queue!(
        stdout,
        Print(character)
    ).expect("Error while printing fill character");
}

fn hide_cursor(stdout: &mut Stdout){
    queue!(
        stdout,
        Hide
    ).expect("Error while hiding cursor");
}