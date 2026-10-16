use crate::display::colour::Colour;
use crate::display::display::Display;
use crate::graphics::Graphic;

pub struct Ball {
    pos_x: f32,
    pos_y: f32,
    speed_x: f32,
    speed_y: f32,
    colour: Colour,
}
impl Ball {
    pub fn new(pos_x: f32, pos_y: f32, speed_x: f32, speed_y: f32, colour: Colour) -> Ball {
        Ball {
            pos_x,
            pos_y,
            speed_x,
            speed_y,
            colour,
        }
    }
}
impl Graphic for Ball {
    /// Draw method for the Rectangle
    fn draw(&self, display: &mut Display) {
        // starting at (x,y), draw the rectangle onto the display
        let x = self.pos_x.round() as isize;
        let y = self.pos_y.round() as isize;
        display.set_pixel(x, y, ' ', Colour::White, self.colour);
        display.set_pixel(x + 1, y, ' ', Colour::White, self.colour);
    }

    fn get_position(&self) -> (isize, isize) {
        (self.pos_x.round() as isize, self.pos_y.round() as isize)
    }

    fn set_position(&mut self, pos_x: isize, pos_y: isize) {
        self.pos_x = pos_x as f32;
        self.pos_y = pos_y as f32;
    }

    fn get_area(&self) -> Vec<(isize, isize)> {
        let (pos_x, pos_y) = self.get_position();

        vec![(pos_x, pos_y), (pos_x + 1, pos_y)]
    }

    fn update_speeds(&mut self, x: f32, y: f32) {
        self.speed_x *= x;
        self.speed_y *= y;
    }

    fn update_position(&mut self) {
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
        assert_eq!(d.pixel_mut(1, 1).unwrap().background(), Colour::Red);
        assert_eq!(d.pixel_mut(2, 1).unwrap().background(), Colour::Red);
        assert_ne!(d.pixel_mut(3, 1).unwrap().background(), Colour::Red);
    }

    #[test]
    fn position_rounds_the_float_position() {
        assert_eq!(
            Ball::new(1.6, 2.4, 0.0, 0.0, Colour::Red).get_position(),
            (2, 2)
        );
    }

    #[test]
    fn area_covers_both_cells() {
        let b = Ball::new(3.0, 4.0, 0.0, 0.0, Colour::Red);
        assert_eq!(b.get_area(), vec![(3, 4), (4, 4)]);
        assert!(b.check_collision(4, 4));
        assert!(!b.check_collision(5, 4));
    }

    #[test]
    fn update_position_applies_speed() {
        let mut b = Ball::new(0.0, 0.0, 1.5, -0.5, Colour::Red);
        b.update_position();
        b.update_position();
        assert_eq!(b.get_position(), (3, -1));
    }

    #[test]
    fn update_speeds_scales_and_can_reverse_direction() {
        let mut b = Ball::new(0.0, 0.0, 1.0, 1.0, Colour::Red);
        b.update_speeds(-1.0, 2.0);
        b.update_position();
        assert_eq!(b.get_position(), (-1, 2));
    }

    #[test]
    fn set_position_replaces_fractional_position() {
        let mut b = Ball::new(0.7, 0.7, 0.0, 0.0, Colour::Red);
        b.set_position(5, 6);
        assert_eq!(b.get_position(), (5, 6));
    }
}
