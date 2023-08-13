# Snake Game in Terminal

<a>A classic Snake game implemented in Rust, playable directly in the terminal. Use arrow keys or WASD to control the snake and try to eat as much food as possible without colliding with the snake itself. The game offers multiple settings for board size, color mode, and snake speed.</a>

## Cloning the Repository
1. Open your terminal.
2. Clone the Snake Game repository using Git:
``
git clone https://github.com/zhonissovsuleimen/terminal-snake.git
``

## Running the Game with Cargo
To play the Snake game, you can use Cargo, Rust's package manager and build tool.

1. After cloning the repository, move into the cloned repository:
``
cd terminal-snake
``

2. Build and run the Snake game with Cargo:
``
cargo run
``

## Building a Standalone Executable
If you want to share the game as a standalone executable, you can build it using Cargo.

1. Build the executable:
``
cargo build --release
``

2. The compiled executable will be located in the target/release directory. You can run it directly from the terminal:
``
.\target\release\snake-game.exe
``

# Command-line Options
```
**-h** or **--help**      Show this help message and exit <br>
**-m** or **--multi**     Enable multi-color mode<br>
**-S** or **--small**     Set small game board<br>
**-H** or **--huge**      Set huge game board<br>
**-s** or **--slow**      Set slow snake speed<br>
**-f** or **--fast**      Set fast snake speed<br>
```
