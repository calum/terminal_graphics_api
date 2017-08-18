pub mod shapes;
pub mod text;

use display::display::Display;

// All graphics should implement this trait
pub trait Graphic {
    fn draw(&self, display: &mut Display);

    fn get_position(&self) -> (usize, usize);

    fn set_position(&mut self, pos_x: usize, pos_y: usize);

    fn move_position(&mut self, dx: isize, dy: isize) {
        let (mut pos_x, mut pos_y) = self.get_position();
        if dx < 0 {
            pos_x = pos_x - dx as usize;
        } else {
            pos_x = pos_x + dx as usize;
        }
        if dy < 0 {
            pos_y = pos_y - dy as usize;
        } else {
            pos_y = pos_y + dy as usize;
        }
        self.set_position(
            pos_x,
            pos_y
        );
    }
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

    // move a graphic
    pub fn move_graphic(&mut self, index: usize, dx: isize, dy: isize) {
        self.graphics[index].move_position(dx, dy);
    }

    // add another graphic object to the graphics vector
    pub fn add(&mut self, graphic: Box<Graphic>) {
        self.graphics.push(graphic);
    }
}
