//! The 16 ANSI terminal colours and conversions between them.

use std::fmt;
use std::str::FromStr;

/// One of the 16 ANSI terminal colours.
///
/// There are the 8 standard colours and a bright version of each. Use them as
/// the foreground or background of a [`Pixel`](crate::Pixel), for example with
/// [`Display::set_pixel`](crate::Display::set_pixel).
///
/// Colours print as their name and parse back from it:
///
/// ```
/// use terminal_graphics::Colour;
///
/// assert_eq!(Colour::BrightRed.to_string(), "BrightRed");
/// assert_eq!("BrightRed".parse(), Ok(Colour::BrightRed));
/// assert!("Purple".parse::<Colour>().is_err());
/// ```
///
/// The colours come from the ANSI escape codes:
/// <https://en.wikipedia.org/wiki/ANSI_escape_code#Colors>
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
    /// Standard white (light grey on most terminals).
    White,
    /// Bright black (dark grey).
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
    /// All 16 colours, in ANSI order: the 8 standard colours, then the 8
    /// bright ones. Useful for iterating over the enum.
    ///
    /// ```
    /// use terminal_graphics::Colour;
    ///
    /// assert_eq!(Colour::ALL.len(), 16);
    /// assert_eq!(Colour::ALL[0], Colour::Black);
    /// ```
    pub const ALL: [Colour; 16] = [
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
    ];

    /// The name of the colour, for example `"Red"`. This is also what
    /// `Display` prints and what [`FromStr`] accepts.
    ///
    /// ```
    /// use terminal_graphics::Colour;
    ///
    /// assert_eq!(Colour::BrightCyan.name(), "BrightCyan");
    /// ```
    pub fn name(self) -> &'static str {
        match self {
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

    /// The colour closest to an RGB value.
    ///
    /// Distance is euclidean, measured against the values returned by
    /// [`Colour::rgb`]. Ties go to the colour that comes first in
    /// [`Colour::ALL`].
    ///
    /// ```
    /// use terminal_graphics::Colour;
    ///
    /// assert_eq!(Colour::from_rgb(0, 0, 0), Colour::Black);
    /// assert_eq!(Colour::from_rgb(255, 255, 255), Colour::BrightWhite);
    /// assert_eq!(Colour::from_rgb(0, 255, 0), Colour::BrightGreen);
    /// ```
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Colour {
        let (red, green, blue) = (red as i32, green as i32, blue as i32);
        let squared_distance = |colour: &Colour| {
            let (r, g, b) = colour.rgb();
            (r as i32 - red).pow(2) + (g as i32 - green).pow(2) + (b as i32 - blue).pow(2)
        };

        // `min_by_key` returns the first minimum, so ties go to the earlier variant
        Colour::ALL
            .into_iter()
            .min_by_key(squared_distance)
            .unwrap_or(Colour::Black)
    }

    /// The ANSI SGR codes for the colour, as `(foreground, background)`.
    ///
    /// The foreground code is 30 to 37 or 90 to 97, and the background code
    /// is always 10 more. These are the numbers used inside an escape
    /// sequence such as `ESC[31m`.
    ///
    /// ```
    /// use terminal_graphics::Colour;
    ///
    /// assert_eq!(Colour::Red.ansi_codes(), (31, 41));
    /// assert_eq!(Colour::BrightRed.ansi_codes(), (91, 101));
    /// ```
    pub fn ansi_codes(self) -> (u8, u8) {
        let foreground = match self {
            Colour::Black => 30,
            Colour::Red => 31,
            Colour::Green => 32,
            Colour::Yellow => 33,
            Colour::Blue => 34,
            Colour::Magenta => 35,
            Colour::Cyan => 36,
            Colour::White => 37,
            Colour::BrightBlack => 90,
            Colour::BrightRed => 91,
            Colour::BrightGreen => 92,
            Colour::BrightYellow => 93,
            Colour::BrightBlue => 94,
            Colour::BrightMagenta => 95,
            Colour::BrightCyan => 96,
            Colour::BrightWhite => 97,
        };
        (foreground, foreground + 10)
    }

    /// Approximate `(red, green, blue)` values for the colour, each 0 to 255.
    /// Terminals pick their own shades, so these are only a guide; they are
    /// what [`Colour::from_rgb`] matches against.
    ///
    /// ```
    /// use terminal_graphics::Colour;
    ///
    /// assert_eq!(Colour::Cyan.rgb(), (0, 128, 128));
    /// ```
    pub fn rgb(self) -> (u8, u8, u8) {
        match self {
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

/// Formats the colour as its name, for example `BrightRed`.
impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// The error returned when parsing a colour name that matches no variant.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseColourError(String);

impl fmt::Display for ParseColourError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown colour name {:?}", self.0)
    }
}

impl std::error::Error for ParseColourError {}

/// Parses a colour from its exact, case-sensitive name, such as `"BrightRed"`.
impl FromStr for Colour {
    type Err = ParseColourError;

    fn from_str(name: &str) -> Result<Colour, ParseColourError> {
        Colour::ALL
            .into_iter()
            .find(|colour| colour.name() == name)
            .ok_or_else(|| ParseColourError(name.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn all_colours_are_distinct() {
        let unique: HashSet<_> = Colour::ALL.into_iter().collect();
        assert_eq!(unique.len(), 16);
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

    #[test]
    fn every_colour_maps_to_itself_by_rgb() {
        for colour in Colour::ALL {
            let (r, g, b) = colour.rgb();
            assert_eq!(Colour::from_rgb(r, g, b), colour);
        }
    }

    #[test]
    fn bright_black_is_grey_not_dark_red() {
        assert_eq!(Colour::BrightBlack.rgb(), (85, 85, 85));
    }

    #[test]
    fn ansi_codes_are_unique_and_background_is_foreground_plus_ten() {
        let mut seen = HashSet::new();
        for colour in Colour::ALL {
            let (fg, bg) = colour.ansi_codes();
            assert_eq!(bg, fg + 10);
            assert!(seen.insert(fg), "duplicate code {fg}");
        }
    }

    #[test]
    fn display_prints_the_name_and_parsing_round_trips() {
        for colour in Colour::ALL {
            assert_eq!(colour.to_string(), colour.name());
            assert_eq!(colour.to_string().parse(), Ok(colour));
        }
    }

    #[test]
    fn parsing_is_exact_and_the_error_names_the_input() {
        assert!("red".parse::<Colour>().is_err());
        let err = "nonsense".parse::<Colour>().unwrap_err();
        assert!(err.to_string().contains("nonsense"));
        let _: Box<dyn std::error::Error> = Box::new(err);
    }
}
