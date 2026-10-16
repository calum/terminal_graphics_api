[![CI](https://github.com/calum/terminal_graphics_api/actions/workflows/ci.yml/badge.svg)](https://github.com/calum/terminal_graphics_api/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/terminal_graphics.svg)](https://crates.io/crates/terminal_graphics)
[![docs.rs](https://docs.rs/terminal_graphics/badge.svg)](https://docs.rs/terminal_graphics)
[![License](https://img.shields.io/crates/l/terminal_graphics.svg)](#license)

# terminal_graphics
Draw shapes, text and animations in a terminal's character grid with ANSI colours. Zero dependencies, 100% Rust.

It works on **cells**: each cell holds one character, a foreground colour and a background colour, and you draw into an in-memory grid that is printed in place. It is not an image protocol (Kitty, Sixel, iTerm2) and not a widget toolkit; for those, look elsewhere.

**[Try it in your browser: play a Snake game built with this library](https://calum.github.io/terminal_graphics_api/)**

## Installation

You need a [Rust toolchain](https://rustup.rs) and a terminal that understands ANSI escape codes (Linux and macOS terminals do, as do recent Windows terminals; the library is aimed at Linux).

Add the crate to your project:

```sh
cargo add terminal_graphics
```

or add it to `Cargo.toml` yourself:

```toml
[dependencies]
terminal_graphics = "1"
```

## Run the example

```sh
git clone https://github.com/calum/terminal_graphics_api
cd terminal_graphics_api
cargo run --example demo
```

The demo is a ~10 second animated tour of the whole library: a custom `Graphic` (plasma) in a `Container`, bouncing `Ball`s with `contains` in a `Graphics` collection, the 16-colour palette, `from_rgb` gradients, clipped scrolling `Text`, and live stats. Pass `-- --frames N` to change the length. Its source is in [`examples/demo.rs`](examples/demo.rs).

![the animated demo](https://raw.githubusercontent.com/calum/terminal_graphics_api/master/assets/demo.gif)

## Your first program

Everything is drawn into a `Display`, a grid of character cells held in memory. Draw things into it, then call `print` to show it. Coordinates are `(x, y)` from the top-left corner, and anything outside the display is clipped.

```rust,no_run
use terminal_graphics::{Colour, Container, Display, Graphic, Text};

fn main() -> std::io::Result<()> {
    // A 40 column by 10 row display.
    let mut display = Display::new(40, 10);

    // Reserve screen space: `print` moves the cursor up over the display
    // before drawing, so it needs 10 lines to draw on.
    display.reserve_space()?;

    // A 24x5 box at (x = 5, y = 2) with a green border and blue background,
    // holding some text. The text's position is relative to the inside of
    // the box, so (1, 1) is one cell in from the top-left of the interior.
    let container = Container::new(5, 2, 24, 5, Colour::BrightGreen, Colour::Blue)
        .with_content(Text::new(1, 1, "Hello, terminal!", Colour::BrightYellow, Colour::Blue));

    container.draw(&mut display);
    display.print()
}
```

## Animating

Keep your graphics in a `Graphics` collection under names. Each frame, advance them with `update` and call `show`, which clears the display, draws every graphic in the order it was added, and prints the result:

```rust,no_run
use std::{thread::sleep, time::Duration};
use terminal_graphics::prelude::*;

fn main() -> std::io::Result<()> {
    let mut display = Display::new(40, 8);
    let mut graphics = Graphics::new();
    graphics
        .add("sky", Rect::new(0, 0, 40, 8, Colour::Blue))
        .add("ball", Ball::new(0.0, 3.0, 1.0, 0.0, Colour::BrightYellow))
        .add("label", Text::new(1, 0, "A ball, moving right", Colour::White, Colour::Blue));

    display.reserve_space()?;
    for _ in 0..38 {
        graphics.update(); // moves every Ball by its speed
        graphics.show(&mut display)?;
        sleep(Duration::from_millis(50));
    }
    Ok(())
}
```

Use `graphics.get_mut("ball")` to change a graphic, `graphics.move_by("label", 1, 0)` to nudge one, and `graphics.remove("ball")` to take one away. Implement the `Graphic` trait to draw your own shapes; `contains(x, y)` tests a cell against a graphic, which is handy for collisions. `use terminal_graphics::prelude::*;` imports everything at once, and the [API documentation](https://docs.rs/terminal_graphics) covers each type.

## Performance

`cargo bench` runs [`benches/render.rs`](benches/render.rs), which uses only `std`. A frame is the cost of turning a `Display` into ANSI output. Colour codes are only sent when the colours change along a row, so uniform and slowly changing frames are small. Numbers from one machine (Windows 11, release build); yours will differ, so run it yourself:

| Frame (80x24)             | Before     | Now      |
|---------------------------|-----------:|---------:|
| `render`, one colour      | 83 us      | 9 us     |
| `render`, 16-colour gradient | 196 us  | 12 us    |
| update + draw + write     | 203 us     | 17 us    |
| output per frame          | 29 KB      | 2.5-3 KB |

At 200x60 a `render` takes 43-51 us and writes 14 KB (it was about 1.2 ms and 181 KB). A real terminal parses every byte you send, so the bytes per frame usually matter more than the CPU time.

## Upgrading from 0.x

Version 1.0 renamed and reshaped much of the API. See [CHANGELOG.md](CHANGELOG.md) for the full list and a before/after migration guide.
## Links
- crates.io: https://crates.io/crates/terminal_graphics
- API documentation: https://docs.rs/terminal_graphics
- Website (with a playable game): https://calum.github.io/terminal_graphics_api/
- Source: https://github.com/calum/terminal_graphics_api
- For AI coding agents: [llms.txt](llms.txt)
- Issues: https://github.com/calum/terminal_graphics_api/issues

## Example application using the API
For example applications, try [termage](https://github.com/calum/terminal_image_display) or [terminal_game_engine](https://github.com/calum/terminal_game_engine).

## License

Licensed under either of MIT or Apache-2.0 at your option.
