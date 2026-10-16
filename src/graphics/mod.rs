//! Everything that is drawn on a [`Display`] is a *graphic*.
//!
//! - the [`Graphic`] trait: anything drawable. Implement it to make your own
//!   type drawable.
//! - [`Graphics`]: a named collection of graphics. It draws them all in the
//!   order they were added, so later graphics are drawn on top of earlier
//!   ones.
//! - ready-made graphics: [`Container`], [`Text`] and the [`shapes`]
//!   ([`Rect`](shapes::Rect), [`Ball`](shapes::Ball)).
//!
//! # Coordinates
//!
//! Positions are `(x, y)` pairs of `isize`. The origin `(0, 0)` is the
//! top-left cell of the display, `x` grows to the right and `y` grows down.
//! Anything drawn outside the display is clipped (silently ignored), so
//! graphics may safely be placed partly or wholly off screen.
//!
//! # The `Graphic` trait
//!
//! Three methods are required:
//!
//! - [`draw`](Graphic::draw) paints the graphic onto a `Display`.
//! - [`position`](Graphic::position) returns the graphic's `(x, y)` position.
//! - [`set_position`](Graphic::set_position) sets the position.
//!
//! The rest have default implementations that you can override:
//!
//! - [`area`](Graphic::area) lists the cells the graphic occupies (default:
//!   just its position) and [`contains`](Graphic::contains) tests one cell
//!   against it. Use them for collision checks.
//! - [`move_by`](Graphic::move_by) moves by a `(dx, dy)` offset.
//! - [`scale_speed`](Graphic::scale_speed) and [`update`](Graphic::update) are
//!   hooks for animation. They do nothing by default; graphics such as
//!   [`Ball`](shapes::Ball) use them to move themselves each frame.
//!
//! # Drawing a collection
//!
//! [`Graphics`] stores graphics under names, so you can find and move them
//! later. [`Graphics::draw`] paints them all, like `Graphic::draw`, and
//! [`Graphics::show`] does the whole frame: clear, paint, print.
//!
//! ```
//! use terminal_graphics::{Colour, Display, Graphic, Graphics};
//!
//! // a custom single-cell graphic
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
//!     fn position(&self) -> (isize, isize) {
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
//! graphics.add("dot", Dot { x: 0, y: 0 });
//! graphics.move_by("dot", 2, 1);
//!
//! let mut display = Display::new(5, 3);
//! graphics.draw(&mut display);
//! assert_eq!(display.pixel(2, 1).unwrap().character(), '.');
//! ```

pub mod container;
pub mod shapes;
pub mod text;

use crate::display::Display;
use std::io;

pub use self::container::Container;
pub use self::text::Text;

/// The part of `start..start + len` that lies inside `0..limit`.
///
/// Used so shapes only visit cells that are on the display, however large or
/// far away they are, and without integer overflow. Empty if there is no
/// overlap.
pub(crate) fn clip(start: isize, len: isize, limit: u32) -> std::ops::Range<isize> {
    let end = start.saturating_add(len.max(0)).min(limit as isize);
    let start = start.max(0);
    start..end.max(start)
}

/// Anything that can be drawn onto a [`Display`].
///
/// [`draw`](Graphic::draw), [`position`](Graphic::position) and
/// [`set_position`](Graphic::set_position) are required. The other methods
/// have default implementations which you may override. See the
/// [module documentation](crate::graphics) for an overview.
pub trait Graphic {
    /// Paints this graphic onto `display`, at its current position.
    /// Cells outside the display are ignored. Does not clear or print.
    fn draw(&self, display: &mut Display);

    /// The `(x, y)` position of the graphic, in display cells.
    fn position(&self) -> (isize, isize);

    /// Sets the `(x, y)` position of the graphic, in display cells.
    fn set_position(&mut self, x: isize, y: isize);

    /// The cells `(x, y)` occupied by this graphic.
    ///
    /// The default is only the position itself, which suits single-cell
    /// graphics. Override it for graphics that cover several cells.
    fn area(&self) -> Vec<(isize, isize)> {
        vec![self.position()]
    }

    /// Whether the cell `(x, y)` is one of the cells in [`area`](Graphic::area).
    /// Override it if you can answer without listing every cell.
    ///
    /// ```
    /// use terminal_graphics::shapes::Rect;
    /// use terminal_graphics::{Colour, Graphic};
    ///
    /// // covers (0, 0), (1, 0), (0, 1) and (1, 1)
    /// let rect = Rect::new(0, 0, 2, 2, Colour::Red);
    /// assert!(rect.contains(1, 1));
    /// assert!(!rect.contains(2, 0));
    /// ```
    fn contains(&self, x: isize, y: isize) -> bool {
        self.area().contains(&(x, y))
    }

