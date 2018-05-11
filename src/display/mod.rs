//! Everything required to interact with the terminal.
//!
//! The Display is used to create a window within the terminal.

pub mod colour;
pub mod display;
pub mod pixel;

#[cfg(test)]
mod tests {
    use display::colour::Colour;
    use display::display::Display;

    #[test]
    fn it_works() {
        let mut screen = Display::new(100, 25);
        screen.clear();

        screen.set_pixel(5, 5, 'o', Colour::Blue, Colour::Black);
        screen.set_pixel(4, 4, '\u{2602}', Colour::Magenta, Colour::White);

        screen.print();
    }
}
