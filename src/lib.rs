pub mod display;
pub mod graphics;

pub use display::colour::Colour;
pub use display::display::Display;

// reexport some good graphics
pub use graphics::container::Container;
pub use graphics::shapes;
pub use graphics::text::Text;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
