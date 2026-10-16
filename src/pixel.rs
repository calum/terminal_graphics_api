//! A single terminal cell: one character with a foreground and background colour.

use crate::colour::Colour;
use std::fmt;

/// One cell of the terminal grid: a character, a foreground colour and a
/// background colour.
///
/// A `Pixel` is one column wide and one row high in the terminal. Pixels live
/// inside a [`Display`](crate::Display); read them with
/// [`Display::pixel`](crate::Display::pixel) and change them with
/// [`Display::set_pixel`](crate::Display::set_pixel) or
/// [`Display::pixel_mut`](crate::Display::pixel_mut).
///
/// The default pixel is a black space on a white background.
///
/// Formatting a pixel with `Display` gives the ANSI escape sequence that
/// sets both colours, then the character, then a reset.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pixel {
    character: char,
    colour: Colour,
    background: Colour,
}

impl Pixel {
    /// Creates a pixel with the given character, foreground colour and
    /// background colour.
    ///
    /// ```
    /// use terminal_graphics::{Colour, Pixel};
    ///
    /// let pixel = Pixel::new('#', Colour::Red, Colour::Black);
    /// assert_eq!(pixel.character(), '#');
    /// assert_eq!(pixel.colour(), Colour::Red);
    /// assert_eq!(pixel.background(), Colour::Black);
    /// ```
    pub fn new(character: char, colour: Colour, background: Colour) -> Pixel {
        Pixel {
            character,
            colour,
            background,
        }
    }

    /// The character shown in this cell.
    pub fn character(&self) -> char {
        self.character
    }

    /// The foreground colour, which is the colour of the character.
    pub fn colour(&self) -> Colour {
        self.colour
    }

    /// The background colour, which fills the cell behind the character.
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

    /// Resets the pixel to the default: a black space on a white background.
    ///
    /// ```
    /// use terminal_graphics::{Colour, Pixel};
    ///
    /// let mut pixel = Pixel::new('#', Colour::Red, Colour::Blue);
    /// pixel.clear();
    /// assert_eq!(pixel, Pixel::default());
    /// ```
    pub fn clear(&mut self) {
        *self = Pixel::default();
    }
}

impl Default for Pixel {
    fn default() -> Pixel {
        Pixel::new(' ', Colour::Black, Colour::White)
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (colour, _) = self.colour.ansi_codes();
        let (_, background) = self.background.ansi_codes();

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
}
