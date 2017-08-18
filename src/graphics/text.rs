use graphics::Graphic;
use display::display::Display;
use display::colour::Colour;

pub struct Text {
    pos_x: usize,
    pos_y: usize,
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

    pub fn from(pos_x: usize, pos_y: usize, text: String, colour: Colour, background: Colour) -> Text {
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
            display.set_pixel(x+index, y, character, self.colour, self.background);
            index += 1;
        }
    }
}
