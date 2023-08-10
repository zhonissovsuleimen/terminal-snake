use crossterm::{
    cursor::{Hide, MoveTo},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind},
    queue,
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Stdout, Write};
use std::time::Duration;

mod snake;
use snake::{direction::Direction, game::Game};

fn main() {
    let mut stdout = stdout();

    const TICK_MS: i32 = 125;
    let mut timer: i32 = TICK_MS;
    let game = &mut Game::new(40, 23);
    let mut new_direction: Direction = Direction::Right;
    hide_cursor(&mut stdout);
    //game loop
    loop {
        match poll(Duration::from_millis(1)) {
            Ok(true) => {
                let asd_event = read().unwrap();

                new_direction = match asd_event.into() {
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
                timer -= 1;
            }
            _ => {
                timer -= 1;
            }
        }
        if timer < 0 {
            clear_terminal(&mut stdout);
            game.change_direction(new_direction);
            game.make_move();
            game.display(&mut stdout);
            timer = TICK_MS;
        }

        //executing command queue
        stdout.flush().expect("Error while flushing stdout");
    }
}

fn clear_terminal(stdout: &mut Stdout) {
    queue!(stdout, Clear(ClearType::FromCursorUp), MoveTo(0, 0))
        .expect("Error while clearing the terminal");
}

fn hide_cursor(stdout: &mut Stdout) {
    queue!(stdout, Hide).expect("Error while hiding cursor");
}
