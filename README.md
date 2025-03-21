# Rustydoku

**Rustydoku** is a game based on Woodoku on the classic puzzle genre developed in [Rust](https://www.rust-lang.org/). 
It combines the gameplay of Woodoku with a modern design and fast performance for Rustaceans)

## Table of Contents

- [Overview](#overview)
- [How to Play](#how-to-play)
  - [Objective](#objective)
  - [Controls](#controls)
- [Installation](#installation)
  - [Running via GitHub](#running-via-github)
- [FAQ](#faq)
- [Contributing](#contributing)
- [License](#license)

## Overview

Rustydoku is a modern puzzle game where your goal is to score as many points as possible by strategically placing figures on a 9x9 grid.
Clear cells by creating specific combinations and enjoy the challenge as the board gradually fills up.

## How to Play

### Objective

- **Grid Setup:**  
  When the game starts, a 9x9 grid is generated. At the beginning, the grid is mostly empty—but you’ll notice that in a chessboard-like pattern,
  3x3 blocks of **Ferris** appear, and that's intentional!

- **Scoring Combinations:**  
  You have two options to clear space and earn points:
  
  1. **Complete a 3x3 Block:**  
     Fill an entire 3x3 block with Ferris.
  
  2. **Complete a Full Row or Column:**  
     Fill an entire row/column (9 cells) with Ferris.
  
  Completing either combination will free up the occupied cells and award you **9 points**.

- **Winning:**  
  There is no defined “win” — the aim is to accumulate as many points as possible.

### Controls

- **Start Game:**  
  The game starts automatically when you run `cargo run`.

- **Move Pieces:**  
  Use your mouse (or your finger on touch devices) to drag and place the pieces.

- **Restart:**  
  Click the restart button at the top-right corner of the screen. If you lose, you can also restart the game from the Game Over panel.

- **Mute Audio:**  
  Click the audio button at the top-right corner to toggle sound on or off. If you lose, you can also mute audio from the Game Over panel.

## Installation

### Running via GitHub

1. **Clone the repository:**

    ```bash
    git clone https://github.com/brizzinck/rustydoku.git
    cd rustydoku
    ```

2. **Build and run the project:**

    ```bash
    cargo run --release
    ```

3. **Enjoy the game!**

## FAQ

- **For Linux Users:**  
  - **What do if i can't run?**  
    Ensure you have the latest Vulkan or OpenGL libraries installed.
    If you encounter performance issues, update your GPU drivers.
    Running the game from a terminal may provide useful logs if troubleshooting is needed.

## Contributing

I welcome contributions to improve Rustydoku! If you’d like to contribute:

1. [Fork the repository](https://github.com/brizzinck/rustydoku/fork).
2. Create a new branch:

    ```bash
    git checkout -b features/your-features
    ```

3. Commit your changes and submit a Pull Request.  

## License

Rustydoku is licensed under the [MIT License](LICENSE). See the LICENSE file for more information.

---

*Created with passion for Rustaceans*
