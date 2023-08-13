use crossterm::{
    cursor::{Hide, MoveTo},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind},
    queue,
    style::Print,
    terminal::{Clear, ClearType, SetTitle},
};
use std::io::{stdout, Stdout, Write};
use std::time::Duration;

mod snake;
use snake::{direction::Direction, game::Game, game_settings::GameSettings};
mod modes;
use modes::color_mode::ColorMode;


fn main() {
    //default settings
    let mut max_width = 20;
    let mut max_height = 11;
    let mut color_mode = ColorMode::Mono;
    let mut terminal_refresh_time: i32 = 125;

    //argument parsing
    let arguments: Vec<String> = std::env::args().filter(|arg| arg.starts_with('-')).collect();
    for mut arg in arguments{
        match arg.as_str(){
            "--multi" | "-m" => {
                color_mode = ColorMode::Multi;
            },
            "--small" | "-S" => {
                max_width = 10;
                max_height = 6;
            }
            "--huge" | "-H" => {
                max_width = 40;
                max_height = 23;
            }
            "--slow" | "-s" => {
                terminal_refresh_time = 220;
            }
            "--fast" | "-f" => {
                terminal_refresh_time = 60;
            }
            _ => panic!("Invalid argument(s)!")
        }    
    }

    let settings = GameSettings{
        max_width: max_width,
        max_height: max_height,
        color_mode: color_mode
    };
    let game = &mut Game::new(settings);
    
    //additional variabless
    let mut stdout = stdout();
    let mut new_direction: Direction = Direction::Right;
    let mut timer: i32 = terminal_refresh_time;

    prepare_terminal(&mut stdout);
    game.print_game_border(&mut stdout);
    //game loop
    loop {
        match poll(Duration::from_millis(1)) {
            Ok(true) => {
                let event = read().unwrap();
                if let Some(from_input) = parse_input(event) {
                    new_direction = from_input;
                }
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
                if game.win_occured() {
                    win(&mut stdout, settings.max_height);
                    break;     
                }
            } else if game.collision_occured() {
                game_over(&mut stdout, settings.max_height);
                break;
            }
            clear_inside_border(&mut stdout, settings.max_width, settings.max_height);
            game.print_objects(&mut stdout);
            timer = terminal_refresh_time;
        }

        //executing command queue
        stdout.flush().expect("Error while flushing stdout");
    }
}

fn game_over(stdout: &mut Stdout, skip_lines: u16) {
    queue!(
        stdout, 
        MoveTo(0, skip_lines + 1),
        Print("\nGame Over!"),
    ).expect("Error while printing game over");
}

fn win(stdout: &mut Stdout, skip_lines: u16) {
    queue!(
        stdout, 
        MoveTo(0, skip_lines + 1),
        Print("\nCongrats! Yow won!"),
    ).expect("Error while printing game over");
}

fn clear_terminal(stdout: &mut Stdout) {
    queue!(
        stdout,
        Clear(ClearType::All),
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

fn parse_input(event: Event) -> Option<Direction> {
    match event.into() {
        Event::Key(KeyEvent {
            code: KeyCode::Up | KeyCode::Char('w'),
            kind: KeyEventKind::Press,
            ..
        }) => Some(Direction::Up),
        Event::Key(KeyEvent {
            code: KeyCode::Right | KeyCode::Char('d'),
            kind: KeyEventKind::Press,
            ..
        }) => Some(Direction::Right),
        Event::Key(KeyEvent {
            code: KeyCode::Down | KeyCode::Char('s'),
            kind: KeyEventKind::Press,
            ..
        }) => Some(Direction::Down),
        Event::Key(KeyEvent {
            code: KeyCode::Left | KeyCode::Char('a'),
            kind: KeyEventKind::Press,
            ..
        }) => Some(Direction::Left),
        _ => None,
    }
}

fn prepare_terminal(stdout: &mut Stdout){
    queue!(stdout, SetTitle("Snake in terminal")).expect("Error while setting terminal title");
    clear_terminal(stdout);
    hide_cursor(stdout);
}