# OFFFLIX

A simple application to auto manage series on your local storage.
![Offlix](https://user-images.githubusercontent.com/64161204/215824616-97705990-09f2-4fac-b32d-e2ffcd8e35eb.jpg)

## Features

- [x] Resume watching
- [x] Play next episode
- [x] Auto increment season
- [x] Play random episode
- [x] GUI
- [x] Multithreaded image loading

## Installation
The application depends on libmpv, so you need to install it first.

### Arch Linux
```
sudo pacman -S mpv
```

### Ubuntu
```
sudo apt install mpv
```
### Windows

#### MSYS2
```
pacman -S mingw-w64-x86_64-mpv
```


## Usage

```
cargo run --release
```

### Media player controls

- 'Space' to pause/resume
- 'Ctrl+Left' to go back 10 seconds
- 'Ctrl+Right' to go forward 10 seconds
- 'Shift+Left' to go back 1 second
- 'Shift+Right' to go forward 1 second
- 'Left' to go back 5 seconds
- 'Right' to go forward 5 seconds
- 'Up' increase volume
- 'Down' decrease volume
- 'F' to toggle fullscreen
- 'Esc' to quit
- 'A' to cycle audio tracks forward
- 'Shift+A' to cycle audio tracks backward
- 'Ctrl+A' to toggle audio
- 'V' to cycle video tracks forward
- 'Shift+V' to cycle video tracks backward
- 'S' to cycle subtitle tracks forward
- 'Shift+S' to cycle subtitle tracks backward
- 'Ctrl+S' to toggle subtitles
