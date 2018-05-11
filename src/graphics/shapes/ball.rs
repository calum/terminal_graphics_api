use display::colour::Colour;
use display::display::Display;
use graphics::Graphic;

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
