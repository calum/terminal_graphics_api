pub mod shapes;
pub mod text;

use display::display::Display;

use std::collections::HashMap;

// All graphics should implement this trait
pub trait Graphic {
    fn draw(&self, display: &mut Display);

    fn get_position(&self) -> (isize, isize);

    fn set_position(&mut self, pos_x: isize, pos_y: isize);

    fn move_position(&mut self, dx: isize, dy: isize) {
        let (mut pos_x, mut pos_y) = self.get_position();

        self.set_position(
            pos_x+dx,
            pos_y+dy
        );
    }
}

pub struct Graphics {
    graphics: Vec<Box<Graphic>>,
    names: HashMap<String, usize>,
}
impl Graphics {
    pub fn new() -> Graphics {
        Graphics {
            graphics: Vec::new(),
            names: HashMap::new(),
        }
    }

    /// draw all the graphics to the display
    pub fn draw(&self, display: &mut Display) {
        display.clear();
        for graphic in self.graphics.iter() {
            graphic.draw(display);
        }
        display.print();
    }

    /// move a graphic
    pub fn move_graphic(&mut self, index: usize, dx: isize, dy: isize) {
        self.graphics[index].move_position(dx, dy);
    }

    /// move a graphic from its name
    pub fn move_named_graphic(&mut self, name: &char, dx: isize, dy: isize) {
        let index = self.get_index_from_name(name);

        if let Some(&i) = index {
            self.graphics[i].move_position(dx, dy);
        }
    }

    /// add another graphic object to the graphics vector
    pub fn add(&mut self, graphic: Box<Graphic>) {
        self.graphics.push(graphic);
    }

    /// Add a graphic with a name
    pub fn add_named(&mut self, name: &char, graphic: Box<Graphic>) {
        self.add(graphic);

        self.names.insert(String::from(name), self.graphics.len());
    }

    fn get_index_from_name(&self, name: &char) -> Option<&usize> {
        self.names.get(name)
    }

}
