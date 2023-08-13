# Snake Game in Terminal

<a>A classic Snake game implemented in Rust, playable directly in the terminal. Use arrow keys or WASD to control the snake and try to eat as much food as possible without colliding with the snake itself. The game offers multiple settings for board size, color mode, and snake speed.</a>

## Cloning the Repository
1. Open your terminal.
2. Clone the repository using Git:
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

## Running a Standalone Executable
If you want to share the game as a standalone executable, you can build it using Cargo.

1. Downlaod an executable from a <a href="https://github.com/zhonissovsuleimen/terminal-snake/releases">latest release</a>.

2. Run the executable directory directly. You can also run it from the terminal:
``
.\terminal-snake.exe
``

# Command-line Options
```
**-h** or **--help**      Show this help message and exit
**-m** or **--multi**     Enable multi-color mode
**-S** or **--small**     Set small game board
**-H** or **--huge**      Set huge game board
**-s** or **--slow**      Set slow snake speed
**-f** or **--fast**      Set fast snake speed
```
