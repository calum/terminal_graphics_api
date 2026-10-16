use crate::display::colour::Colour;
use crate::display::pixel::*;
use std::io::{self, Write};

pub struct Display {
    rows: Vec<Pixels>,
    width: usize,
}

impl Display {
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

    /// set a pixel; coordinates outside the display are ignored
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

    /// get a mutable reference to a pixel at position (x, y),
    /// or `None` if it is outside the display
    pub fn pixel_mut(&mut self, x: isize, y: isize) -> Option<&mut Pixel> {
        if x < 0 || y < 0 {
            return None;
        }
        self.rows.get_mut(y as usize)?.get_mut(x as usize)
    }

    /// get a mutable reference to a pixel at position (x, y)
    ///
    /// Panics if (x, y) is outside the display; prefer [`Display::pixel_mut`].
    pub fn get_mut_pixel(&mut self, x: isize, y: isize) -> &mut Pixel {
        &mut self.rows[y as usize][x as usize]
    }

    /// Clears the display
    pub fn clear(&mut self) {
        for pixels in &mut self.rows {
            for pixel in pixels {
                pixel.clear();
            }
        }
    }

    /// Renders the display to a string of ANSI escape codes
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

    /// Writes the rendered display to any writer
    pub fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(self.render().as_bytes())
    }

    /// Prints the display to the terminal
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
