//! Ready-made shapes: [`Rect`], a filled rectangle, and [`Ball`], a
//! two-cell-wide moving block. Both implement [`Graphic`].

use crate::colour::Colour;
use crate::display::Display;
use crate::graphics::{clip, Graphic};

pub mod ball;

pub use self::ball::Ball;

/// A filled rectangle.
///
/// `(pos_x, pos_y)` is the top-left cell. `width` and `height` are the size in
/// cells, so a `Rect` of width 3 and height 2 covers 6 cells. A zero or
/// negative size covers no cells and draws nothing. Parts outside the display
/// are clipped.
///
/// ```
/// use terminal_graphics::shapes::Rect;
/// use terminal_graphics::{Colour, Display, Graphic};
///
/// let rect = Rect::new(1, 1, 2, 2, Colour::Red);
///
/// let mut display = Display::new(4, 4);
/// rect.draw(&mut display);
///
/// assert_eq!(display.pixel(1, 1).unwrap().background(), Colour::Red);
/// assert_eq!(display.pixel(2, 2).unwrap().background(), Colour::Red);
/// assert_eq!(display.pixel(3, 3).unwrap().background(), Colour::White);
/// assert_eq!(rect.area().len(), 4);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rect {
    pos_x: isize,
    pos_y: isize,
    width: isize,
    height: isize,
    colour: Colour,
}

impl Rect {
    /// Creates a rectangle.
    ///
    /// - `pos_x`, `pos_y`: top-left cell, in display cells.
    /// - `width`, `height`: size in cells.
    /// - `colour`: the fill colour, used as the background of each cell.
    pub fn new(pos_x: isize, pos_y: isize, width: isize, height: isize, colour: Colour) -> Rect {
        Rect {
            pos_x,
            pos_y,
            width,
            height,
            colour,
        }
    }

    /// The size as `(width, height)`, in cells.
    pub fn size(&self) -> (isize, isize) {
        (self.width, self.height)
    }

    /// Sets the size in cells.
    pub fn set_size(&mut self, width: isize, height: isize) {
        self.width = width;
        self.height = height;
    }

    /// The fill colour.
    pub fn colour(&self) -> Colour {
        self.colour
    }

    /// Sets the fill colour.
    pub fn set_colour(&mut self, colour: Colour) {
        self.colour = colour;
    }
}

impl Graphic for Rect {
    /// Fills every cell of the rectangle with the fill colour. The cells hold
    /// a space, so only the background is visible.
    fn draw(&self, display: &mut Display) {
        let (display_width, display_height) = display.size();
        let columns = clip(self.pos_x, self.width, display_width);
        let rows = clip(self.pos_y, self.height, display_height);
        for pixel_x in columns {
            for pixel_y in rows.clone() {
                display.set_pixel(pixel_x, pixel_y, ' ', Colour::White, self.colour);
            }
        }
    }

    fn position(&self) -> (isize, isize) {
        (self.pos_x, self.pos_y)
    }

    /// Moves the top-left cell of the rectangle.
    fn set_position(&mut self, pos_x: isize, pos_y: isize) {
        self.pos_x = pos_x;
        self.pos_y = pos_y;
    }

    /// Every cell covered by the rectangle (`width * height` cells in total),
    /// ordered column by column.
    fn area(&self) -> Vec<(isize, isize)> {
        let end_x = self.pos_x.saturating_add(self.width);
        let end_y = self.pos_y.saturating_add(self.height);

        (self.pos_x..end_x)
            .flat_map(|x| (self.pos_y..end_y).map(move |y| (x, y)))
            .collect()
    }

    /// Whether `(x, y)` is inside the rectangle. Answers directly from the
    /// bounds, without listing every cell.
    fn contains(&self, x: isize, y: isize) -> bool {
        (self.pos_x..self.pos_x.saturating_add(self.width)).contains(&x)
            && (self.pos_y..self.pos_y.saturating_add(self.height)).contains(&y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_fills_exactly_its_area() {
        let r = Rect::new(1, 1, 2, 2, Colour::Red);
        let mut d = Display::new(4, 4);
        r.draw(&mut d);
        for y in 0..4 {
            for x in 0..4 {
                let inside = (1..3).contains(&x) && (1..3).contains(&y);
                let bg = d.pixel(x, y).unwrap().background();
                assert_eq!(bg == Colour::Red, inside, "({},{})", x, y);
            }
        }
    }

    #[test]
    fn rect_area_lists_every_cell_once() {
        let r = Rect::new(2, 3, 3, 2, Colour::Red);
        let mut area = r.area();
        area.sort();
        area.dedup();
        assert_eq!(area.len(), 6);
        assert!(area.contains(&(2, 3)) && area.contains(&(4, 4)));
    }

    #[test]
    fn rect_contains_matches_its_area() {
        let r = Rect::new(-1, 0, 3, 2, Colour::Red);
        for x in -3..5 {
            for y in -3..5 {
                assert_eq!(r.contains(x, y), r.area().contains(&(x, y)), "({x},{y})");
            }
        }
    }

    #[test]
    fn rect_with_zero_or_negative_size_has_no_area_and_draws_nothing() {
        for (w, h) in [(0, 5), (5, 0), (-3, 5)] {
            let r = Rect::new(0, 0, w, h, Colour::Red);
            assert!(r.area().is_empty() && !r.contains(0, 0));
            let mut d = Display::new(2, 2);
            r.draw(&mut d);
            assert_eq!(d, Display::new(2, 2));
        }
    }

    #[test]
    fn rect_partly_off_screen_is_clipped() {
        let mut d = Display::new(2, 2);
        Rect::new(-1, -1, 5, 5, Colour::Red).draw(&mut d);
        assert_eq!(d.pixel(1, 1).unwrap().background(), Colour::Red);
    }

    #[test]
    fn rect_moves_by_delta() {
        let mut r = Rect::new(1, 1, 1, 1, Colour::Red);
        r.move_by(2, -1);
        assert_eq!(r.position(), (3, 0));
    }

    #[test]
    fn rect_size_and_colour_can_be_changed() {
        let mut r = Rect::new(0, 0, 1, 1, Colour::Red);
        r.set_size(3, 2);
        r.set_colour(Colour::Blue);
        assert_eq!((r.size(), r.colour()), ((3, 2), Colour::Blue));
        assert_eq!(r.area().len(), 6);
    }
}
