# Introduction

A simple Rust Minesweeper game, but the square is now hexagon :)

## Features

* Display cell/number/bomb
* Check win/lose
* Detect mouse event

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

## Finally

`cargo run` to run the project

# What I learned

...

# Technologies used

* Rust
* Macroquad (similar to C++ raylib)

# Time to complete

Who knows

# Contributors

[Binh2](https://github.com/Binh2)