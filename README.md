# OFFFLIX

A simple application to auto manage series on your local storage.

## Features

- [x] Resume watching
- [x] Play next episode
- [x] Auto increment season
- [x] Play random episode
- [ ] GUI

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
Modify the root argument of main.rs to your series root directory.

```
cargo run --release
```

