use display::colour::Colour;

pub type Pixels = Vec<Pixel>;

pub struct Pixel {
    character: char,
    colour: Colour,
    background: Colour,
}

impl Pixel {
    // Generates a new pixel
    pub fn new(character: char, colour: Colour, background: Colour) -> Pixel {
        Pixel {
            character,
            colour,
            background,
        }
    }

    // set the character
    pub fn set_character(&mut self, character: char) {
        self.character = character;
    }

    // set the colour
    pub fn set_colour(&mut self, colour: Colour) {
        self.colour = colour;
    }

    // set the background
    pub fn set_background(&mut self, background: Colour) {
        self.background = background;
    }

    // clear all the values to defaults
    pub fn clear(&mut self) {
        *self = Pixel {
            character: ' ',
            colour: Colour::Black,
            background: Colour::White,
        };
    }

    // formats the pixel and returns a string slice
    pub fn to_string(&self) -> String {
        let (colour, _) = self.colour.get_codes();
        let (_, background) = self.background.get_codes();

        let formated_pixel = format!("\x1b[{};{}m{}\x1b[0;0m", colour, background, self.character);
        formated_pixel
    }

    // Generates a Pixels object
    pub fn generate_pixels(size: u32) -> Pixels {
        let mut pixels = Pixels::new();
        for _ in 0..size {
            pixels.push(Pixel::new(' ', Colour::Black, Colour::White));
        }
        pixels
    }
}