    /// Moves the graphic by `(dx, dy)` cells. Saturates instead of
    /// overflowing.
    fn move_by(&mut self, dx: isize, dy: isize) {
        let (x, y) = self.position();
        self.set_position(x.saturating_add(dx), y.saturating_add(dy));
    }

    /// Multiplies the graphic's speed by `(factor_x, factor_y)`, for
    /// animation. Pass a negative factor to reverse that direction.
    ///
    /// Does nothing by default.
    fn scale_speed(&mut self, _factor_x: f32, _factor_y: f32) {}

    /// Advances the graphic by one animation step, for example by adding its
    /// speed to its position. Call it once per frame.
    ///
    /// Does nothing by default.
    fn update(&mut self) {}
}

/// A collection of named graphics, drawn in the order they were added
/// (later graphics are drawn on top of earlier ones).
///
/// Names are compared exactly, and each name refers to at most one graphic.
///
/// ```
/// use terminal_graphics::shapes::Ball;
/// use terminal_graphics::{Colour, Display, Graphics};
///
/// let mut display = Display::new(30, 10);
/// let mut graphics = Graphics::new();
/// graphics.add("ball", Ball::new(0.0, 5.0, 1.0, 0.0, Colour::Red));
///
/// for _ in 0..3 {
///     graphics.update(); // advance every graphic one step
///     graphics.draw(&mut display); // paint them (`show` also clears and prints)
/// }
/// assert_eq!(graphics.get("ball").unwrap().position(), (3, 5));
/// ```
#[derive(Default)]
pub struct Graphics {
    graphics: Vec<(String, Box<dyn Graphic>)>,
}

impl Graphics {
    /// Creates an empty collection.
    pub fn new() -> Graphics {
        Graphics::default()
    }

    /// Adds a graphic under `name`.
    ///
    /// If a graphic with the same name already exists, it is replaced in its
    /// original place in the drawing order. Otherwise the graphic is added on
    /// top of the others. Returns `self` so calls can be chained.
    pub fn add(&mut self, name: impl Into<String>, graphic: impl Graphic + 'static) -> &mut Self {
        let name = name.into();
        let graphic: Box<dyn Graphic> = Box::new(graphic);
        match self.graphics.iter_mut().find(|(n, _)| *n == name) {
            Some(entry) => entry.1 = graphic,
            None => self.graphics.push((name, graphic)),
        }
        self
    }

