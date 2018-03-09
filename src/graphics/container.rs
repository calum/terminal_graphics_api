use graphics::Graphic;
use display::display::Display;
use display::colour::Colour;

/// The Container graphic can be placed
/// anywhere in the Display and holds
/// a Graphic.
///
/// A Graphic within the container is drawn
/// in the coordinates relative to the
/// container.
pub struct Container<'a> {
    pos_x: isize,
    pos_y: isize,
    width: isize,
    height: isize,
    content: Option<&'a mut Graphic>,
    border: Colour,
    background: Colour,
}
impl<'a> Container<'a> {
    /// Create an empty Container
    pub fn new() -> Container<'static> {
        Container {
            pos_x: 0,
            pos_y: 0,
            width: 10,
            height: 10,
            content: None,
            border: Colour::Black,
            background: Colour::White
        }
    }

    /// Create a Container
    pub fn from(pos_x: isize, pos_y: isize, width: isize, height: isize, graphic: &'a mut Graphic, border: Colour, background: Colour) -> Container {
        let (x, y) = graphic.get_position();

        // move relative to the container
        graphic.set_position(pos_x + x, pos_y + y);

        let content = Some(graphic);
        Container {
            pos_x,
            pos_y,
            width,
            height,
            content,
            border,
            background,
        }
    }

    pub fn set_content(&mut self, graphic: &'a mut Graphic) {
        let (x, y) = graphic.get_position();

        // move relative to the container
        graphic.set_position(self.pos_x + x, self.pos_y + y);

        self.content = Some(graphic);
    }

    pub fn set_background(&mut self, background: Colour) {
        self.background = background;
    }

    pub fn set_width(&mut self, width: isize) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: isize) {
        self.height = height;
    }
}
impl<'a> Graphic for Container<'a> {
    fn draw(&self, display: &mut Display) {
        let x = self.pos_x;
        let y = self.pos_y;
        let width = self.width;
        let height = self.height;
        let background = self.background;
        let border = self.border;

        // set the background colour
        for i in x..x+width {
            for j in y..y+height {
                let pixel = display.get_mut_pixel(i, j);
                pixel.set_background(background);
            }
        }

        if let Some(ref contents) = self.content {
            contents.draw(display);
        }

        // Draw the border
        display.set_pixel(x, y+height, '└', border, background);
        display.set_pixel(x, y, '┌', border, background);
        display.set_pixel(x+width, y+height, '┘', border, background);
        display.set_pixel(x+width, y, '┐', border, background);
        for i in 1..width {
            display.set_pixel(x+i, y+height, '─', border, background);
            display.set_pixel(x+i, y, '─', border, background);
        }
        for i in 1..height {
            display.set_pixel(x, y+i, '│', border, background);
            display.set_pixel(x+width, y+i, '│', border, background);
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
