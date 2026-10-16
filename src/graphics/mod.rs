//! Everything that is drawn on a [`Display`]
//! is a *graphic*. This module provides the building blocks:
//!
//! - the [`Graphic`] trait: anything drawable. Implement it to make your own
//!   type drawable.
//! - [`Graphics`]: a named collection of boxed graphics. It draws them all in
//!   the order they were added, so later graphics are drawn on top of earlier
//!   ones.
//! - [`container::Container`], [`text::Text`] and the [`shapes`] module
//!   (`Rect`, `Ball`): ready-made graphics.
//!
//! # Coordinates
//!
//! Positions are `(x, y)` pairs of `isize`. The origin `(0, 0)` is the
//! top-left cell of the display, `x` grows to the right and `y` grows down.
//! Anything drawn outside the display edges is clipped (silently ignored), so
//! graphics may safely be placed partly or wholly off screen.
//!
//! # The `Graphic` trait
//!
//! Three methods are required:
//!
//! - `draw` paints the graphic onto a `Display`.
//! - `get_position` returns the graphic's `(x, y)` position.
//! - `set_position` sets the position.
//!
//! The rest have default implementations that you can override:
//!
//! - `get_area` lists the cells the graphic occupies (default: just its
//!   position).
//! - `check_collision` says whether a cell is inside `get_area` (default:
//!   compares against `get_area`).
//! - `move_position` moves by a `(dx, dy)` offset (default: reads the position
//!   and calls `set_position`).
//! - `update_speeds` and `update_position` are hooks for animation. They do
//!   nothing by default. Graphics such as `shapes::Ball` use them to move
//!   themselves each frame.
//!
//! # Drawing a collection
//!
//! [`Graphics`] stores graphics under names, so you can find and move them
//! later:
//!
//! - `add_named` adds a graphic, or replaces the one with the same name in
//!   place.
//! - `move_named_graphic` moves a graphic by name.
//! - `draw` clears the display, draws every graphic and prints the result to
//!   the terminal.
//! - `render_to` does the same as `draw` but does not print. Use it when you
//!   want to inspect or write the output yourself, for example in tests.
//!
//! # Example
//!
//! A custom graphic that occupies a single cell:
//!
//! ```
//! use terminal_graphics::graphics::{Graphic, Graphics};
//! use terminal_graphics::{Colour, Display};
//!
//! // a single-cell graphic
//! struct Dot {
//!     x: isize,
//!     y: isize,
//! }
//!
//! impl Graphic for Dot {
//!     fn draw(&self, display: &mut Display) {
//!         display.set_pixel(self.x, self.y, '.', Colour::Red, Colour::Black);
//!     }
//!
//!     fn get_position(&self) -> (isize, isize) {
//!         (self.x, self.y)
//!     }
//!
//!     fn set_position(&mut self, x: isize, y: isize) {
//!         self.x = x;
//!         self.y = y;
//!     }
//! }
//!
//! let mut graphics = Graphics::new();
//! graphics.add_named("dot", Box::new(Dot { x: 0, y: 0 }));
//! graphics.move_named_graphic("dot", 2, 1);
//!
//! let mut display = Display::new(5, 3);
//! graphics.render_to(&mut display);
//! assert!(display.render().contains('.'));
//! ```

pub mod container;
pub mod shapes;
pub mod text;

use crate::display::display::Display;

/// Anything that can be drawn onto a [`Display`].
///
/// `draw`, `get_position` and `set_position` are required. The other methods
/// have default implementations which you may override. See the
/// [module documentation](crate::graphics) for an overview.
pub trait Graphic {
    /// Paints this graphic onto `display`, at its current position.
    /// Cells outside the display are ignored.
    fn draw(&self, display: &mut Display);

    /// Returns the `(x, y)` position of the graphic, in display cells.
    fn get_position(&self) -> (isize, isize);

