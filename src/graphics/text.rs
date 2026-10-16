use crate::display::colour::Colour;
use crate::display::display::Display;
use crate::graphics::Graphic;

pub struct Text {
    pos_x: isize,
    pos_y: isize,
    text: String,
    colour: Colour,
    background: Colour,
}
impl Text {
    pub fn new() -> Text {
        Text {
            pos_x: 0,
            pos_y: 0,
            text: String::new(),
            colour: Colour::Black,
            background: Colour::White,
        }
    }

    pub fn from(
        pos_x: isize,
        pos_y: isize,
        text: String,
        colour: Colour,
        background: Colour,
    ) -> Text {
        Text {
            pos_x,
            pos_y,
            text,
            colour,
            background,
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}
impl Graphic for Text {
    fn draw(&self, display: &mut Display) {
        let x = self.pos_x;
        let y = self.pos_y;
        let text = &self.text;

        for (index, character) in text.chars().enumerate() {
            display.set_pixel(
                x + index as isize,
                y,
                character,
                self.colour,
                self.background,
            );
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

impl Default for Text {
    fn default() -> Text {
        Text::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(d: &mut Display, y: isize, len: isize) -> String {
        (0..len)
            .map(|x| d.pixel_mut(x, y).unwrap().character())
            .collect()
    }

    #[test]
    fn draws_characters_left_to_right_from_its_position() {
        let mut d = Display::new(6, 2);
        Text::from(1, 1, "abc".into(), Colour::Red, Colour::Blue).draw(&mut d);
        assert_eq!(row(&mut d, 1, 6), " abc  ");
        let p = d.pixel_mut(2, 1).unwrap();
        assert_eq!((p.colour(), p.background()), (Colour::Red, Colour::Blue));
    }

    #[test]
    fn counts_characters_not_bytes() {
        let mut d = Display::new(4, 1);
        Text::from(0, 0, "é☂x".into(), Colour::Red, Colour::Blue).draw(&mut d);
        assert_eq!(row(&mut d, 0, 4), "é☂x ");
    }

    #[test]
    fn set_text_replaces_the_text() {
        let mut t = Text::new();
        t.set_text("hi".into());
        let mut d = Display::new(2, 1);
        t.draw(&mut d);
        assert_eq!(row(&mut d, 0, 2), "hi");
    }

    #[test]
    fn negative_start_clips_the_leading_characters() {
        let mut d = Display::new(3, 1);
        Text::from(-2, 0, "abcde".into(), Colour::Red, Colour::Blue).draw(&mut d);
        assert_eq!(row(&mut d, 0, 3), "cde");
    }

    #[test]
    fn default_is_the_same_as_new() {
        assert_eq!(Text::default().get_position(), (0, 0));
    }

    #[test]
    fn position_can_be_set_and_moved() {
        let mut t = Text::new();
        t.set_position(3, 4);
        t.move_position(-1, 2);
        assert_eq!(t.get_position(), (2, 6));
    }
}
