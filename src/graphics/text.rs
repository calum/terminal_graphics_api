use display::colour::Colour;
use display::display::Display;
use graphics::Graphic;

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

        let mut index = 0;
        for character in text.chars() {
            display.set_pixel(x + index, y, character, self.colour, self.background);
            index += 1;
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
