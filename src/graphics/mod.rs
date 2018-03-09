//! The Graphic trait should be implemented by anything that
//! you want to be drawn onto the terminal.
//!
//! After implementing the Graphic trait, a Graphic can be added
//! to a Graphics structure.

pub mod shapes;
pub mod text;
pub mod container;

use display::display::Display;

use std::collections::HashMap;

// All graphics should implement this trait
pub trait Graphic {
    fn draw(&self, display: &mut Display);

    fn get_position(&self) -> (isize, isize);

    fn set_position(&mut self, pos_x: isize, pos_y: isize);

    fn get_area(&self) -> Vec<(isize, isize)>{
        vec![self.get_position()]
    }

    fn check_collision(&self, pos_x: isize, pos_y: isize) -> bool {
        for &(x, y) in self.get_area().iter() {
            if (x, y) == (pos_x, pos_y) {
                return true;
            }
        }
        false
    }

    fn move_position(&mut self, dx: isize, dy: isize) {
        let (pos_x, pos_y) = self.get_position();

        self.set_position(
            pos_x+dx,
            pos_y+dy
        );
    }

    fn update_speeds(&mut self, _x: f32, _y:f32) {}

    fn update_position(&mut self) {}
}

pub struct Graphics {
    graphics: HashMap<String, Box<Graphic>>,
}
impl Graphics {
    pub fn new() -> Graphics {
        Graphics {
            graphics: HashMap::new(),
        }
    }

    /// draw all the graphics to the display
    pub fn draw(&self, display: &mut Display) {
        display.clear();
        for (_, graphic) in self.graphics.iter() {
            graphic.draw(display);
        }
        display.print();
    }

    /// move a graphic from its name
    pub fn move_named_graphic(&mut self, name: &str, dx: isize, dy: isize) {
        self.get_mut_named(name).unwrap().move_position(dx, dy);
    }

    /// Add a graphic with a name
    pub fn add_named(&mut self, name: &str, graphic: Box<Graphic>) {
        self.graphics.insert(name.to_string(), graphic);
    }

    /// get a mutable reference to a graphic
    pub fn get_mut_named(&mut self, name: &str) -> Option<&mut Box<Graphic>> {
        self.graphics.get_mut(name)
    }

    /// get a reference to a graphic
    pub fn get_named(&self, name: &str) -> Option<&Box<Graphic>> {
        self.graphics.get(name)
    }

    pub fn remove_named(&mut self, name: &str) {
        let _ = self.graphics.remove(name);
    }
}
