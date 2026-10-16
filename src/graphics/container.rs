//! A bordered box that holds another graphic.

use crate::colour::Colour;
use crate::display::Display;
use crate::graphics::{clip, Graphic};

/// A bordered, filled box that can hold one other [`Graphic`].
///
/// # Position and size
///
/// The container covers exactly `width` by `height` cells, starting at its
/// top-left cell `(x, y)`. The outer ring of cells is the border; everything
/// is filled with the background colour. A container smaller than 2 by 2 has
/// no room for a border and is only filled.
///
/// # Content
///
/// A container owns its content, which can be any [`Graphic`]. The content's
/// position when you add it is taken as relative to the container's interior:
/// `(0, 0)` is the cell just inside the border, at the top left. The content
/// is moved to that absolute position, so afterwards the content reports its
/// absolute position. Moving the container with
/// [`set_position`](Graphic::set_position) or [`move_by`](Graphic::move_by)
/// moves the content with it.
///
/// # Drawing
///
/// The background is painted first, then the content, then the border. The
/// border therefore covers any content that reaches the edge. The border uses
/// the box-drawing characters `┌ ┐ └ ┘ ─ │`. Cells outside the display are
/// clipped.
///
/// ```
/// use terminal_graphics::{Colour, Container, Display, Graphic, Text};
///
/// // the text is at (0, 0) relative to the inside of the container
/// let container = Container::new(2, 1, 8, 4, Colour::Blue, Colour::Green)
///     .with_content(Text::new(0, 0, "hi", Colour::Red, Colour::Black));
///
/// let mut display = Display::new(12, 6);
/// container.draw(&mut display);
///
/// // the border's top-left corner is at the container's position
/// assert_eq!(display.pixel(2, 1).unwrap().character(), '┌');
/// // and the text starts just inside it
/// assert_eq!(display.pixel(3, 2).unwrap().character(), 'h');
/// // the box is exactly 8 by 4 cells
/// assert_eq!(display.pixel(9, 4).unwrap().character(), '┘');
/// ```
pub struct Container {
    pos_x: isize,
    pos_y: isize,
    width: isize,
    height: isize,
    content: Option<Box<dyn Graphic>>,
    border: Colour,
    background: Colour,
}

impl Container {
    /// Creates an empty container.
    ///
    /// - `pos_x`, `pos_y`: top-left cell of the container, in display cells.
    /// - `width`, `height`: total size in cells, border included. Zero or
    ///   negative sizes draw nothing.
    /// - `border`: colour of the border characters.
    /// - `background`: colour of the filled cells.
    pub fn new(
        pos_x: isize,
        pos_y: isize,
        width: isize,
        height: isize,
        border: Colour,
        background: Colour,
    ) -> Container {
        Container {
            pos_x,
            pos_y,
            width,
            height,
            content: None,
            border,
            background,
        }
    }

    /// Sets the content, builder style. See [`Container::set_content`].
    pub fn with_content(mut self, graphic: impl Graphic + 'static) -> Container {
        self.set_content(graphic);
        self
    }

    /// Replaces the content with `graphic`.
    ///
    /// The graphic's current position is taken as relative to the inside of
    /// the container, and it is moved to the matching absolute position.
    pub fn set_content(&mut self, mut graphic: impl Graphic + 'static) {
        let (x, y) = graphic.position();
        graphic.set_position(
            self.pos_x.saturating_add(1).saturating_add(x),
            self.pos_y.saturating_add(1).saturating_add(y),
        );
        self.content = Some(Box::new(graphic));
    }

    /// Removes and returns the content, if any.
    pub fn take_content(&mut self) -> Option<Box<dyn Graphic>> {
        self.content.take()
    }

    /// The content, if any. Its position is absolute.
    pub fn content(&self) -> Option<&dyn Graphic> {
        self.content.as_deref()
    }

    /// The content, if any, for changing it. Its position is absolute.
    pub fn content_mut(&mut self) -> Option<&mut dyn Graphic> {
        match self.content.as_mut() {
            Some(content) => Some(&mut **content),
            None => None,
        }
    }

    /// The size as `(width, height)`, border included.
    pub fn size(&self) -> (isize, isize) {
        (self.width, self.height)
    }

    /// Sets the width in cells, border included.
    pub fn set_width(&mut self, width: isize) {
        self.width = width;
    }

