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

    /// Get the enum value from an RGB value.
    /// The implementation needs improving.
    ///
    /// ```
    /// use terminal_graphics::Colour;
    ///
    /// assert_eq!(Colour::Black, Colour::from_rgb(0,0,0));
    /// assert_eq!(Colour::White, Colour::from_rgb(255,255,255));
    /// assert_eq!(Colour::Magenta, Colour::from_rgb(255,0,255));
    /// ```
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Colour {
        // round the values:
        let mut red_rounded = 0;
        let mut green_rounded = 0;
        let mut blue_rounded = 0;
        if red > 127 {
            red_rounded = 255;
        }
        if green > 127 {
            green_rounded = 255;
        }
        if blue > 127 {
            blue_rounded = 255;
        }

        match (red_rounded, green_rounded, blue_rounded) {
            (0, 0, 0) => Colour::Black,
            (255, 255, 255) => Colour::White,
            (_, 0, 0) => Colour::Red,
            (0, _, 0) => Colour::Green,
            (0, 0, _) => Colour::Blue,
            (_, _, 0) => Colour::Green,
            (_, 0, _) => Colour::Magenta,
            (0, _, _) => Colour::Cyan,
            (_, _, _) => Colour::White,
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
