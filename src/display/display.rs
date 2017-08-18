use display::colour::Colour;
use display::pixel::*;

pub struct Display {
    rows: Vec<Pixels>,
}

impl Display {
    pub fn new(size_x: u32, size_y: u32) -> Display {
        let mut rows = Vec::new();
        for _ in 0..size_y {
            rows.push(Pixel::generate_pixels(size_x));
        }
        Display {
            rows
        }
    }

    // set a pixel:
    pub fn set_pixel(&mut self, x: usize, y: usize, character: char, colour: Colour, background: Colour) {
        // get the pixel:
        let mut pixel = &mut self.rows[y][x];

        // set the properties for the pixel
        pixel.set_character(character);
        pixel.set_colour(colour);
        pixel.set_background(background);
    }

    // Clears the display
    pub fn clear(&mut self) {
        for pixels in &mut self.rows {
            for mut pixel in pixels {
                pixel.clear();
            }
        }
    }

    // Prints the display to the temrinal
    pub fn print(&self) {
        // restore the cursor position
        print!("\x1b[u");

        // move the cursor to the top left corner:
        print!("\x1b[2J");
        print!("\x1b[{}A", self.rows.len());
        print!("\x1b[100D");

        for row in self.rows.iter() {
            for pixel in row.iter() {
                print!("{}", pixel.to_string());
            }

            // erase the rest of the line
            print!("\x1b[K");

            // start the next line:
            print!("\x1b[1B");
            print!("\x1b[{}D", self.rows[0].len());
        }

        // Save the cursor position
        print!("\x1b[s");
    }
}
