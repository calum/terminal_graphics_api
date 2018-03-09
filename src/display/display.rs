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

    /// set a pixel
    pub fn set_pixel(&mut self, x: isize, y: isize, character: char, colour: Colour, background: Colour) {
        // check if (x,y) is within the bounds of the display
        if
            x < 0                            ||
            x >= self.rows[0].len() as isize ||
            y < 0                            ||
            y >= self.rows.len() as isize
        {
            return
        }

        // get the pixel:
        let pixel = &mut self.rows[y as usize][x as usize];

        // set the properties for the pixel
        pixel.set_character(character);
        pixel.set_colour(colour);
        pixel.set_background(background);
    }

    /// get a mutable reference to a pixel at position (x, y)
    pub fn get_mut_pixel(&mut self, x: isize, y: isize) -> &mut Pixel {
        &mut self.rows[y as usize][x as usize]
    }

    /// Clears the display
    pub fn clear(&mut self) {
        for pixels in &mut self.rows {
            for mut pixel in pixels {
                pixel.clear();
            }
        }
    }

    /// Prints the display to the temrinal
    pub fn print(&self) {
        // move the cursor to the top left corner
        print!("\x1b[{}F", self.rows.len());

        for row in self.rows.iter() {
            // print each pixel
            for pixel in row.iter() {
                print!("{}", pixel.to_string());
            }

            // erase the rest of the line
            print!("\x1b[K");

            // start the next line
            print!("\x1b[1B");

            // move to the start of the line
            print!("\x1b[{}D", self.rows[0].len());
        }
    }
}
