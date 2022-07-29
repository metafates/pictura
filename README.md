<h1 align="center">Pictura</h1>  
<p align="center">
    <img width="200" src="assets/logo.png">
</p>

<h3 align="center">
    Wallpaper Manager 
</h3>

## Table of Contents

- [About](#about)
- [Usage](#usage)
- [Installation](#installation)

## About

Pictura is a wallpaper manager that automatically
gathers wallpapers metadata, compresses them and generates
a fancy static html page.

## Usage

```bash
mkdir mywalls
cd mywalls

# This will initialize pictura and create `Wallpapers` folder
# See `pictura init --help` for more info
pictura init 

# Add some wallpapers
mv oldwalls/* Wallpapers

# Dirs inside `Wallpaper` will be treated as categories 
mkdir Wallpapers/Nature
mv oldwalls/*forest* Wallpapers/Nature

# Generate the page. Now you can visit generated `index.html` 
pictura sync
```

You can edit gallery configuration file at `.pictura/config.toml`

## Installation

### Using cargo

> Don't have cargo installed? [Download it here](https://doc.rust-lang.org/cargo/getting-started/installation.html)

```bash
cargo install pictura
```