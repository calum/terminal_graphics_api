extern crate terminal_graphics;

use terminal_graphics::{Display, Colour, Container, Text};
use terminal_graphics::graphics::shapes::Rect;
use terminal_graphics::graphics::Graphic;

fn main() {
    let mut display = Display::new(50, 50);

    let mut text = Text::from(2, 1, String::from("Title!"), Colour::Blue, Colour::Black);

    let mut container = Container::from(10, 10, 10, 10, &mut text, Colour::BrightRed, Colour::BrightBlack);

    container.draw(&mut display);

    display.print();
}
