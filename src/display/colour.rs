/// The Colour enum is used to set the colours in the Terminal
///
/// The available colours come from the ANSI_escape_codes
/// <https://en.wikipedia.org/wiki/ANSI_escape_code#Colors>

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Colour {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Colour {
    /// Get the enum value from a string
    ///
    /// ```
    /// use terminal_graphics::Colour;
    ///
    /// let black = Colour::of("Black");
    /// assert_eq!(Colour::Black, black);
    /// ```
    pub fn of(colour: &str) -> Colour {
        match colour {
            "Black" => Colour::Black,
            "Red" => Colour::Red,
            "Green" => Colour::Green,
            "Yellow" => Colour::Yellow,
            "Blue" => Colour::Blue,
            "Magenta" => Colour::Magenta,
            "Cyan"  => Colour::Cyan,
            "White" => Colour::White,
            _   => Colour::White,
        }
    }

    /// Gets the values for the colour:
    /// from <https://en.wikipedia.org/wiki/ANSI_escape_code#Colors>
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
        }
    }
}