    /// The graphic called `name`, or `None` if there is none.
    pub fn get(&self, name: &str) -> Option<&dyn Graphic> {
        self.graphics
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, g)| g.as_ref())
    }

    /// A mutable reference to the graphic called `name`, or `None` if there is
    /// none. Use it to change a graphic, for example to call `set_position`.
    pub fn get_mut(&mut self, name: &str) -> Option<&mut dyn Graphic> {
        match self.graphics.iter_mut().find(|(n, _)| n == name) {
            Some((_, graphic)) => Some(&mut **graphic),
            None => None,
        }
    }

    /// Removes the graphic called `name` and returns it, or `None` if there is
    /// none.
    pub fn remove(&mut self, name: &str) -> Option<Box<dyn Graphic>> {
        let index = self.graphics.iter().position(|(n, _)| n == name)?;
        Some(self.graphics.remove(index).1)
    }

    /// Whether there is a graphic called `name`.
    pub fn contains(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    /// Moves the graphic called `name` by `(dx, dy)` cells. Returns `false`,
    /// and does nothing, if there is no graphic with that name.
    pub fn move_by(&mut self, name: &str, dx: isize, dy: isize) -> bool {
        match self.get_mut(name) {
            Some(graphic) => {
                graphic.move_by(dx, dy);
                true
            }
            None => false,
        }
    }

    /// The number of graphics.
    pub fn len(&self) -> usize {
        self.graphics.len()
    }

    /// Whether the collection has no graphics.
    pub fn is_empty(&self) -> bool {
        self.graphics.is_empty()
    }

    /// Calls [`Graphic::update`] on every graphic, advancing the animation by
    /// one step.
    pub fn update(&mut self) {
        for (_, graphic) in self.graphics.iter_mut() {
            graphic.update();
        }
    }

    /// Paints every graphic onto `display`, in the order they were added.
    /// Like [`Graphic::draw`], it neither clears nor prints.
    pub fn draw(&self, display: &mut Display) {
        for (_, graphic) in self.graphics.iter() {
            graphic.draw(display);
        }
    }

    /// Shows one complete frame: clears `display`, paints every graphic, then
    /// prints it. Pixels left over from earlier frames are removed by the
    /// clear.
    ///
    /// Returns an error if stdout cannot be written to. To inspect the frame
    /// instead of printing it, call `display.clear()` and
    /// [`Graphics::draw`], then use [`Display::render`].
    pub fn show(&self, display: &mut Display) -> io::Result<()> {
        display.clear();
        self.draw(display);
        display.print()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colour::Colour;

    struct Dot(isize, isize);
    impl Graphic for Dot {
        fn draw(&self, display: &mut Display) {
            display.set_pixel(self.0, self.1, '.', Colour::Red, Colour::Black);
        }
        fn position(&self) -> (isize, isize) {
            (self.0, self.1)
        }
        fn set_position(&mut self, x: isize, y: isize) {
            self.0 = x;
            self.1 = y;
        }
    }

    #[test]
    fn clip_keeps_only_the_part_inside_the_limit() {
        assert_eq!(clip(2, 3, 10), 2..5);
        assert_eq!(clip(-2, 5, 10), 0..3);
        assert_eq!(clip(8, 5, 10), 8..10);
        assert!(clip(10, 5, 10).is_empty());
        assert!(clip(-9, 5, 10).is_empty());
        assert!(clip(0, -5, 10).is_empty());
        assert!(clip(isize::MAX, isize::MAX, 10).is_empty());
        assert_eq!(clip(isize::MIN, isize::MAX, 10), 0..0);
    }

    #[test]
    fn default_area_is_just_the_position() {
        assert_eq!(Dot(2, 3).area(), vec![(2, 3)]);
    }

    #[test]
    fn default_contains_only_hits_the_position() {
        let d = Dot(2, 3);
        assert!(d.contains(2, 3));
        assert!(!d.contains(3, 2));
    }

    #[test]
    fn default_move_by_adds_the_delta() {
        let mut d = Dot(2, 3);
        d.move_by(-5, 1);
        assert_eq!(d.position(), (-3, 4));
    }

    #[test]
    fn move_by_saturates_instead_of_overflowing() {
        let mut d = Dot(isize::MAX, isize::MIN);
        d.move_by(1, -1);
        assert_eq!(d.position(), (isize::MAX, isize::MIN));
    }

    #[test]
    fn default_animation_hooks_do_nothing() {
        let mut d = Dot(2, 3);
        d.scale_speed(9.0, 9.0);
        d.update();
        assert_eq!(d.position(), (2, 3));
    }

    #[test]
    fn move_by_name_moves_only_that_graphic_and_reports_missing_names() {
        let mut g = Graphics::new();
        g.add("a", Dot(0, 0)).add("b", Dot(5, 5));
        assert!(g.move_by("a", 1, 2));
        assert!(!g.move_by("ghost", 1, 1));
        assert_eq!(g.get("a").unwrap().position(), (1, 2));
        assert_eq!(g.get("b").unwrap().position(), (5, 5));
    }

    #[test]
    fn get_mut_allows_mutation_and_is_none_when_missing() {
        let mut g = Graphics::new();
        g.add("a", Dot(0, 0));
        g.get_mut("a").unwrap().set_position(7, 7);
        assert_eq!(g.get("a").unwrap().position(), (7, 7));
        assert!(g.get_mut("nope").is_none());
    }

    #[test]
    fn adding_an_existing_name_replaces_it_in_place() {
        let mut g = Graphics::new();
        g.add("a", Dot(0, 0))
            .add("b", Dot(1, 1))
            .add("a", Dot(9, 9));
        assert_eq!(g.len(), 2);
        assert_eq!(g.get("a").unwrap().position(), (9, 9));

        // "a" is still drawn first, so "b" wins where they overlap
        let mut d = Display::new(1, 1);
        g.add("a", Dot(0, 0)).add("b", Dot(0, 0));
        g.draw(&mut d);
        assert_eq!(d.pixel(0, 0).unwrap().character(), '.');
    }

    #[test]
    fn remove_returns_the_graphic_and_leaves_the_others() {
        let mut g = Graphics::new();
        g.add("a", Dot(0, 0)).add("b", Dot(1, 1));
        assert_eq!(g.remove("a").unwrap().position(), (0, 0));
        assert!(g.remove("a").is_none());
        assert!(!g.contains("a") && g.contains("b"));
        assert!(!g.is_empty());
    }

    #[test]
    fn draw_paints_without_clearing() {
        let mut g = Graphics::new();
        g.add("a", Dot(0, 0));
        let mut d = Display::new(2, 1);
        d.set_pixel(1, 0, 'x', Colour::Red, Colour::Red);
        g.draw(&mut d);
        assert_eq!(d.pixel(1, 0).unwrap().character(), 'x');
        assert_eq!(d.pixel(0, 0).unwrap().character(), '.');
    }

    #[test]
    fn show_clears_stale_pixels_and_prints() {
        let mut g = Graphics::new();
        g.add("a", Dot(0, 0));
        let mut d = Display::new(2, 1);
        d.set_pixel(1, 0, 'x', Colour::Red, Colour::Red);
        g.show(&mut d).unwrap();
        assert_eq!(d.pixel(1, 0).unwrap().character(), ' ');
        assert_eq!(d.pixel(0, 0).unwrap().character(), '.');
    }
}
