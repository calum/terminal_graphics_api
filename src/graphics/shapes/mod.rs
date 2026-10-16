use crate::display::colour::Colour;
use crate::display::display::Display;
use crate::graphics::Graphic;

pub mod ball;

pub use self::ball::Ball;

/// a rectangle
pub struct Rect {
    pos_x: isize,
    pos_y: isize,
    width: isize,
    height: isize,
    colour: Colour,
}

impl Rect {
    pub fn new(pos_x: isize, pos_y: isize, width: isize, height: isize, colour: Colour) -> Rect {
        Rect {
            pos_x,
            pos_y,
            width,
            height,
            colour,
        }
    }
}

impl Graphic for Rect {
    /// Draw method for the Rectangle
    fn draw(&self, display: &mut Display) {
        // starting at (x,y), draw the rectangle onto the display
        let x = self.pos_x;
        let y = self.pos_y;
        for pixel_x in x..x + self.width {
            for pixel_y in y..y + self.height {
                display.set_pixel(pixel_x, pixel_y, ' ', Colour::White, self.colour);
            }
        }
    }

    fn get_position(&self) -> (isize, isize) {
        (self.pos_x, self.pos_y)
    }

    fn set_position(&mut self, pos_x: isize, pos_y: isize) {
        self.pos_x = pos_x;
        self.pos_y = pos_y;
    }

    fn get_area(&self) -> Vec<(isize, isize)> {
        let mut area = Vec::new();

        for x in self.pos_x..self.pos_x + self.width {
            for y in self.pos_y..self.pos_y + self.height {
                area.push((x, y));
            }
        }

        area
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
                let bg = d.pixel_mut(x, y).unwrap().background();
                assert_eq!(bg == Colour::Red, inside, "({},{})", x, y);
            }
        }
    }

    #[test]
    fn rect_area_lists_every_cell_once() {
        let r = Rect::new(2, 3, 3, 2, Colour::Red);
        let mut area = r.get_area();
        area.sort();
        area.dedup();
        assert_eq!(area.len(), 6);
        assert!(area.contains(&(2, 3)) && area.contains(&(4, 4)));
    }

    #[test]
    fn rect_collision_matches_its_area() {
        let r = Rect::new(0, 0, 2, 2, Colour::Red);
        assert!(r.check_collision(1, 1));
        assert!(!r.check_collision(2, 2));
        assert!(!r.check_collision(-1, 0));
    }

    #[test]
    fn rect_with_zero_size_has_no_area_and_draws_nothing() {
        let r = Rect::new(0, 0, 0, 5, Colour::Red);
        assert!(r.get_area().is_empty());
        let mut d = Display::new(2, 2);
        r.draw(&mut d);
        assert_eq!(d.render(), Display::new(2, 2).render());
    }

    #[test]
    fn rect_partly_off_screen_is_clipped() {
        let mut d = Display::new(2, 2);
        Rect::new(-1, -1, 5, 5, Colour::Red).draw(&mut d);
        assert_eq!(d.pixel_mut(1, 1).unwrap().background(), Colour::Red);
    }

    #[test]
    fn rect_moves_by_delta() {
        let mut r = Rect::new(1, 1, 1, 1, Colour::Red);
        r.move_position(2, -1);
        assert_eq!(r.get_position(), (3, 0));
    }
}