    /// Sets the height in cells, border included.
    pub fn set_height(&mut self, height: isize) {
        self.height = height;
    }

    /// Sets the colour of the filled cells.
    pub fn set_background(&mut self, background: Colour) {
        self.background = background;
    }

    /// Sets the colour of the border characters.
    pub fn set_border(&mut self, border: Colour) {
        self.border = border;
    }
}

impl Graphic for Container {
    /// Draws the background, then the content, then the border.
    fn draw(&self, display: &mut Display) {
        let (display_width, display_height) = display.size();
        let columns = clip(self.pos_x, self.width, display_width);
        let rows = clip(self.pos_y, self.height, display_height);
        let (border, background) = (self.border, self.background);

        for x in columns {
            for y in rows.clone() {
                if let Some(pixel) = display.pixel_mut(x, y) {
                    pixel.set_background(background);
                }
            }
        }

        if let Some(content) = &self.content {
            content.draw(display);
        }

        if self.width < 2 || self.height < 2 {
            return;
        }
        let (x, y) = (self.pos_x, self.pos_y);
        let right = x.saturating_add(self.width - 1);
        let bottom = y.saturating_add(self.height - 1);
        display.set_pixel(x, y, '┌', border, background);
        display.set_pixel(right, y, '┐', border, background);
        display.set_pixel(x, bottom, '└', border, background);
        display.set_pixel(right, bottom, '┘', border, background);
        for i in clip(x.saturating_add(1), self.width - 2, display_width) {
            display.set_pixel(i, y, '─', border, background);
            display.set_pixel(i, bottom, '─', border, background);
        }
        for j in clip(y.saturating_add(1), self.height - 2, display_height) {
            display.set_pixel(x, j, '│', border, background);
            display.set_pixel(right, j, '│', border, background);
        }
    }

    fn position(&self) -> (isize, isize) {
        (self.pos_x, self.pos_y)
    }

    /// Moves the top-left cell of the container. The content moves with it.
    fn set_position(&mut self, pos_x: isize, pos_y: isize) {
        let dx = pos_x.saturating_sub(self.pos_x);
        let dy = pos_y.saturating_sub(self.pos_y);
        self.pos_x = pos_x;
        self.pos_y = pos_y;
        if let Some(content) = &mut self.content {
            content.move_by(dx, dy);
        }
    }

    /// Every cell of the box, border included, column by column.
    fn area(&self) -> Vec<(isize, isize)> {
        let end_x = self.pos_x.saturating_add(self.width);
        let end_y = self.pos_y.saturating_add(self.height);
        (self.pos_x..end_x)
            .flat_map(|x| (self.pos_y..end_y).map(move |y| (x, y)))
            .collect()
    }

