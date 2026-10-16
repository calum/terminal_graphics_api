//! The Graphic trait should be implemented by anything that
//! you want to be drawn onto the terminal.
//!
//! After implementing the Graphic trait, a Graphic can be added
//! to a Graphics structure.

pub mod container;
pub mod shapes;
pub mod text;

use crate::display::display::Display;

// All graphics should implement this trait
pub trait Graphic {
    fn draw(&self, display: &mut Display);

    fn get_position(&self) -> (isize, isize);

    fn set_position(&mut self, pos_x: isize, pos_y: isize);

    fn get_area(&self) -> Vec<(isize, isize)> {
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

        self.set_position(pos_x + dx, pos_y + dy);
    }

    fn update_speeds(&mut self, _x: f32, _y: f32) {}

    fn update_position(&mut self) {}
}

/// A collection of named graphics, drawn in the order they were added
/// (later graphics are drawn on top of earlier ones).
#[derive(Default)]
pub struct Graphics {
    graphics: Vec<(String, Box<dyn Graphic>)>,
}
impl Graphics {
    pub fn new() -> Graphics {
        Graphics::default()
    }

    /// clear the display, draw all the graphics to it and print it
    pub fn draw(&self, display: &mut Display) {
        self.render_to(display);
        display.print();
    }

    /// clear the display and draw all the graphics to it without printing
    pub fn render_to(&self, display: &mut Display) {
        display.clear();
        for (_, graphic) in self.graphics.iter() {
            graphic.draw(display);
        }
    }

    /// move a graphic from its name; does nothing if there is no such graphic
    pub fn move_named_graphic(&mut self, name: &str, dx: isize, dy: isize) {
        if let Some(graphic) = self.get_mut_named(name) {
            graphic.move_position(dx, dy);
        }
    }

    /// Add a graphic with a name, replacing (in place) any graphic with the same name
    pub fn add_named(&mut self, name: &str, graphic: Box<dyn Graphic>) {
        match self.graphics.iter_mut().find(|(n, _)| n == name) {
            Some(entry) => entry.1 = graphic,
            None => self.graphics.push((name.to_string(), graphic)),
        }
    }

    /// get a mutable reference to a graphic
    pub fn get_mut_named(&mut self, name: &str) -> Option<&mut Box<dyn Graphic>> {
        self.graphics
            .iter_mut()
            .find(|(n, _)| n == name)
            .map(|(_, g)| g)
    }

    /// get a reference to a graphic
    #[allow(clippy::borrowed_box)]
    pub fn get_named(&self, name: &str) -> Option<&Box<dyn Graphic>> {
        self.graphics
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, g)| g)
    }

    pub fn remove_named(&mut self, name: &str) {
        self.graphics.retain(|(n, _)| n != name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display::colour::Colour;

    struct Dot(isize, isize);
    impl Graphic for Dot {
        fn draw(&self, display: &mut Display) {
            display.set_pixel(self.0, self.1, '.', Colour::Red, Colour::Black);
        }
        fn get_position(&self) -> (isize, isize) {
            (self.0, self.1)
        }
        fn set_position(&mut self, x: isize, y: isize) {
            self.0 = x;
            self.1 = y;
        }
    }

    #[test]
    fn default_area_is_just_the_position() {
        assert_eq!(Dot(2, 3).get_area(), vec![(2, 3)]);
    }

    #[test]
    fn default_collision_only_hits_the_position() {
        let d = Dot(2, 3);
        assert!(d.check_collision(2, 3));
        assert!(!d.check_collision(3, 2));
    }

    #[test]
    fn default_move_position_adds_the_delta() {
        let mut d = Dot(2, 3);
        d.move_position(-5, 1);
        assert_eq!(d.get_position(), (-3, 4));
    }

    #[test]
    fn default_update_hooks_do_nothing() {
        let mut d = Dot(2, 3);
        d.update_speeds(9.0, 9.0);
        d.update_position();
        assert_eq!(d.get_position(), (2, 3));
    }

    #[test]
    fn move_named_graphic_moves_only_that_graphic() {
        let mut g = Graphics::new();
        g.add_named("a", Box::new(Dot(0, 0)));
        g.add_named("b", Box::new(Dot(5, 5)));
        g.move_named_graphic("a", 1, 2);
        assert_eq!(g.get_named("a").unwrap().get_position(), (1, 2));
        assert_eq!(g.get_named("b").unwrap().get_position(), (5, 5));
    }

    #[test]
    fn get_mut_named_allows_mutation_and_is_none_when_missing() {
        let mut g = Graphics::new();
        g.add_named("a", Box::new(Dot(0, 0)));
        g.get_mut_named("a").unwrap().set_position(7, 7);
        assert_eq!(g.get_named("a").unwrap().get_position(), (7, 7));
        assert!(g.get_mut_named("nope").is_none());
    }

    #[test]
    fn render_to_clears_stale_pixels_first() {
        let mut g = Graphics::new();
        g.add_named("a", Box::new(Dot(0, 0)));
        let mut d = Display::new(2, 1);
        d.set_pixel(1, 0, 'x', Colour::Red, Colour::Red);
        g.render_to(&mut d);
        assert_eq!(d.pixel_mut(1, 0).unwrap().character(), ' ');
        assert_eq!(d.pixel_mut(0, 0).unwrap().character(), '.');
    }

    #[test]
    fn remove_named_removes_only_that_graphic() {
        let mut g = Graphics::new();
        g.add_named("a", Box::new(Dot(0, 0)));
        g.add_named("b", Box::new(Dot(1, 1)));
        g.remove_named("a");
        assert!(g.get_named("a").is_none());
        assert!(g.get_named("b").is_some());
    }

    #[test]
    fn draw_renders_and_prints_without_panicking() {
        let g = Graphics::new();
        g.draw(&mut Display::new(1, 1));
    }
}
