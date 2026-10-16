//! The [`Display`]: an in-memory grid of pixels that is rendered to the terminal.

use crate::display::colour::Colour;
use crate::display::pixel::*;
use std::io::{self, Write};

/// The terminal buffer: a grid of pixels that is drawn into, then printed.
///
/// Nothing is written to the terminal until you call [`Display::print`],
/// [`Display::render`] or [`Display::write_to`]. Coordinates are `(x, y)`,
/// with `(0, 0)` at the top left.
pub struct Display {
    rows: Vec<Pixels>,
    width: usize,
}

impl Display {
    /// Creates a display `size_x` columns wide and `size_y` rows high, with
    /// every pixel set to the default (a black space on a white background).
    ///
    /// A size of 0 in either direction gives a display with no pixels, so all
    /// drawing into it is clipped. Never panics.
    ///
    /// ```
    /// use terminal_graphics::Display;
    ///
    /// let display = Display::new(10, 3);
    /// assert_eq!(display.size(), (10, 3));
    /// ```
    pub fn new(size_x: u32, size_y: u32) -> Display {
        let mut rows = Vec::new();
        for _ in 0..size_y {
            rows.push(Pixel::generate_pixels(size_x));
        }
        Display {
            rows,
            width: size_x as usize,
        }
    }

    /// Returns the display size as `(width, height)`, in cells (one cell is
    /// one character and one pixel)
    ///
    /// ```
    /// use terminal_graphics::display::display::Display;
    ///
    /// let display = Display::new(7, 3);
    /// assert_eq!(display.size(), (7, 3));
    /// ```
    pub fn size(&self) -> (u32, u32) {
        (self.width as u32, self.rows.len() as u32)
    }

    /// Sets the character and both colours of the pixel at `(x, y)`.
    ///
    /// `(0, 0)` is the top left, `x` grows to the right and `y` grows down.
    /// Coordinates outside the display are silently ignored (clipped), so
    /// this never panics.
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
            pixel.set_character(character);
            pixel.set_colour(colour);
            pixel.set_background(background);
        }
    }

    /// Returns a mutable reference to the pixel at `(x, y)`, or `None` if that
    /// position is outside the display.
    ///
    /// Use this when you want to change a pixel and handle the out-of-bounds
    /// case yourself. Unlike [`Display::get_mut_pixel`], it never panics.
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
        if x < 0 || y < 0 {
            return None;
        }
        self.rows.get_mut(y as usize)?.get_mut(x as usize)
    }

    /// Returns a mutable reference to the pixel at `(x, y)`.
    ///
    /// **Panics** if `(x, y)` is outside the display, including any negative
    /// coordinate. To avoid the panic, use [`Display::pixel_mut`], which
    /// returns `None` instead.
    pub fn get_mut_pixel(&mut self, x: isize, y: isize) -> &mut Pixel {
        &mut self.rows[y as usize][x as usize]
    }

    /// Resets every pixel to the default (a black space on a white background).
    ///
    /// The size of the display does not change.
    pub fn clear(&mut self) {
        for pixels in &mut self.rows {
            for pixel in pixels {
                pixel.clear();
            }
        }
    }

    /// Renders the display to a string of ANSI escape codes, without printing it.
    ///
    /// The string starts by moving the cursor up by the display's height, so
    /// printing the result again overwrites the previous frame in place. Each
    /// row is followed by a move back to the start of the next line. A display
    /// with no rows renders to an empty string.
    ///
    /// Use this for tests, or to send the output somewhere other than stdout.
    /// To print to the terminal, use [`Display::print`].
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
        if self.rows.is_empty() {
            return out;
        }

        // move the cursor to the top left corner
        out.push_str(&format!("\x1b[{}F", self.rows.len()));

        for row in self.rows.iter() {
            for pixel in row.iter() {
                out.push_str(&pixel.to_string());
            }

            // erase the rest of the line
            out.push_str("\x1b[K");

            // start the next line
            out.push_str("\x1b[1B");

            // move to the start of the line
            if self.width > 0 {
                out.push_str(&format!("\x1b[{}D", self.width));
            }
        }
        out
    }

    /// Writes the rendered display, as produced by [`Display::render`], to any
    /// writer: a file, a `Vec<u8>`, a network socket, and so on.
    ///
    /// Returns the first error the writer reports. This does not flush the
    /// writer.
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

    /// Prints the display to the terminal (stdout), then flushes it.
    ///
    /// Printing the same display again redraws it in place. Errors are
    /// ignored, so this never panics (for example when stdout is closed).
    /// To check for errors, use [`Display::write_to`] instead.
    pub fn print(&self) {
        let stdout = io::stdout();
        let mut lock = stdout.lock();
        // a closed stdout (e.g. a broken pipe) is not worth panicking over
        let _ = self.write_to(&mut lock).and_then(|_| lock.flush());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(display: &mut Display, x: isize, y: isize) -> Pixel {
        display.pixel_mut(x, y).unwrap().clone()
    }

    #[test]
    fn set_pixel_sets_all_three_properties() {
        let mut d = Display::new(3, 3);
        d.set_pixel(1, 2, 'o', Colour::Blue, Colour::Black);
        assert_eq!(
            at(&mut d, 1, 2),
            Pixel::new('o', Colour::Blue, Colour::Black)
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
        assert_eq!(d.render(), Display::new(3, 2).render());
    }

    #[test]
    fn pixel_mut_is_none_outside_and_some_on_the_last_cell() {
        let mut d = Display::new(3, 2);
        assert!(d.pixel_mut(2, 1).is_some());
        assert!(d.pixel_mut(3, 1).is_none());
        assert!(d.pixel_mut(2, 2).is_none());
        assert!(d.pixel_mut(-1, 0).is_none());
    }

    #[test]
    #[should_panic]
    fn get_mut_pixel_panics_out_of_bounds() {
        Display::new(1, 1).get_mut_pixel(1, 1);
    }

    #[test]
    fn clear_resets_every_pixel() {
        let mut d = Display::new(2, 2);
        d.set_pixel(0, 0, 'x', Colour::Red, Colour::Blue);
        d.set_pixel(1, 1, 'y', Colour::Red, Colour::Blue);
        d.clear();
        assert_eq!(d.render(), Display::new(2, 2).render());
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
    fn size_returns_width_and_height() {
        assert_eq!(Display::new(7, 3).size(), (7, 3));
    }

    #[test]
    fn size_of_zero_by_zero_display_is_zero() {
        assert_eq!(Display::new(0, 0).size(), (0, 0));
    }

    #[test]
    fn size_of_display_with_no_rows_keeps_width() {
        assert_eq!(Display::new(5, 0).size(), (5, 0));
    }

    #[test]
    fn size_of_display_with_no_columns_keeps_height() {
        assert_eq!(Display::new(0, 4).size(), (0, 4));
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
