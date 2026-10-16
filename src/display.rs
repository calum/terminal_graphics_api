//! The [`Display`]: an in-memory grid of pixels that is rendered to the terminal.

use crate::colour::Colour;
use crate::pixel::Pixel;
use std::fmt::Write as _;
use std::io::{self, Write};

/// The terminal buffer: a grid of [`Pixel`]s that you draw into, then print.
///
/// Nothing reaches the terminal until you call [`Display::print`] (or get the
/// output with [`Display::render`] / [`Display::write_to`]). Coordinates are
/// `(x, y)` with `(0, 0)` at the top left, `x` growing to the right and `y`
/// growing down. Drawing outside the display is silently clipped, so nothing
/// here panics on out-of-range coordinates.
///
/// ```
/// use terminal_graphics::{Colour, Display};
///
/// let mut display = Display::new(10, 3);
/// display.set_pixel(1, 1, '#', Colour::Red, Colour::Black);
///
/// assert_eq!(display.pixel(1, 1).unwrap().character(), '#');
/// assert!(display.render().contains('#'));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Display {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl Display {
    /// Creates a display `width` columns wide and `height` rows high, with
    /// every pixel set to the default (a black space on a white background).
    ///
    /// A size of 0 in either direction gives a display with no pixels, so all
    /// drawing into it is clipped.
    ///
    /// ```
    /// use terminal_graphics::Display;
    ///
    /// let display = Display::new(10, 3);
    /// assert_eq!(display.size(), (10, 3));
    /// ```
    pub fn new(width: u32, height: u32) -> Display {
        let (width, height) = (width as usize, height as usize);
        Display {
            pixels: vec![Pixel::default(); width * height],
            width,
            height,
        }
    }

    /// The display size as `(width, height)`, in cells.
    pub fn size(&self) -> (u32, u32) {
        (self.width as u32, self.height as u32)
    }

    /// The number of columns.
    pub fn width(&self) -> u32 {
        self.width as u32
    }

    /// The number of rows.
    pub fn height(&self) -> u32 {
        self.height as u32
    }

    fn index(&self, x: isize, y: isize) -> Option<usize> {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return None;
        }
        Some(y as usize * self.width + x as usize)
    }

    /// Sets the character and both colours of the pixel at `(x, y)`.
    /// Coordinates outside the display are ignored.
    ///
    /// ```
    /// use terminal_graphics::{Colour, Display};
    ///
    /// let mut display = Display::new(4, 2);
    /// display.set_pixel(3, 1, 'x', Colour::Green, Colour::Blue);
    ///
    /// // (4, 0) is outside a 4 by 2 display, so this has no effect
    /// display.set_pixel(4, 0, 'y', Colour::Red, Colour::Red);
    /// assert!(!display.render().contains('y'));
    /// ```
    pub fn set_pixel(
        &mut self,
        x: isize,
        y: isize,
        character: char,
        colour: Colour,
        background: Colour,
    ) {
        if let Some(pixel) = self.pixel_mut(x, y) {
            *pixel = Pixel::new(character, colour, background);
        }
    }

    /// The pixel at `(x, y)`, or `None` if that position is outside the
    /// display. Use it to read what has been drawn.
    ///
    /// ```
    /// use terminal_graphics::{Colour, Display};
    ///
    /// let mut display = Display::new(3, 1);
    /// display.set_pixel(2, 0, '#', Colour::Red, Colour::Black);
    ///
    /// assert_eq!(display.pixel(2, 0).unwrap().character(), '#');
    /// assert!(display.pixel(3, 0).is_none());
    /// ```
    pub fn pixel(&self, x: isize, y: isize) -> Option<&Pixel> {
        self.index(x, y).map(|i| &self.pixels[i])
    }

    /// A mutable reference to the pixel at `(x, y)`, or `None` if that
    /// position is outside the display. Use it to change just one property of
    /// a pixel, such as its background.
    ///
    /// ```
    /// use terminal_graphics::{Colour, Display};
    ///
    /// let mut display = Display::new(3, 3);
    /// if let Some(pixel) = display.pixel_mut(2, 2) {
    ///     pixel.set_character('@');
    ///     pixel.set_colour(Colour::Yellow);
    /// }
    ///
    /// assert!(display.pixel_mut(3, 0).is_none());
    /// ```
    pub fn pixel_mut(&mut self, x: isize, y: isize) -> Option<&mut Pixel> {
        self.index(x, y).map(|i| &mut self.pixels[i])
    }

    /// Resets every pixel to the default (a black space on a white
    /// background). The size of the display does not change.
    pub fn clear(&mut self) {
        self.pixels.fill(Pixel::default());
    }

    /// Renders the display to a string of ANSI escape codes, without printing it.
    ///
    /// The string starts by moving the cursor up by the display's height, so
    /// printing it again overwrites the previous frame in place. A display
    /// with no rows renders to an empty string.
    ///
    /// Use this for tests, or to send the output somewhere other than stdout.
    ///
    /// ```
    /// use terminal_graphics::{Colour, Display};
    ///
    /// let mut display = Display::new(2, 1);
    /// display.set_pixel(0, 0, 'A', Colour::Red, Colour::Black);
    ///
    /// let output = display.render();
    /// assert!(output.starts_with("\x1b[1F"));
    /// assert!(output.contains('A'));
    /// ```
    pub fn render(&self) -> String {
        let mut out = String::new();
        if self.height == 0 {
            return out;
        }

        // move the cursor to the top left corner
        let _ = write!(out, "\x1b[{}F", self.height);

        for row in 0..self.height {
            for pixel in &self.pixels[row * self.width..(row + 1) * self.width] {
                let _ = write!(out, "{pixel}");
            }

            // erase the rest of the line
            out.push_str("\x1b[K");

            // start the next line
            out.push_str("\x1b[1B");

            // move to the start of the line
            if self.width > 0 {
                let _ = write!(out, "\x1b[{}D", self.width);
            }
        }
        out
    }

    /// Writes the rendered display, as produced by [`Display::render`], to any
    /// writer: a file, a `Vec<u8>`, a socket, and so on. Does not flush.
    ///
    /// ```
    /// use terminal_graphics::Display;
    ///
    /// let display = Display::new(3, 1);
    /// let mut bytes: Vec<u8> = Vec::new();
    /// display.write_to(&mut bytes).unwrap();
    /// assert_eq!(bytes, display.render().into_bytes());
    /// ```
    pub fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(self.render().as_bytes())
    }

    /// Prints the display to stdout and flushes it. Printing the same display
    /// again redraws it in place.
    ///
    /// Returns the error if stdout cannot be written to, for example a closed
    /// pipe. Call [`Display::reserve_space`] once before the first frame.
    pub fn print(&self) -> io::Result<()> {
        let stdout = io::stdout();
        let mut lock = stdout.lock();
        self.write_to(&mut lock)?;
        lock.flush()
    }

    /// Prints one blank line per display row, making room for the display.
    ///
    /// Each frame starts by moving the cursor *up* over the previous one, so
    /// there must be at least `height` lines above the cursor to draw on.
    /// Call this once before the first [`Display::print`]; without it the
    /// first frame overwrites whatever is above the cursor. Do not call it
    /// between frames.
    ///
    /// ```no_run
    /// use terminal_graphics::Display;
    ///
    /// let display = Display::new(40, 10);
    /// display.reserve_space()?;
    /// display.print()?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn reserve_space(&self) -> io::Result<()> {
        let stdout = io::stdout();
        let mut lock = stdout.lock();
        lock.write_all("\n".repeat(self.height).as_bytes())?;
        lock.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_pixel_sets_all_three_properties() {
        let mut d = Display::new(3, 3);
        d.set_pixel(1, 2, 'o', Colour::Blue, Colour::Black);
        assert_eq!(
            d.pixel(1, 2),
            Some(&Pixel::new('o', Colour::Blue, Colour::Black))
        );
    }

    #[test]
    fn set_pixel_ignores_every_out_of_bounds_edge() {
        let mut d = Display::new(3, 2);
        for (x, y) in [
            (-1, 0),
            (0, -1),
            (3, 0),
            (0, 2),
            (isize::MIN, 0),
            (0, isize::MAX),
        ] {
            d.set_pixel(x, y, 'x', Colour::Red, Colour::Red);
        }
        assert_eq!(d, Display::new(3, 2));
    }

    #[test]
    fn pixel_and_pixel_mut_are_none_outside_and_some_on_the_last_cell() {
        let mut d = Display::new(3, 2);
        assert!(d.pixel(2, 1).is_some() && d.pixel_mut(2, 1).is_some());
        for (x, y) in [(3, 1), (2, 2), (-1, 0), (0, -1)] {
            assert!(d.pixel(x, y).is_none() && d.pixel_mut(x, y).is_none());
        }
    }

    #[test]
    fn pixel_mut_changes_the_display() {
        let mut d = Display::new(2, 2);
        d.pixel_mut(1, 0).unwrap().set_character('z');
        assert_eq!(d.pixel(1, 0).unwrap().character(), 'z');
        assert_eq!(d.pixel(0, 1).unwrap().character(), ' ');
    }

    #[test]
    fn rows_do_not_bleed_into_each_other() {
        let mut d = Display::new(3, 2);
        d.set_pixel(2, 0, 'a', Colour::Red, Colour::Red);
        assert_eq!(d.pixel(0, 1).unwrap().character(), ' ');
    }

    #[test]
    fn clear_resets_every_pixel() {
        let mut d = Display::new(2, 2);
        d.set_pixel(0, 0, 'x', Colour::Red, Colour::Blue);
        d.set_pixel(1, 1, 'y', Colour::Red, Colour::Blue);
        d.clear();
        assert_eq!(d, Display::new(2, 2));
    }

    #[test]
    fn render_moves_cursor_up_by_height_and_resets_each_row() {
        let out = Display::new(4, 2).render();
        assert!(out.starts_with("\x1b[2F"));
        assert_eq!(out.matches("\x1b[K").count(), 2);
        assert_eq!(out.matches("\x1b[1B").count(), 2);
        assert_eq!(out.matches("\x1b[4D").count(), 2);
    }

    #[test]
    fn render_of_empty_display_is_empty() {
        assert_eq!(Display::new(5, 0).render(), "");
    }

    #[test]
    fn zero_width_rows_render_without_a_cursor_move_left() {
        assert!(!Display::new(0, 2).render().contains('D'));
    }

    #[test]
    fn size_keeps_both_dimensions_even_when_empty() {
        assert_eq!(Display::new(7, 3).size(), (7, 3));
        assert_eq!(Display::new(0, 0).size(), (0, 0));
        assert_eq!(Display::new(5, 0).size(), (5, 0));
        assert_eq!(Display::new(0, 4).size(), (0, 4));
        assert_eq!(
            (Display::new(7, 3).width(), Display::new(7, 3).height()),
            (7, 3)
        );
    }

    #[test]
    fn write_to_propagates_writer_errors() {
        struct Broken;
        impl Write for Broken {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Err(io::Error::new(io::ErrorKind::BrokenPipe, "closed"))
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        assert!(Display::new(1, 1).write_to(&mut Broken).is_err());
    }
}
