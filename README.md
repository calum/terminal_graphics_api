[![CI](https://github.com/calum/terminal_graphics_api/actions/workflows/ci.yml/badge.svg)](https://github.com/calum/terminal_graphics_api/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/terminal_graphics.svg)](https://crates.io/crates/terminal_graphics)
[![docs.rs](https://docs.rs/terminal_graphics/badge.svg)](https://docs.rs/terminal_graphics)
[![License](https://img.shields.io/crates/l/terminal_graphics.svg)](#license)

# Rust Terminal Graphics API
A zero dependencies, 100% Rust graphics API for the terminal.

## Installation

You need a [Rust toolchain](https://rustup.rs) and a terminal that understands ANSI escape codes (most Linux and macOS terminals do; the library is aimed at Linux).

Add the crate to your project:

```sh
cargo add terminal_graphics
```

or add it to `Cargo.toml` yourself:

```toml
[dependencies]
terminal_graphics = "0.2"
```

## Run the example

```sh
git clone https://github.com/calum/terminal_graphics_api
cd terminal_graphics_api
cargo run --example demo
```

The demo draws a titled box to the terminal. Its source is in [`examples/demo.rs`](examples/demo.rs).

## Your first program

Everything is drawn into a `Display`, a grid of character cells held in memory. Draw things into it, then call `print` to show it. Coordinates are `(x, y)` from the top-left corner, and anything outside the display is clipped.

```rust
use terminal_graphics::graphics::Graphic;
use terminal_graphics::{Colour, Container, Display, Text};

fn main() {
    // A 40 column by 10 row display.
    let mut display = Display::new(40, 10);

    // Reserve screen space: `print` moves the cursor up over the display
    // before drawing, so it needs 10 lines to draw on.
    for _ in 0..display.size().1 {
        println!();
    }

    // Some text, placed at (x = 2, y = 1) relative to its container.
    let mut title = Text::from(2, 1, "Hello, terminal!".to_string(), Colour::BrightYellow, Colour::Blue);

    // A 20x5 box at (x = 5, y = 2) with a green border and blue background.
    let container = Container::from(5, 2, 20, 5, &mut title, Colour::BrightGreen, Colour::Blue);

    container.draw(&mut display);
    display.print();
}
```

To animate, keep your graphics in a `Graphics` collection, change them each frame (for example with `move_named_graphic`) and call `graphics.draw(&mut display)` in a loop. The [API documentation](https://docs.rs/terminal_graphics) has a worked example, and also covers the other building blocks: `Rect`, `Ball`, and the `Graphic` trait for drawing your own shapes.

## Links
- crates.io: https://crates.io/crates/terminal_graphics
- API documentation: https://docs.rs/terminal_graphics
- Source: https://github.com/calum/terminal_graphics_api
- Issues: https://github.com/calum/terminal_graphics_api/issues

## Example application using the API
For example applications, try [termage](https://github.com/calum/terminal_image_display) or [terminal_game_engine](https://github.com/calum/terminal_game_engine).

### Example Gifs

![example_terminal game](https://raw.githubusercontent.com/calum/terminal_graphics_api/master/assets/terminal_game.gif)

![ferris in the terminal](https://raw.githubusercontent.com/calum/terminal_graphics_api/master/assets/ferris_crab.png)

## License

Licensed under either of MIT or Apache-2.0 at your option.
