//! The 16 ANSI terminal colours and conversions between them.

/// The Colour enum is used to set the colours in the terminal.
///
/// There are 16 colours: the 8 standard ANSI colours, and a bright version of
/// each. Use them as the foreground or background of a pixel, for example
/// with [`Display::set_pixel`](crate::display::display::Display::set_pixel).
///
/// The available colours come from the ANSI_escape_codes
/// <https://en.wikipedia.org/wiki/ANSI_escape_code#Colors>
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Colour {
    /// Standard black.
    Black,
    /// Standard red.
    Red,
    /// Standard green.
    Green,
    /// Standard yellow.
    Yellow,
    /// Standard blue.
    Blue,
    /// Standard magenta.
    Magenta,
    /// Standard cyan.
    Cyan,
    /// Standard white.
    White,
    /// Bright black (grey).
    BrightBlack,
    /// Bright red.
    BrightRed,
    /// Bright green.
    BrightGreen,
    /// Bright yellow.
    BrightYellow,
    /// Bright blue.
    BrightBlue,
    /// Bright magenta.
    BrightMagenta,
    /// Bright cyan.
    BrightCyan,
    /// Bright white.
    BrightWhite,
}

impl Colour {
    /// Get the enum value from a name.
    ///
    /// The name must match a variant exactly, for example `"BrightRed"`. An
    /// unknown name does not panic; it returns [`Colour::White`].
    ///
    /// ```
    /// use terminal_graphics::Colour;
    ///
    /// let black = Colour::of("Black");
    /// assert_eq!(Colour::Black, black);
    ///
    /// // unknown names fall back to White
    /// assert_eq!(Colour::of("Purple"), Colour::White);
    /// ```
    pub fn of(colour: &str) -> Colour {
        match colour {
            "Black" => Colour::Black,
            "Red" => Colour::Red,
            "Green" => Colour::Green,
            "Yellow" => Colour::Yellow,
            "Blue" => Colour::Blue,
            "Magenta" => Colour::Magenta,
            "Cyan" => Colour::Cyan,
            "White" => Colour::White,
            "BrightBlack" => Colour::BrightBlack,
            "BrightRed" => Colour::BrightRed,
            "BrightGreen" => Colour::BrightGreen,
            "BrightYellow" => Colour::BrightYellow,
            "BrightBlue" => Colour::BrightBlue,
            "BrightMagenta" => Colour::BrightMagenta,
            "BrightCyan" => Colour::BrightCyan,
            "BrightWhite" => Colour::BrightWhite,
            _ => Colour::White,
        }
    }

    /// Get the name of the colour, for example `"Red"`.
    ///
    /// This is the inverse of [`Colour::of`]: `Colour::of(c.to_str()) == c`
    /// for every colour.
    ///
    /// ```
    /// use terminal_graphics::Colour;
    ///
    /// assert_eq!(Colour::BrightCyan.to_str(), "BrightCyan");
    /// ```
    pub fn to_str(&self) -> &str {
        match *self {
            Colour::Black => "Black",
            Colour::Red => "Red",
            Colour::Green => "Green",
            Colour::Yellow => "Yellow",
            Colour::Blue => "Blue",
            Colour::Magenta => "Magenta",
            Colour::Cyan => "Cyan",
            Colour::White => "White",
            Colour::BrightBlack => "BrightBlack",
            Colour::BrightRed => "BrightRed",
            Colour::BrightGreen => "BrightGreen",
            Colour::BrightYellow => "BrightYellow",
            Colour::BrightBlue => "BrightBlue",
            Colour::BrightMagenta => "BrightMagenta",
            Colour::BrightCyan => "BrightCyan",
            Colour::BrightWhite => "BrightWhite",
        }
    }

    /// Returns an array of all the colours.
    /// Useful for iterating over the enum.
    ///
    /// ```
    /// use terminal_graphics::Colour;
    ///
    /// assert_eq!(Colour::variants().len(), 16);
    /// ```
    pub fn variants() -> [Colour; 16] {
        [
            Colour::Black,
            Colour::Red,
            Colour::Green,
            Colour::Yellow,
            Colour::Blue,
            Colour::Magenta,
            Colour::Cyan,
            Colour::White,
            Colour::BrightBlack,
            Colour::BrightRed,
            Colour::BrightGreen,
            Colour::BrightYellow,
            Colour::BrightBlue,
            Colour::BrightMagenta,
            Colour::BrightCyan,
            Colour::BrightWhite,
        ]
    }

