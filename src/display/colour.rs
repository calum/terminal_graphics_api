#[derive(Copy, Clone)]
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