    /// Sets the `(x, y)` position of the graphic, in display cells.
    fn set_position(&mut self, pos_x: isize, pos_y: isize);

    /// Returns the cells `(x, y)` occupied by this graphic.
    ///
    /// The default is only the position itself, which suits single-cell
    /// graphics. Override it for graphics that cover several cells.
    fn get_area(&self) -> Vec<(isize, isize)> {
        vec![self.get_position()]
    }

    /// Returns `true` if the cell `(pos_x, pos_y)` is one of the cells in
    /// [`get_area`](Graphic::get_area).
    ///
    /// # Example
    ///
    /// ```
    /// use terminal_graphics::graphics::Graphic;
    /// use terminal_graphics::shapes::Rect;
    /// use terminal_graphics::Colour;
    ///
    /// // covers (0, 0), (1, 0), (0, 1) and (1, 1)
    /// let rect = Rect::new(0, 0, 2, 2, Colour::Red);
    /// assert!(rect.check_collision(1, 1));
    /// assert!(!rect.check_collision(2, 0));
    /// ```
    fn check_collision(&self, pos_x: isize, pos_y: isize) -> bool {
        for &(x, y) in self.get_area().iter() {
            if (x, y) == (pos_x, pos_y) {
                return true;
            }
        }
        false
    }

    /// Moves the graphic by `(dx, dy)` cells, by reading its position and
    /// calling [`set_position`](Graphic::set_position).
    fn move_position(&mut self, dx: isize, dy: isize) {
        let (pos_x, pos_y) = self.get_position();

        self.set_position(pos_x + dx, pos_y + dy);
    }

    /// Multiplies the graphic's speed by `(x, y)`, for animation.
    ///
    /// Does nothing by default. Pass negative values to reverse direction.
    fn update_speeds(&mut self, _x: f32, _y: f32) {}

    /// Advances the graphic by one animation step, for example by adding its
    /// speed to its position.
    ///
    /// Does nothing by default.
    fn update_position(&mut self) {}
}

/// A collection of named graphics, drawn in the order they were added
/// (later graphics are drawn on top of earlier ones).
///
/// Graphics are looked up by name. Names are compared exactly, and each name
/// refers to at most one graphic.
#[derive(Default)]
pub struct Graphics {
    graphics: Vec<(String, Box<dyn Graphic>)>,
}
impl Graphics {
    /// Creates an empty collection. Equivalent to [`Graphics::default`].
    pub fn new() -> Graphics {
        Graphics::default()
    }

    /// Clears the display, draws all the graphics to it, then prints it to
    /// the terminal. See [`render_to`](Graphics::render_to) for the drawing
    /// behaviour.
    pub fn draw(&self, display: &mut Display) {
        self.render_to(display);
        display.print();
    }

    /// Clears the display and draws all the graphics to it, in insertion
    /// order, without printing. Pixels left over from earlier frames are
    /// removed by the clear.
    pub fn render_to(&self, display: &mut Display) {
        display.clear();
        for (_, graphic) in self.graphics.iter() {
            graphic.draw(display);
        }
    }

    /// Moves the graphic called `name` by `(dx, dy)` cells.
    ///
    /// Does nothing if there is no graphic with that name.
    pub fn move_named_graphic(&mut self, name: &str, dx: isize, dy: isize) {
        if let Some(graphic) = self.get_mut_named(name) {
            graphic.move_position(dx, dy);
        }
    }

    /// Adds a graphic under `name`.
    ///
    /// If a graphic with the same name already exists, it is replaced in its
    /// original place in the drawing order. Otherwise the graphic is added on
    /// top of the others.
    pub fn add_named(&mut self, name: &str, graphic: Box<dyn Graphic>) {
        match self.graphics.iter_mut().find(|(n, _)| n == name) {
            Some(entry) => entry.1 = graphic,
            None => self.graphics.push((name.to_string(), graphic)),
        }
    }