    fn contains(&self, x: isize, y: isize) -> bool {
        (self.pos_x..self.pos_x.saturating_add(self.width)).contains(&x)
            && (self.pos_y..self.pos_y.saturating_add(self.height)).contains(&y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::text::Text;

    fn ch(d: &Display, x: isize, y: isize) -> char {
        d.pixel(x, y).unwrap().character()
    }

    fn container(x: isize, y: isize, w: isize, h: isize) -> Container {
        Container::new(x, y, w, h, Colour::Blue, Colour::Green)
    }

    #[test]
    fn content_is_offset_to_just_inside_the_border() {
        let c =
            container(10, 5, 8, 8).with_content(Text::new(2, 1, "x", Colour::Red, Colour::Black));
        let mut d = Display::new(30, 30);
        c.draw(&mut d);
        // 10 + border 1 + 2, 5 + border 1 + 1
        assert_eq!(ch(&d, 13, 7), 'x');
        assert_eq!(c.content().unwrap().position(), (13, 7));
    }

    #[test]
    fn covers_exactly_width_by_height_cells() {
        let mut d = Display::new(10, 10);
        container(1, 1, 4, 3).draw(&mut d);
        assert_eq!(ch(&d, 1, 1), '┌');
        assert_eq!(ch(&d, 4, 1), '┐');
        assert_eq!(ch(&d, 1, 3), '└');
        assert_eq!(ch(&d, 4, 3), '┘');
        assert_eq!(ch(&d, 2, 1), '─');
        assert_eq!(ch(&d, 3, 3), '─');
        assert_eq!(ch(&d, 1, 2), '│');
        assert_eq!(ch(&d, 4, 2), '│');
        // nothing beyond the box
        assert_eq!(d.pixel(5, 1).unwrap().background(), Colour::White);
        assert_eq!(d.pixel(1, 4).unwrap().background(), Colour::White);
    }

    #[test]
    fn fills_the_whole_box_with_the_background() {
        let mut d = Display::new(10, 10);
        container(0, 0, 4, 4).draw(&mut d);
        assert_eq!(d.pixel(1, 1).unwrap().background(), Colour::Green);
        assert_eq!(d.pixel(3, 3).unwrap().background(), Colour::Green);
        assert_eq!(d.pixel(4, 4).unwrap().background(), Colour::White);
    }

    #[test]
    fn boxes_smaller_than_two_by_two_are_only_filled() {
        for (w, h) in [(1, 5), (5, 1), (1, 1)] {
            let mut d = Display::new(6, 6);
            container(0, 0, w, h).draw(&mut d);
            assert_eq!(d.pixel(0, 0).unwrap().background(), Colour::Green);
            assert_eq!(ch(&d, 0, 0), ' ');
        }
        for (w, h) in [(0, 3), (-2, 3), (3, 0)] {
            let mut d = Display::new(6, 6);
            container(0, 0, w, h).draw(&mut d);
            assert_eq!(d, Display::new(6, 6));
        }
    }

    #[test]
    fn two_by_two_is_four_corners() {
        let mut d = Display::new(3, 3);
        container(0, 0, 2, 2).draw(&mut d);
        assert_eq!(
            (ch(&d, 0, 0), ch(&d, 1, 0), ch(&d, 0, 1), ch(&d, 1, 1)),
            ('┌', '┐', '└', '┘')
        );
    }

    #[test]
    fn border_covers_content_that_reaches_the_edge() {
        let c = container(0, 0, 4, 3).with_content(Text::new(
            -1,
            0,
            "abcd",
            Colour::Red,
            Colour::Black,
        ));
        let mut d = Display::new(6, 6);
        c.draw(&mut d);
        assert_eq!(ch(&d, 0, 1), '│');
    }

    #[test]
    fn moving_the_container_moves_the_content() {
        let mut c =
            container(0, 0, 6, 4).with_content(Text::new(0, 0, "x", Colour::Red, Colour::Black));
        c.move_by(3, 2);
        assert_eq!(c.position(), (3, 2));
        assert_eq!(c.content().unwrap().position(), (4, 3));
        let mut d = Display::new(10, 10);
        c.draw(&mut d);
        assert_eq!(ch(&d, 4, 3), 'x');
    }

    #[test]
    fn content_can_be_changed_replaced_and_taken() {
        let mut c = container(1, 1, 6, 4);
        assert!(c.content().is_none() && c.content_mut().is_none());
        c.set_content(Text::new(0, 0, "a", Colour::Red, Colour::Black));
        c.content_mut().unwrap().move_by(1, 0);
        assert_eq!(c.content().unwrap().position(), (3, 2));
        c.set_content(Text::new(1, 1, "b", Colour::Red, Colour::Black));
        assert_eq!(c.content().unwrap().position(), (3, 3));
        assert_eq!(c.take_content().unwrap().position(), (3, 3));
        assert!(c.content().is_none());
    }

    #[test]
    fn area_and_contains_cover_the_whole_box() {
        let c = container(2, 3, 3, 2);
        assert_eq!(c.area().len(), 6);
        assert!(c.contains(2, 3) && c.contains(4, 4));
        assert!(!c.contains(5, 3) && !c.contains(2, 5) && !c.contains(1, 3));
    }

    #[test]
    fn setters_change_size_and_colours() {
        let mut c = container(0, 0, 2, 2);
        c.set_width(5);
        c.set_height(3);
        c.set_background(Colour::Red);
        c.set_border(Colour::Yellow);
        assert_eq!(c.size(), (5, 3));
        let mut d = Display::new(6, 6);
        c.draw(&mut d);
        let corner = d.pixel(0, 0).unwrap();
        assert_eq!(
            (corner.colour(), corner.background()),
            (Colour::Yellow, Colour::Red)
        );
        assert_eq!(d.pixel(2, 1).unwrap().background(), Colour::Red);
    }
}
