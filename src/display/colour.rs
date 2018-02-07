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
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
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
            "BrightBlack" => Colour::BrightBlack,
            "BrightRed" => Colour::BrightRed,
            "BrightGreen" => Colour::BrightGreen,
            "BrightYellow" => Colour::BrightYellow,
            "BrightBlue" => Colour::BrightBlue,
            "BrightMagenta" => Colour::BrightMagenta,
            "BrightCyan"  => Colour::BrightCyan,
            "BrightWhite" => Colour::BrightWhite,
            _   => Colour::White,
        }
    }

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
        [Colour::Black,
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
        Colour::BrightWhite]
    }

    /// Get the enum value from an RGB value.
    /// Finds the closest colour using euclidean distance.
    ///
    /// ```
    /// use terminal_graphics::Colour;
    ///
    /// assert_eq!(Colour::Black, Colour::from_rgb(0,0,0));
    /// assert_eq!(Colour::BrightWhite, Colour::from_rgb(255,255,255));
    /// assert_eq!(Colour::BrightGreen, Colour::from_rgb(85, 255, 85));
    /// ```
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Colour {

        let mut closest = Colour::Black;
        let mut distance: f64 = 255.00;

        for colour in Colour::variants().iter() {
            // find the distance between this rbg value and the colour
            let (r, g, b) = colour.get_rbg();
            println!("{}, {}, {}", r, g, b);
            let this_distance = (((r + red) as f64).powi(2) + ((g + green) as f64).powi(2) + ((b + blue) as f64).powi(2)).sqrt();
            if this_distance < distance {
                closest = Colour::of(colour.to_str());
                distance = this_distance;
            }
        }

        closest
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

    /// Gets the RBG tiple for the colour.
    ///
    /// ```
    /// use terminal_graphics::Colour;
    ///
    /// let red = Colour::Cyan;
    /// let (r, b, g) = red.get_rbg();
    ///
    /// assert_eq!(r, 0);
    /// assert_eq!(b, 170);
    /// assert_eq!(g, 170);
    /// ```
    pub fn get_rbg(&self) -> (u8, u8, u8) {
        match *self {
            Colour::Black => (0, 0, 0),
            Colour::Red => (170, 0, 0),
            Colour::Green => (0, 170, 0),
            Colour::Yellow => (170, 170, 0),
            Colour::Blue => (0, 0, 170),
            Colour::Magenta => (170, 0, 170),
            Colour::Cyan => (0, 170, 170),
            Colour::White => (170, 170, 170),
            Colour::BrightBlack => (85, 85, 85),
            Colour::BrightRed => (255, 85, 85),
            Colour::BrightGreen => (85, 255, 85),
            Colour::BrightYellow => (255, 255, 85),
            Colour::BrightBlue => (85, 85, 255),
            Colour::BrightMagenta => (255, 85, 255),
            Colour::BrightCyan => (85, 255, 255),
            Colour::BrightWhite => (255, 255, 255),
        }
    }
}
