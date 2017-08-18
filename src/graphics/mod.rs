pub mod shapes;
pub mod text;

use display::display::Display;

// All graphics should implement this trait
pub trait Graphic {
    fn draw(&self, display: &mut Display);
}

// Vector to hold all the graphics
pub struct Graphics {
    graphics: Vec<Box<Graphic>>,
}
impl Graphics {
    pub fn new() -> Graphics {
        Graphics {
            graphics: Vec::new(),
        }
    }

    // draw all the graphics to the display
    pub fn draw(&self, display: &mut Display) {
        display.clear();
        for graphic in self.graphics.iter() {
            graphic.draw(display);
        }
        display.print();
    }

    // add another graphic object to the graphics vector
    pub fn add(&mut self, graphic: Box<Graphic>) {
        self.graphics.push(graphic);
    }
}
