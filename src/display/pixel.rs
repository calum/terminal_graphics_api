//! A single terminal cell: one character with a foreground and background colour.

use crate::display::colour::Colour;
use std::fmt;

/// A list of pixels, used for one row of a [`Display`](crate::display::display::Display).
pub type Pixels = Vec<Pixel>;

/// One cell of the terminal grid: a character, a foreground colour and a
/// background colour.
///
/// A `Pixel` is a single character cell, so one pixel is one column wide and
/// one row high in the terminal. Pixels are normally reached through
/// [`Display`](crate::display::display::Display), not created directly.
#[derive(Clone, Debug, PartialEq)]
pub struct Pixel {
    character: char,
    colour: Colour,
    background: Colour,
}

impl Pixel {
    /// Creates a new pixel with the given character and colours.
    ///
    /// ```
    /// use terminal_graphics::{Colour, display::pixel::Pixel};
    ///
    /// let pixel = Pixel::new('#', Colour::Red, Colour::Black);
    /// assert_eq!(pixel.character(), '#');
    /// ```
    pub fn new(character: char, colour: Colour, background: Colour) -> Pixel {
        Pixel {
            character,
            colour,
            background,
        }
    }

    /// Returns the character shown in this cell.
    pub fn character(&self) -> char {
        self.character
    }

    /// Returns the foreground colour, which is the colour of the character.
    pub fn colour(&self) -> Colour {
        self.colour
    }

    /// Returns the background colour, which fills the cell behind the character.
    pub fn background(&self) -> Colour {
        self.background
    }

    /// Sets the character shown in this cell.
    pub fn set_character(&mut self, character: char) {
        self.character = character;
    }

    /// Sets the foreground colour, which is the colour of the character.
    pub fn set_colour(&mut self, colour: Colour) {
        self.colour = colour;
    }

    /// Sets the background colour, which fills the cell behind the character.
    pub fn set_background(&mut self, background: Colour) {
        self.background = background;
    }

    /// Resets the pixel to its default: a black space on a white background.
    ///
    /// ```
    /// use terminal_graphics::{Colour, display::pixel::Pixel};
    ///
    /// let mut pixel = Pixel::new('#', Colour::Red, Colour::Blue);
    /// pixel.clear();
    /// assert_eq!(pixel, Pixel::default());
    /// ```
    pub fn clear(&mut self) {
        *self = Pixel::default();
    }

    /// Creates a row of `size` default pixels.
    ///
    /// A size of 0 returns an empty list. This never panics.
    pub fn generate_pixels(size: u32) -> Pixels {
        vec![Pixel::default(); size as usize]
    }
}

/// The default pixel is a black space on a white background.
impl Default for Pixel {
    fn default() -> Pixel {
        Pixel::new(' ', Colour::Black, Colour::White)
    }
}

/// Formats the pixel as one cell of ANSI output: an escape sequence that sets
/// the foreground and background colours, the character, then a reset.
///
/// This is what [`Display::render`](crate::display::display::Display::render)
/// writes for each cell.
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
