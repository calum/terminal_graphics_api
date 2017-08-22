use graphics::Graphic;
use display::display::Display;
use display::colour::Colour;

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
        for pixel_x in x..x+self.width {
            for pixel_y in y..y+self.height {
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
}
