use crate::display::colour::Colour;
use std::fmt;

pub type Pixels = Vec<Pixel>;

#[derive(Clone, Debug, PartialEq)]
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

    pub fn character(&self) -> char {
        self.character
    }

    pub fn colour(&self) -> Colour {
        self.colour
    }

    pub fn background(&self) -> Colour {
        self.background
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
        *self = Pixel::default();
    }

    // Generates a Pixels object
    pub fn generate_pixels(size: u32) -> Pixels {
        vec![Pixel::default(); size as usize]
    }
}

impl Default for Pixel {
    fn default() -> Pixel {
        Pixel::new(' ', Colour::Black, Colour::White)
    }
}

/// Formats the pixel as an ANSI escape sequence followed by its character.
impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (colour, _) = self.colour.get_codes();
        let (_, background) = self.background.get_codes();

        write!(
            f,
            "\x1b[{};{}m{}\x1b[0;0m",
            colour, background, self.character
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_pixel_is_black_space_on_white() {
        let p = Pixel::default();
        assert_eq!(
            (p.character(), p.colour(), p.background()),
            (' ', Colour::Black, Colour::White)
        );
    }

    #[test]
    fn setters_change_only_their_field() {
        let mut p = Pixel::default();
        p.set_character('x');
        assert_eq!(p, Pixel::new('x', Colour::Black, Colour::White));
        p.set_colour(Colour::Red);
        assert_eq!(p, Pixel::new('x', Colour::Red, Colour::White));
        p.set_background(Colour::Blue);
        assert_eq!(p, Pixel::new('x', Colour::Red, Colour::Blue));
    }

    #[test]
    fn clear_restores_defaults() {
        let mut p = Pixel::new('x', Colour::Red, Colour::Blue);
        p.clear();
        assert_eq!(p, Pixel::default());
    }

    #[test]
    fn formats_as_foreground_and_background_ansi_codes() {
        let p = Pixel::new('x', Colour::Red, Colour::BrightBlue);
        assert_eq!(p.to_string(), "\x1b[31;104mx\x1b[0;0m");
    }

    #[test]
    fn generate_pixels_makes_requested_count_of_defaults() {
        let pixels = Pixel::generate_pixels(3);
        assert_eq!(pixels, vec![Pixel::default(); 3]);
        assert!(Pixel::generate_pixels(0).is_empty());
    }
}
