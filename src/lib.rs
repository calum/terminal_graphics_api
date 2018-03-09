pub mod display;
pub mod graphics;

pub use display::colour::Colour as Colour;
pub use display::display::Display as Display;

// reexport some good graphics
pub use graphics::container::Container;
pub use graphics::text::Text;
pub use graphics::shapes;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
