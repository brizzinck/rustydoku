# Rustydoku

**Rustydoku** is a game based on Woodoku on the classic puzzle genre developed in [Rust](https://www.rust-lang.org/). 
It combines the gameplay of Woodoku with a modern design and fast performance for Rustaceans)

https://github.com/user-attachments/assets/91a8f7ff-446b-43cc-9f8f-e9cf4ed717a8

## Table of Contents

- [Overview](#overview)
- [How to Play](#how-to-play)
  - [Objective](#objective)
  - [Controls](#controls)
- [Play Online](#play-online)
- [Installation](#installation)
  - [Running via GitHub](#running-via-github)
  - [Install as a CLI Binary](#install-as-a-cli-binary)
- [FAQ](#faq)
- [Tips](#tips)
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
  The game begins automatically when you run `cargo run` from the terminal (desktop version) or click the **Start** button in the browser (web version).

- **Move Pieces:**  
  Use your mouse (or your finger on touch devices) to drag and place the pieces.

- **Restart:**  
  Click the restart button at the top-right corner of the screen. If you lose, you can also restart the game from the Game Over panel.

- **Mute Audio:**  
  Click the audio button at the top-right corner to toggle sound on or off. If you lose, you can also mute audio from the Game Over panel.

## Play Online

You can also play **Rustydoku** directly in your browser via **Itch.io**:

[Play Rustydoku on Itch.io](https://skalse.itch.io/rustydoku)

No installation needed — just click and start playing!

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

### Install as a CLI Binary

You can install Rustydoku system-wide (for your user) using Cargo, so you can run it from anywhere like a regular command:

1. **Clone the repository (if not done yet):**

    ```bash
    git clone https://github.com/brizzinck/rustydoku.git
    cd rustydoku
    ```

2. **Install the binary to `~/.cargo/bin`:**

    ```bash
    cargo install --path .
    ```

3. **Run the game from anywhere:**

    ```bash
    rustydoku
    ```

This installs the game like a normal terminal command, so you can launch it easily without re-entering the project folder each time.

## FAQ

- **Arch/Manjaro Linux (Wayland) Users Can't Launch**

  If you're using **Wayland** on Arch or Manjaro and Rustydoku fails to launch due to GPU detection issues, follow these steps to ensure proper GPU and Vulkan support for Bevy:

  1. **Install Essential Dependencies:**

      Update your system and install the required packages:
      
      ```bash
      sudo pacman -Syu
      sudo pacman -S libx11 pkgconf alsa-lib wayland libxkbcommon
      ```
      
      *Note:* Depending on your sound server, you might also need `pipewire-alsa` or `pulseaudio-alsa`. For Vulkan support, install the ICD loaders:
      
      ```bash
      sudo pacman -S vulkan-icd-loader lib32-vulkan-icd-loader
      ```

  2. **Set the Rendering Backend (Optional):**

      If you experience GPU detection issues, try forcing the Vulkan backend:
      
      ```bash
      export WGPU_BACKEND=vulkan
      ```

  3. **Troubleshoot GPU Detection Issues:**

      If you see errors like:
      
      ```
      Unable to find a GPU! Make sure you have installed required drivers!
      ```
      or
      ```
      vkEnumeratePhysicalDevices failed with ERROR_INITIALIZATION_FAILED
      ```
      
      - **Verify Your GPU:**  
        Run the following command to ensure your NVIDIA GPU is recognized:
        
        ```bash
        lspci -k | grep -A 2 -E "(VGA|3D)"
        ```
        
        ```
        09:00.0 VGA compatible controller: NVIDIA Corporation TU106 [GeForce RTX 2060 SUPER] (rev a1)
	        Subsystem: Micro-Star International Co., Ltd. [MSI] Device c757
	        Kernel driver in use: nvidia
        ```

      - **Check Vulkan Installation:**  
        Run `vulkaninfo` to verify that Vulkan detects your GPU:
        
        ```bash
        vulkaninfo
        ```
        
        If it fails, try:
        
        ```bash
        prime-run vulkaninfo
        ```
      
      - **Set the Vulkan ICD Loader:**  
        Ensure the Vulkan ICD loader uses NVIDIA’s driver by exporting:
        
        ```bash
        export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/nvidia_icd.json
        ```

  4. **Reinstall and Update NVIDIA Drivers:**

      If necessary, reinstall the NVIDIA drivers:
      
      ```bash
      sudo pacman -S nvidia nvidia-utils
      sudo mkinitcpio -P
      sudo reboot
      ```

  5. **Try a Different Kernel:**

      Some users report better compatibility with the LTS kernel:
      
      ```bash
      sudo pacman -S linux-lts linux-lts-headers
      sudo mkinitcpio -P
      sudo reboot
      ```

  Following these steps should help resolve GPU detection issues on Wayland setups and ensure that Rustydoku runs on your system.

## Tips

- **Run the Game Without Blocking the Terminal (Linux/macOS):**  
  If you're using **Linux** or **macOS**, you can launch **Rustydoku** without locking your terminal (so you can keep using it) by running it in the background:

    ```bash
    rustydoku &
    ```

    or, if you haven't installed it as a binary:

    ```bash
    cargo run --release &
    ```

  This will allow you to continue using the terminal while the game runs in the background.

  To completely detach the game from the terminal and suppress any output:

    ```bash
    nohup rustydoku > /dev/null 2>&1 &
    ```

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
