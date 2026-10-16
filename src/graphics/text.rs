//! Single-line text.

use crate::colour::Colour;
use crate::display::Display;
use crate::graphics::Graphic;

/// A single line of text.
///
/// Each character occupies one display cell, laid out left to right from the
/// position. Characters are counted as `char`s, not bytes, so multi-byte
/// characters such as `é` take one cell. Characters that fall outside the
/// display are clipped.
///
/// ```
/// use terminal_graphics::{Colour, Display, Graphic, Text};
///
/// let text = Text::new(1, 0, "abc", Colour::Red, Colour::Blue);
///
/// let mut display = Display::new(5, 1);
/// text.draw(&mut display);
///
/// assert_eq!(text.position(), (1, 0));
/// assert_eq!(display.pixel(0, 0).unwrap().character(), ' ');
/// assert_eq!(display.pixel(1, 0).unwrap().character(), 'a');
/// assert_eq!(display.pixel(3, 0).unwrap().character(), 'c');
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Text {
    pos_x: isize,
    pos_y: isize,
    text: String,
    colour: Colour,
    background: Colour,
}

impl Text {
    /// Creates text starting at `(pos_x, pos_y)`.
    ///
    /// - `text`: the characters to show, one per cell. Anything convertible
    ///   to a `String` works, such as `"hi"`.
    /// - `colour`: the foreground (character) colour.
    /// - `background`: the background colour of each cell.
    pub fn new(
        pos_x: isize,
        pos_y: isize,
        text: impl Into<String>,
        colour: Colour,
        background: Colour,
    ) -> Text {
        Text {
            pos_x,
            pos_y,
            text: text.into(),
            colour,
            background,
        }
    }

    /// The text being shown.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Replaces the text. The position and colours are unchanged.
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    /// The foreground colour.
    pub fn colour(&self) -> Colour {
        self.colour
    }

    /// Sets the foreground colour.
    pub fn set_colour(&mut self, colour: Colour) {
        self.colour = colour;
    }

    /// The background colour of each cell.
    pub fn background(&self) -> Colour {
        self.background
    }

    /// Sets the background colour of each cell.
    pub fn set_background(&mut self, background: Colour) {
        self.background = background;
    }
}

impl Graphic for Text {
    /// Draws each character to the cell to its right, on the same row.
    fn draw(&self, display: &mut Display) {
        let (width, height) = display.size();
        if self.pos_y < 0 || self.pos_y >= height as isize {
            return;
        }
        // characters left of the display are skipped without being visited
        let skip = self.pos_x.saturating_neg().max(0) as usize;
        let visible = (width as isize).saturating_sub(self.pos_x.max(0)).max(0) as usize;

        for (index, character) in self.text.chars().enumerate().skip(skip).take(visible) {
            let x = self.pos_x.saturating_add(index as isize);
            display.set_pixel(x, self.pos_y, character, self.colour, self.background);
        }
    }

    fn position(&self) -> (isize, isize) {
        (self.pos_x, self.pos_y)
    }

    /// Moves the start of the text: the first character is drawn at
    /// `(pos_x, pos_y)`.
    fn set_position(&mut self, pos_x: isize, pos_y: isize) {
        self.pos_x = pos_x;
        self.pos_y = pos_y;
    }

    /// One cell for each character.
    fn area(&self) -> Vec<(isize, isize)> {
        (0..self.text.chars().count())
            .map(|i| (self.pos_x.saturating_add(i as isize), self.pos_y))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(d: &Display, y: isize, len: isize) -> String {
        (0..len)
            .map(|x| d.pixel(x, y).unwrap().character())
            .collect()
    }

    #[test]
    fn draws_characters_left_to_right_from_its_position() {
        let mut d = Display::new(6, 2);
        Text::new(1, 1, "abc", Colour::Red, Colour::Blue).draw(&mut d);
        assert_eq!(row(&d, 1, 6), " abc  ");
        let p = d.pixel(2, 1).unwrap();
        assert_eq!((p.colour(), p.background()), (Colour::Red, Colour::Blue));
    }

    #[test]
    fn counts_characters_not_bytes() {
        let mut d = Display::new(4, 1);
        Text::new(0, 0, "é☂x", Colour::Red, Colour::Blue).draw(&mut d);
        assert_eq!(row(&d, 0, 4), "é☂x ");
    }

    #[test]
    fn accepts_str_and_string() {
        assert_eq!(Text::new(0, 0, "a", Colour::Red, Colour::Red).text(), "a");
        let mut t = Text::new(0, 0, String::from("b"), Colour::Red, Colour::Red);
        t.set_text("c");
        assert_eq!(t.text(), "c");
        t.set_text(String::from("d"));
        assert_eq!(t.text(), "d");
    }

    #[test]
    fn negative_start_clips_the_leading_characters() {
        let mut d = Display::new(3, 1);
        Text::new(-2, 0, "abcde", Colour::Red, Colour::Blue).draw(&mut d);
        assert_eq!(row(&d, 0, 3), "cde");
    }

    #[test]
    fn rows_outside_the_display_draw_nothing() {
        let mut d = Display::new(3, 1);
        Text::new(0, 1, "abc", Colour::Red, Colour::Blue).draw(&mut d);
        Text::new(0, -1, "abc", Colour::Red, Colour::Blue).draw(&mut d);
        assert_eq!(d, Display::new(3, 1));
    }

    #[test]
    fn colours_can_be_changed() {
        let mut t = Text::new(0, 0, "x", Colour::Red, Colour::Blue);
        t.set_colour(Colour::Green);
        t.set_background(Colour::Black);
        assert_eq!((t.colour(), t.background()), (Colour::Green, Colour::Black));
    }

    #[test]
    fn area_has_one_cell_per_character() {
        let t = Text::new(2, 3, "é☂x", Colour::Red, Colour::Blue);
        assert_eq!(t.area(), vec![(2, 3), (3, 3), (4, 3)]);
        assert!(t.contains(4, 3) && !t.contains(5, 3));
    }

    #[test]
    fn position_can_be_set_and_moved() {
        let mut t = Text::new(0, 0, "", Colour::Red, Colour::Blue);
        t.set_position(3, 4);
        t.move_by(-1, 2);
        assert_eq!(t.position(), (2, 6));
    }
}
