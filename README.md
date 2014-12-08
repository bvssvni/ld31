Ludum Dare 31: Sea Birds' Breakfast
====

My entry for Ludum Dare 31

| Binaries |
|----------|
| [OSX](https://github.com/bvssvni/ld31/raw/master/bin/seabirdsbreakfast-osx.zip) |

![screenshot](./screenshot.png)

To compile you need to install [Rust](http://www.rust-lang.org/)

Open up the Terminal window, navigate to the project folder and type:

```
cargo run
```

## Instructions

* Use left/right/up/down to swim

### Edit streams (modify the game)

1. Change `EDIT` in "src/settings.rs" to `true`
2. Drag & drop with mouse to insert new current streams
3. Hit "S" on the keyboard to print current streams
4. Modify "assets/stream.txt" (no comma behind last number)

### Ship a binary

```
cargo build --features ship
```

1. Copy the "/target/seabirdsbreakfast" into a new folder
2. Copy the "assets" folder to the same folder
3. Zip the folder
