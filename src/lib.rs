pub mod display;
pub mod graphics;

pub use display::colour::Colour as Colour;
pub use display::display::Display as Display;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
