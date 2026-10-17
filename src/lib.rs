//! Draw shapes, text and animations in a terminal's character grid with ANSI
//! colours. Zero dependencies, 100% Rust.
//!
//! It works on cells (one character plus foreground and background colours),
//! not pixels: it is not an image protocol such as Kitty, Sixel or iTerm2.
//!
//! Try it in your browser, with a Snake game built only with this crate:
//! <https://calum.github.io/terminal_graphics_api/>
//!
//! # How it works
//!
//! 1. Create a [`Display`]: an in-memory grid of cells. Each cell is a
//!    [`Pixel`]: one character, a foreground [`Colour`] and a background
//!    [`Colour`].
//! 2. Draw into it. Either call [`Display::set_pixel`] directly, or build
//!    something that implements [`Graphic`] (this crate provides [`Text`],
//!    [`Container`], [`shapes::Rect`] and [`shapes::Ball`]) and call its
//!    `draw` method.
//! 3. Show it with [`Display::print`]. Nothing appears in the terminal until
//!    you do. [`Display::render`] and [`Display::write_to`] give you the same
//!    output as a `String` or on any writer, which is handy for tests.
//!
//! Coordinates are `(x, y)` with `(0, 0)` at the top-left; `x` grows to the
//! right and `y` grows downwards. Anything drawn outside the display is
//! silently clipped, so graphics may safely move off-screen. Nothing in the
//! library panics on out-of-range input.
//!
//! # Quick start
//!
//! ```
//! use terminal_graphics::{Colour, Display, Graphic, Text};
//!
//! let mut display = Display::new(20, 5);
//!
//! // a single cell
//! display.set_pixel(0, 0, '*', Colour::Yellow, Colour::Black);
//!
//! // something drawable
//! Text::new(2, 2, "Hello!", Colour::Green, Colour::Black).draw(&mut display);
//!
//! // `display.print()` would put it on screen; `render()` returns the same bytes
//! let output = display.render();
//! assert!(output.contains('*') && output.contains('H'));
//! ```
//!
//! # Animating
//!
//! [`Graphics`] keeps named graphics and draws them all, in the order they
//! were added. A frame is: advance the animation, then
//! [`show`](Graphics::show), which clears the display, draws everything and
//! prints it:
//!
//! ```no_run
//! use std::{thread::sleep, time::Duration};
//! use terminal_graphics::shapes::Ball;
//! use terminal_graphics::{Colour, Display, Graphics};
//!
//! # fn main() -> std::io::Result<()> {
//! let mut display = Display::new(30, 10);
//! let mut graphics = Graphics::new();
//! graphics.add("ball", Ball::new(0.0, 5.0, 1.0, 0.0, Colour::Red));
//!
//! display.reserve_space()?; // once, before the first frame
//! for _ in 0..28 {
//!     graphics.update();
//!     graphics.show(&mut display)?;
//!     sleep(Duration::from_millis(33));
//! }
//! # Ok(())
//! # }
//! ```
//!
//! For everything in one import, use `terminal_graphics::prelude::*`.
//!
//! # Keyboard and mouse input
//!
//! This crate draws; it does not read input. For key presses and mouse events
//! (clicks, drags, scrolling) use the companion crate
//! [`terminal_graphics_interactive`](https://github.com/calum/terminal_graphics_interactive),
//! which builds on this one and re-exports it. It is early (0.1); add it with
//! `cargo add terminal_graphics_interactive`.
//!
//! # Terminals
//!
//! Output uses ANSI escape codes, which Linux and macOS terminals (and recent
//! Windows terminals) understand. Each frame moves the cursor up over the
//! previous one, so call [`Display::reserve_space`] once before the first
//! [`Display::print`].
//!
//! Run the bundled example with `cargo run --example demo`.
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod colour;
mod display;
pub mod graphics;
mod pixel;

pub use colour::{Colour, ParseColourError};
pub use display::Display;
pub use graphics::shapes;
pub use graphics::{Container, Graphic, Graphics, Text};
pub use pixel::Pixel;

// compile and run-check the code blocks in the README, so it can't go stale
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/// Everything you usually need, in one import:
///
/// ```
/// use terminal_graphics::prelude::*;
///
/// let mut display = Display::new(10, 2);
/// let mut graphics = Graphics::new();
/// graphics.add("hi", Text::new(0, 0, "hi", Colour::Red, Colour::Black));
/// graphics.draw(&mut display);
/// assert_eq!(display.pixel(0, 0).unwrap().character(), 'h');
/// ```
pub mod prelude {
    pub use crate::shapes::{Ball, Rect};
    pub use crate::{Colour, Container, Display, Graphic, Graphics, Pixel, Text};
}
