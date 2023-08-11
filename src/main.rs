use crossterm::{
    cursor::{Hide, MoveTo},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind},
    queue,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Stdout, Write};
use std::time::Duration;

mod snake;
use snake::{direction::Direction, game::Game};

fn main() {
    let mut stdout = stdout();
    const MAX_WIDTH: u16 = 40;
    const MAX_HEIGHT: u16 = 23;
    let game = &mut Game::new(MAX_WIDTH, MAX_HEIGHT);
    let mut new_direction: Direction = Direction::Right;
    const TICK_MS: i32 = 125;
    let mut timer: i32 = TICK_MS;

    clear_terminal(&mut stdout);
    hide_cursor(&mut stdout);
    game.print_game_border(&mut stdout);
    //game loop
    loop {
        match poll(Duration::from_millis(1)) {
            Ok(true) => {
                let event = read().unwrap();

                new_direction = match event.into() {
                    Event::Key(KeyEvent {
                        code: KeyCode::Up | KeyCode::Char('w'),
                        kind: KeyEventKind::Press,
                        ..
                    }) => Direction::Up,
                    Event::Key(KeyEvent {
                        code: KeyCode::Right | KeyCode::Char('d'),
                        kind: KeyEventKind::Press,
                        ..
                    }) => Direction::Right,
                    Event::Key(KeyEvent {
                        code: KeyCode::Down | KeyCode::Char('s'),
                        kind: KeyEventKind::Press,
                        ..
                    }) => Direction::Down,
                    Event::Key(KeyEvent {
                        code: KeyCode::Left | KeyCode::Char('a'),
                        kind: KeyEventKind::Press,
                        ..
                    }) => Direction::Left,
                    _ => new_direction,
                };
            }
            _ => {}
        }

        timer -= 1;
        if timer < 0 {
            game.change_direction(new_direction);
            game.make_move();
            if game.consumption_occured() {
                game.respawn_food();
                game.grow();
            } else if game.collision_occured() {
                game_over(&mut stdout, MAX_HEIGHT);
                break;
            }
            clear_inside_border(&mut stdout, MAX_WIDTH, MAX_HEIGHT);
            game.print_objects(&mut stdout);
            timer = TICK_MS;
        }

        //executing command queue
        stdout.flush().expect("Error while flushing stdout");
    }
}

fn game_over(stdout: &mut Stdout, max_height: u16) {
    queue!(
        stdout, 
        MoveTo(0, max_height + 1),
        Print("\nGame Over!"),
    ).expect("Error while printing game over");
}

fn clear_terminal(stdout: &mut Stdout) {
    queue!(
        stdout,
        Clear(ClearType::FromCursorUp),
        MoveTo(0, 0)
    ).expect("Error while clearing the terminal");
}

fn clear_inside_border(stdout: &mut Stdout, max_width: u16, max_height: u16) {
    for y in 1..max_height + 1 {
        for x in 1..max_width + 1 {
            queue!(
                stdout,
                MoveTo(x, y),
                Print(' ')
            ).expect("Error clearing inside inside border");
        }
    }
}

fn hide_cursor(stdout: &mut Stdout) {
    queue!(stdout, Hide).expect("Error while hiding cursor");
}
