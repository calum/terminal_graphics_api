//! A zero-dependency, 100% Rust graphics API for drawing in a terminal.
//!
//! # How it works
//!
//! 1. Create a [`Display`]: an in-memory grid of cells. Each cell holds one
//!    character, a foreground [`Colour`] and a background [`Colour`].
//! 2. Draw into it. Either call [`Display::set_pixel`] directly, or build
//!    something that implements [`graphics::Graphic`] (this crate provides
//!    [`Text`], [`Container`], [`shapes::Rect`] and [`shapes::Ball`]) and call
//!    its `draw` method.
//! 3. Show it with [`Display::print`]. Nothing appears in the terminal until
//!    you do. [`Display::render`] and [`Display::write_to`] give you the same
//!    output as a `String` or on any writer, which is handy for tests.
//!
//! Coordinates are `(x, y)` with `(0, 0)` at the top-left; `x` grows to the
//! right and `y` grows downwards. Anything drawn outside the display is
//! silently clipped, so graphics may safely move off-screen.
//!
//! # Quick start
//!
//! ```
//! use terminal_graphics::graphics::Graphic;
//! use terminal_graphics::{Colour, Display, Text};
//!
//! let mut display = Display::new(20, 5);
//!
//! // a single cell
//! display.set_pixel(0, 0, '*', Colour::Yellow, Colour::Black);
//!
//! // something drawable
//! Text::from(2, 2, "Hello!".to_string(), Colour::Green, Colour::Black).draw(&mut display);
//!
//! // `display.print()` would put it on screen; `render()` returns the same bytes
//! let output = display.render();
//! assert!(output.contains('*') && output.contains('H'));
//! ```
//!
//! # Animating
//!
//! [`graphics::Graphics`] keeps named graphics and redraws them all, in the
//! order they were added, each time you call `draw`:
//!
//! ```
//! use terminal_graphics::graphics::Graphics;
//! use terminal_graphics::shapes::Ball;
//! use terminal_graphics::{Colour, Display};
//!
//! let mut display = Display::new(30, 10);
//! let mut graphics = Graphics::new();
//! graphics.add_named("ball", Box::new(Ball::new(0.0, 5.0, 1.0, 0.0, Colour::Red)));
//!
//! for _ in 0..3 {
//!     graphics.get_mut_named("ball").unwrap().update_position();
//!     graphics.render_to(&mut display); // use `graphics.draw(&mut display)` to also print
//! }
//! ```
//!
//! Run the bundled example with `cargo run --example demo`.
#![forbid(unsafe_code)]

pub mod display;
pub mod graphics;

pub use display::colour::Colour;
pub use display::display::Display;

// re-export the most commonly used graphics
pub use graphics::container::Container;
pub use graphics::shapes;
pub use graphics::text::Text;
