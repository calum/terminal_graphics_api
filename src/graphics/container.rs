use crate::display::colour::Colour;
use crate::display::display::Display;
use crate::graphics::Graphic;

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
    content: Option<&'a mut dyn Graphic>,
    border: Colour,
    background: Colour,
}
impl<'a> Container<'a> {
    /// Create an empty Container
    pub fn new() -> Container<'a> {
        Container {
            pos_x: 0,
            pos_y: 0,
            width: 10,
            height: 10,
            content: None,
            border: Colour::Black,
            background: Colour::White,
        }
    }

    /// Create a Container
    pub fn from(
        pos_x: isize,
        pos_y: isize,
        width: isize,
        height: isize,
        graphic: &'a mut dyn Graphic,
        border: Colour,
        background: Colour,
    ) -> Container<'a> {
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

    pub fn set_content(&mut self, graphic: &'a mut dyn Graphic) {
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
        for i in x..x + width {
            for j in y..y + height {
                if let Some(pixel) = display.pixel_mut(i, j) {
                    pixel.set_background(background);
                }
            }
        }

        if let Some(ref contents) = self.content {
            contents.draw(display);
        }

        // Draw the border
        display.set_pixel(x, y + height, '└', border, background);
        display.set_pixel(x, y, '┌', border, background);
        display.set_pixel(x + width, y + height, '┘', border, background);
        display.set_pixel(x + width, y, '┐', border, background);
        for i in 1..width {
            display.set_pixel(x + i, y + height, '─', border, background);
            display.set_pixel(x + i, y, '─', border, background);
        }
        for i in 1..height {
            display.set_pixel(x, y + i, '│', border, background);
            display.set_pixel(x + width, y + i, '│', border, background);
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

impl<'a> Default for Container<'a> {
    fn default() -> Container<'a> {
        Container::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::text::Text;

    fn px(d: &mut Display, x: isize, y: isize) -> (char, Colour) {
        let p = d.pixel_mut(x, y).unwrap();
        (p.character(), p.background())
    }

    #[test]
    fn new_container_is_at_origin() {
        assert_eq!(Container::new().get_position(), (0, 0));
    }

    #[test]
    fn content_is_offset_by_the_container_position() {
        let mut text = Text::from(2, 1, "x".into(), Colour::Red, Colour::Black);
        let c = Container::from(10, 5, 8, 8, &mut text, Colour::Blue, Colour::Green);
        let mut d = Display::new(30, 30);
        c.draw(&mut d);
        assert_eq!(px(&mut d, 12, 6).0, 'x');
    }

    #[test]
    fn draws_corners_and_edges_of_the_border() {
        let mut d = Display::new(10, 10);
        let mut c = Container::new();
        c.set_width(4);
        c.set_height(3);
        c.set_position(1, 1);
        c.draw(&mut d);
        assert_eq!(px(&mut d, 1, 1).0, '┌');
        assert_eq!(px(&mut d, 5, 1).0, '┐');
        assert_eq!(px(&mut d, 1, 4).0, '└');
        assert_eq!(px(&mut d, 5, 4).0, '┘');
        assert_eq!(px(&mut d, 3, 1).0, '─');
        assert_eq!(px(&mut d, 3, 4).0, '─');
        assert_eq!(px(&mut d, 1, 2).0, '│');
        assert_eq!(px(&mut d, 5, 3).0, '│');
    }

    #[test]
    fn fills_the_interior_with_the_background() {
        let mut d = Display::new(10, 10);
        let mut c = Container::new();
        c.set_background(Colour::Green);
        c.set_width(3);
        c.set_height(3);
        c.draw(&mut d);
        assert_eq!(px(&mut d, 1, 1).1, Colour::Green);
        assert_eq!(px(&mut d, 5, 5).1, Colour::White);
    }

    #[test]
    fn set_content_offsets_the_new_content() {
        let mut text = Text::from(1, 1, "z".into(), Colour::Red, Colour::Black);
        let mut c = Container::new();
        c.set_position(4, 4);
        c.set_content(&mut text);
        let mut d = Display::new(10, 10);
        c.draw(&mut d);
        assert_eq!(px(&mut d, 5, 5).0, 'z');
    }
}
