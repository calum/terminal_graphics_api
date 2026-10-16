//! Everything required to draw into the terminal.
//!
//! A [`Display`](display::Display) is an in-memory grid of pixels. Each
//! pixel holds one character, a foreground [`Colour`](colour::Colour) and a
//! background `Colour`. You draw into the grid either directly with
//! [`Display::set_pixel`](display::Display::set_pixel), or by passing the
//! display to the `draw` method of a `Graphic` (for example a shape or
//! text), which then writes its own pixels into the grid.
//!
//! Nothing reaches the terminal while you draw. Call
//! [`Display::print`](display::Display::print) to write the grid to stdout,
//! or [`Display::render`](display::Display::render) to get it as a `String`
//! of ANSI escape codes, or [`Display::write_to`](display::Display::write_to)
//! to send it to any writer. Printing the same display again redraws it in
//! place, which is how animation is done.
//!
//! Coordinates are `(x, y)`, with `(0, 0)` at the top left. `x` grows to the
//! right and `y` grows downwards. Drawing outside the display is silently
//! clipped: pixels that fall outside the grid are skipped rather than causing
//! a panic.
//!
//! ```
//! use terminal_graphics::{Colour, Display};
//!
//! let mut display = Display::new(10, 3);
//! display.set_pixel(1, 1, '#', Colour::Red, Colour::Black);
//!
//! assert!(display.render().contains('#'));
//! ```

pub mod colour;
#[allow(clippy::module_inception)]
pub mod display;
pub mod pixel;
