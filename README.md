# Introduction

A simple Rust Minesweeper game, but the square is now hexagon :).

I stopped here because it's not the that hard, unless I want to make a 12 neighbor hexagon minesweeper. And I don't think I learn anything more, unless I delve into the algorithm side of this project.

## Demo

Check out my website at [https://binh2.github.io/n2_rust-hexagon-minesweeper/](https://binh2.github.io/n2_rust-hexagon-minesweeper/)

Screenshot of the demo website: 

![Screenshot of how normal gameplay looks like](https://github.com/Binh2/n2_rust-hexagon-minesweeper/blob/main/demo-normal.png?raw=true "Normal")
![Screenshot of when you win the game](https://github.com/Binh2/n2_rust-hexagon-minesweeper/blob/main/demo-win.png?raw=true "Win")
![Screenshot of when you lose the game](https://github.com/Binh2/n2_rust-hexagon-minesweeper/blob/main/demo-lose.png?raw=true "Lose")

## Features

* Display cell/number/bomb
* Check win/lose
* Detect mouse event

## Future features (If I decided to do this project again)

1. Redesign the reset button
2. A timer: So I can time myself
3. Mouse event recorder: So I can playback how fast I was winning.
4. Show the leftover amount of bomb
5. Simple database of the best player
6. Make a 12-neighbor hexagon minesweeper
7. Run a algorithm to solve it
8. Rate difficulty.

# How to run this project locally
## Linux
### Ubuntu system dependencies
`apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev`

### Fedora system dependencies
`dnf install libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel`

### Arch Linux system dependencies
`pacman -S pkg-config libx11 libxi mesa-libgl alsa-lib`

## Windows
On windows both MSVC and GNU target are supported, no additional dependencies required.

Also cross-compilation to windows from linux is supported:

`rustup target add x86_64-pc-windows-gnu`

`cargo run --target x86_64-pc-windows-gnu`

## If it doesn't work
Go to [https://github.com/not-fl3/macroquad](https://github.com/not-fl3/macroquad)

## To display the game

`cargo run --bin main` to run the project
`cargo run --bin gen_img` to generate images of the assets

## To display the game in a browser

`basic-http-server .` to run the project on the web

# What I learned

Not much. Just struggling with some of the boolean operations.

# Technologies used

* Rust
* Macroquad (similar to C++ raylib)

# Time to complete

3 days

# Contributors

[Binh2](https://github.com/Binh2)