    /// Returns a mutable reference to the graphic called `name`, or `None` if
    /// there is none. Use it to change a graphic, such as calling
    /// `set_position`.
    pub fn get_mut_named(&mut self, name: &str) -> Option<&mut Box<dyn Graphic>> {
        self.graphics
            .iter_mut()
            .find(|(n, _)| n == name)
            .map(|(_, g)| g)
    }

    /// Returns a reference to the graphic called `name`, or `None` if there is
    /// none.
    #[allow(clippy::borrowed_box)]
    pub fn get_named(&self, name: &str) -> Option<&Box<dyn Graphic>> {
        self.graphics
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, g)| g)
    }

    /// Removes the graphic called `name`. Does nothing if there is none.
    pub fn remove_named(&mut self, name: &str) {
        self.graphics.retain(|(n, _)| n != name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display::colour::Colour;

    struct Dot(isize, isize);
    impl Graphic for Dot {
        fn draw(&self, display: &mut Display) {
            display.set_pixel(self.0, self.1, '.', Colour::Red, Colour::Black);
        }
        fn get_position(&self) -> (isize, isize) {
            (self.0, self.1)
        }
        fn set_position(&mut self, x: isize, y: isize) {
            self.0 = x;
            self.1 = y;
        }
    }

    #[test]
    fn default_area_is_just_the_position() {
        assert_eq!(Dot(2, 3).get_area(), vec![(2, 3)]);
    }

    #[test]
    fn default_collision_only_hits_the_position() {
        let d = Dot(2, 3);
        assert!(d.check_collision(2, 3));
        assert!(!d.check_collision(3, 2));
    }

    #[test]
    fn default_move_position_adds_the_delta() {
        let mut d = Dot(2, 3);
        d.move_position(-5, 1);
        assert_eq!(d.get_position(), (-3, 4));
    }

    #[test]
    fn default_update_hooks_do_nothing() {
        let mut d = Dot(2, 3);
        d.update_speeds(9.0, 9.0);
        d.update_position();
        assert_eq!(d.get_position(), (2, 3));
    }

    #[test]
    fn move_named_graphic_moves_only_that_graphic() {
        let mut g = Graphics::new();
        g.add_named("a", Box::new(Dot(0, 0)));
        g.add_named("b", Box::new(Dot(5, 5)));
        g.move_named_graphic("a", 1, 2);
        assert_eq!(g.get_named("a").unwrap().get_position(), (1, 2));
        assert_eq!(g.get_named("b").unwrap().get_position(), (5, 5));
    }

    #[test]
    fn get_mut_named_allows_mutation_and_is_none_when_missing() {
        let mut g = Graphics::new();
        g.add_named("a", Box::new(Dot(0, 0)));
        g.get_mut_named("a").unwrap().set_position(7, 7);
        assert_eq!(g.get_named("a").unwrap().get_position(), (7, 7));
        assert!(g.get_mut_named("nope").is_none());
    }

    #[test]
    fn render_to_clears_stale_pixels_first() {
        let mut g = Graphics::new();
        g.add_named("a", Box::new(Dot(0, 0)));
        let mut d = Display::new(2, 1);
        d.set_pixel(1, 0, 'x', Colour::Red, Colour::Red);
        g.render_to(&mut d);
        assert_eq!(d.pixel_mut(1, 0).unwrap().character(), ' ');
        assert_eq!(d.pixel_mut(0, 0).unwrap().character(), '.');
    }

    #[test]
    fn remove_named_removes_only_that_graphic() {
        let mut g = Graphics::new();
        g.add_named("a", Box::new(Dot(0, 0)));
        g.add_named("b", Box::new(Dot(1, 1)));
        g.remove_named("a");
        assert!(g.get_named("a").is_none());
        assert!(g.get_named("b").is_some());
    }

    #[test]
    fn draw_renders_and_prints_without_panicking() {
        let g = Graphics::new();
        g.draw(&mut Display::new(1, 1));
    }
}