    /// Get the enum value from an RGB value.
    ///
    /// Finds the closest colour using euclidean distance, comparing against
    /// the values returned by [`Colour::get_rbg`]. Ties go to the colour that
    /// comes first in [`Colour::variants`].
    ///
    /// Each of `red`, `green` and `blue` is in the range 0 to 255.
    ///
    /// ```
    /// use terminal_graphics::Colour;
    ///
    /// assert_eq!(Colour::Black, Colour::from_rgb(0,0,0));
    /// assert_eq!(Colour::BrightWhite, Colour::from_rgb(255,255,255));
    /// assert_eq!(Colour::BrightGreen, Colour::from_rgb(0, 255, 0));
    /// ```
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Colour {
        let mut closest = Colour::Black;
        let mut distance: f64 = 1000.00;

        for colour in Colour::variants().iter() {
            // find the distance between this rbg value and the colour
            let (r, g, b) = colour.get_rbg();
            let red_1 = r as f64;
            let blue_1 = b as f64;
            let green_1 = g as f64;
            let red_2 = red as f64;
            let blue_2 = blue as f64;
            let green_2 = green as f64;
            let this_distance =
                ((red_1 - red_2).powi(2) + (green_1 - green_2).powi(2) + (blue_1 - blue_2).powi(2))
                    .sqrt();
            if this_distance < distance {
                closest = *colour;
                distance = this_distance;
            }
        }

        closest
    }

    /// Gets the ANSI SGR codes for the colour, as `(foreground, background)`.
    ///
    /// The foreground code is in the range 30 to 37 or 90 to 97. The
    /// background code is always 10 more than the foreground code. These are
    /// the numbers used inside an escape sequence such as `ESC[31m`.
    ///
    /// See <https://en.wikipedia.org/wiki/ANSI_escape_code#Colors>
    ///
    /// ```
    /// use terminal_graphics::Colour;
    ///
    /// let red = Colour::Red;
    /// let (character_colour, background_colour) = red.get_codes();
    ///
    /// assert_eq!(character_colour, 31);
    /// assert_eq!(background_colour, 41);
    /// ```
    pub fn get_codes(&self) -> (u32, u32) {
        match *self {
            Colour::Black => (30, 40),
            Colour::Red => (31, 41),
            Colour::Green => (32, 42),
            Colour::Yellow => (33, 43),
            Colour::Blue => (34, 44),
            Colour::Magenta => (35, 45),
            Colour::Cyan => (36, 46),
            Colour::White => (37, 47),
            Colour::BrightBlack => (90, 100),
            Colour::BrightRed => (91, 101),
            Colour::BrightGreen => (92, 102),
            Colour::BrightYellow => (93, 103),
            Colour::BrightBlue => (94, 104),
            Colour::BrightMagenta => (95, 105),
            Colour::BrightCyan => (96, 106),
            Colour::BrightWhite => (97, 107),
        }
    }

    /// Gets the RGB triple for the colour, in the order `(red, green, blue)`.
    ///
    /// Despite the name `get_rbg` (red, blue, green), the tuple is always
    /// `(red, green, blue)`, with each channel in the range 0 to 255. These
    /// are approximate values for the colours, used by [`Colour::from_rgb`].
    ///
    /// ```
    /// use terminal_graphics::Colour;
    ///
    /// let cyan = Colour::Cyan;
    /// let (r, g, b) = cyan.get_rbg();
    ///
    /// assert_eq!(r, 0);
    /// assert_eq!(g, 128);
    /// assert_eq!(b, 128);
    /// ```
    pub fn get_rbg(&self) -> (u8, u8, u8) {
        match *self {
            Colour::Black => (0, 0, 0),
            Colour::Red => (128, 0, 0),
            Colour::Green => (0, 128, 0),
            Colour::Yellow => (128, 128, 0),
            Colour::Blue => (0, 0, 128),
            Colour::Magenta => (128, 0, 128),
            Colour::Cyan => (0, 128, 128),
            Colour::White => (128, 128, 128),
            Colour::BrightBlack => (85, 85, 85),
            Colour::BrightRed => (255, 0, 0),
            Colour::BrightGreen => (0, 255, 0),
            Colour::BrightYellow => (255, 255, 0),
            Colour::BrightBlue => (0, 0, 255),
            Colour::BrightMagenta => (255, 0, 255),
            Colour::BrightCyan => (0, 255, 255),
            Colour::BrightWhite => (255, 255, 255),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants_are_all_distinct() {
        let v = Colour::variants();
        for (i, a) in v.iter().enumerate() {
            for b in v.iter().skip(i + 1) {
                assert_ne!(a, b);
            }
        }
    }

    #[test]
    fn from_rgb_picks_the_nearest_colour() {
        assert_eq!(Colour::from_rgb(250, 10, 10), Colour::BrightRed);
        assert_eq!(Colour::from_rgb(10, 10, 120), Colour::Blue);
        assert_eq!(Colour::from_rgb(90, 90, 90), Colour::BrightBlack);
    }

    #[test]
    fn from_rgb_breaks_ties_towards_the_earlier_variant() {
        // equidistant from Black (0,0,0) and Red (128,0,0)
        assert_eq!(Colour::from_rgb(64, 0, 0), Colour::Black);
    }
}
