use std::io::{stdout, Write, Stdout};
use std::thread;
use std::time::Duration;
use crossterm::{
    queue,
    terminal::{Clear, ClearType},
    cursor::{MoveTo, Hide}
};

mod snake;
use snake::game::Game;

fn main() {
    let mut stdout = stdout();
    
    const TICK_DELAY_MS: u64 = 200; 
    let game = &mut Game::new(40, 23);

    hide_cursor(&mut stdout);
    //game loop
    loop{
        clear_terminal(&mut stdout);
        game.display(&mut stdout);
        game.make_move();
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

fn hide_cursor(stdout: &mut Stdout){
    queue!(
        stdout,
        Hide
    ).expect("Error while hiding cursor");
}