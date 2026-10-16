//! A two-cell-wide ball with a floating point position and speed.

use crate::colour::Colour;
use crate::display::Display;
use crate::graphics::Graphic;

/// A moving block that is 2 cells wide and 1 cell high.
///
/// The position and speed are `f32`, so a ball can move by fractions of a
/// cell per step. Positions are rounded to the nearest cell when drawn and
/// when read with [`position`](Graphic::position). The ball is drawn at the
/// rounded position `(x, y)` and `(x + 1, y)`, and [`area`](Graphic::area)
/// returns those two cells.
///
/// - [`update`](Graphic::update) adds the current speed to the position. Call
///   it once per animation frame.
/// - [`scale_speed`](Graphic::scale_speed) multiplies each speed by the given
///   factor. Use a negative factor to reverse a direction.
///
/// ```
/// use terminal_graphics::shapes::Ball;
/// use terminal_graphics::{Colour, Graphic};
///
/// let mut ball = Ball::new(2.0, 3.0, 1.0, -1.0, Colour::Red);
/// ball.update();
/// assert_eq!(ball.position(), (3, 2));
/// assert_eq!(ball.area(), vec![(3, 2), (4, 2)]);
///
/// ball.scale_speed(-1.0, 1.0); // bounce off a wall
/// ball.update();
/// assert_eq!(ball.position(), (2, 1));
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Ball {
    pos_x: f32,
    pos_y: f32,
    speed_x: f32,
    speed_y: f32,
    colour: Colour,
}

impl Ball {
    /// Creates a ball.
    ///
    /// - `pos_x`, `pos_y`: position of the left cell, in display cells.
    ///   Fractional values are allowed and are rounded when drawn.
    /// - `speed_x`, `speed_y`: distance added to the position on each call to
    ///   `update`, in cells.
    /// - `colour`: the background colour of the two cells. The characters are
    ///   spaces, so the colour is what is seen.
    pub fn new(pos_x: f32, pos_y: f32, speed_x: f32, speed_y: f32, colour: Colour) -> Ball {
        Ball {
            pos_x,
            pos_y,
            speed_x,
            speed_y,
            colour,
        }
    }

    /// The exact, unrounded `(x, y)` position.
    pub fn exact_position(&self) -> (f32, f32) {
        (self.pos_x, self.pos_y)
    }

    /// The `(x, y)` speed, in cells per update.
    pub fn speed(&self) -> (f32, f32) {
        (self.speed_x, self.speed_y)
    }

    /// Sets the `(x, y)` speed, in cells per update.
    pub fn set_speed(&mut self, speed_x: f32, speed_y: f32) {
        self.speed_x = speed_x;
        self.speed_y = speed_y;
    }

    /// The ball's colour.
    pub fn colour(&self) -> Colour {
        self.colour
    }

    /// Sets the ball's colour.
    pub fn set_colour(&mut self, colour: Colour) {
        self.colour = colour;
    }
}

impl Graphic for Ball {
    /// Draws the two cells of the ball, at its rounded position.
    fn draw(&self, display: &mut Display) {
        for (x, y) in self.area() {
            display.set_pixel(x, y, ' ', Colour::White, self.colour);
        }
    }

    /// The rounded `(x, y)` position of the left cell. A position that is not
    /// a number counts as 0, and one beyond the integer range saturates.
    fn position(&self) -> (isize, isize) {
        (self.pos_x.round() as isize, self.pos_y.round() as isize)
    }

    /// Sets the position to the whole cell `(pos_x, pos_y)`, discarding any
    /// fractional part. The speed is unchanged.
    fn set_position(&mut self, pos_x: isize, pos_y: isize) {
        self.pos_x = pos_x as f32;
        self.pos_y = pos_y as f32;
    }

    /// The two cells covered by the ball: its rounded position and the cell to
    /// its right.
    fn area(&self) -> Vec<(isize, isize)> {
        let (pos_x, pos_y) = self.position();

        vec![(pos_x, pos_y), (pos_x.saturating_add(1), pos_y)]
    }

    /// Multiplies the horizontal and vertical speeds by the given factors. A
    /// negative factor reverses that direction.
    fn scale_speed(&mut self, factor_x: f32, factor_y: f32) {
        self.speed_x *= factor_x;
        self.speed_y *= factor_y;
    }

    /// Adds the current speed to the position. Call once per animation frame.
    fn update(&mut self) {
        self.pos_x += self.speed_x;
        self.pos_y += self.speed_y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draws_two_cells_wide() {
        let b = Ball::new(1.0, 1.0, 0.0, 0.0, Colour::Red);
        let mut d = Display::new(4, 3);
        b.draw(&mut d);
        assert_eq!(d.pixel(1, 1).unwrap().background(), Colour::Red);
        assert_eq!(d.pixel(2, 1).unwrap().background(), Colour::Red);
        assert_ne!(d.pixel(3, 1).unwrap().background(), Colour::Red);
    }

    #[test]
    fn position_rounds_the_float_position() {
        assert_eq!(
            Ball::new(1.6, 2.4, 0.0, 0.0, Colour::Red).position(),
            (2, 2)
        );
    }

    #[test]
    fn area_covers_both_cells() {
        let b = Ball::new(3.0, 4.0, 0.0, 0.0, Colour::Red);
        assert_eq!(b.area(), vec![(3, 4), (4, 4)]);
        assert!(b.contains(4, 4));
        assert!(!b.contains(5, 4));
    }

    #[test]
    fn update_applies_speed() {
        let mut b = Ball::new(0.0, 0.0, 1.5, -0.5, Colour::Red);
        b.update();
        b.update();
        assert_eq!(b.position(), (3, -1));
        assert_eq!(b.exact_position(), (3.0, -1.0));
    }

    #[test]
    fn scale_speed_scales_and_can_reverse_direction() {
        let mut b = Ball::new(0.0, 0.0, 1.0, 1.0, Colour::Red);
        b.scale_speed(-1.0, 2.0);
        assert_eq!(b.speed(), (-1.0, 2.0));
        b.update();
        assert_eq!(b.position(), (-1, 2));
    }

    #[test]
    fn set_position_replaces_fractional_position_and_keeps_speed() {
        let mut b = Ball::new(0.7, 0.7, 2.0, 3.0, Colour::Red);
        b.set_position(5, 6);
        assert_eq!(b.position(), (5, 6));
        assert_eq!(b.exact_position(), (5.0, 6.0));
        assert_eq!(b.speed(), (2.0, 3.0));
    }

    #[test]
    fn speed_and_colour_can_be_changed() {
        let mut b = Ball::new(0.0, 0.0, 1.0, 1.0, Colour::Red);
        b.set_speed(0.5, 0.0);
        b.set_colour(Colour::Blue);
        assert_eq!((b.speed(), b.colour()), ((0.5, 0.0), Colour::Blue));
    }

    #[test]
    fn extreme_positions_saturate() {
        for pos in [f32::MAX, f32::MIN, f32::INFINITY, f32::NAN] {
            let b = Ball::new(pos, pos, 0.0, 0.0, Colour::Red);
            b.draw(&mut Display::new(2, 2));
            assert_eq!(b.area().len(), 2);
        }
        assert_eq!(
            Ball::new(f32::NAN, 0.0, 0.0, 0.0, Colour::Red).position().0,
            0
        );
    }
}
