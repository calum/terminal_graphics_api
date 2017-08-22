//! # An example application
//!
//! Draws a rectangle onto the terminal and allows the user to move
//! the rectangle with the arrow keys.

extern crate terminal_graphics;
extern crate termion;

use terminal_graphics::Display;
use terminal_graphics::Colour;
use terminal_graphics::graphics;
use terminal_graphics::graphics::shapes;
use terminal_graphics::graphics::text::Text;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn main() {

    let mut screen = Display::new(100,25);
    screen.clear();

    screen.set_pixel(5,5,'o',Colour::Blue, Colour::Black);
    screen.set_pixel(4,4,'\u{2602}',Colour::Magenta, Colour::White);


    // create a graphics object
    let mut graphics = graphics::Graphics::new();

    // create a rectangle
    let rectangle = shapes::Rect::new(10, 4, 2, 3, Colour::Magenta);

    // add the rectangle to the graphics
    graphics.add(Box::new(rectangle));

    // draw the graphics:
    graphics.draw(&mut screen);

    // add some text:
    let mut text = Text::new();
    text.set_text(String::from("Hello!"));
    graphics.add(Box::new(text));

    graphics.draw(&mut screen);

    // read input from keyboard
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Left => graphics.move_graphic(0, -1, 0),
            Key::Right => graphics.move_graphic(0, 1, 0),
            Key::Down => graphics.move_graphic(0, 0, 1),
            Key::Up => graphics.move_graphic(0, 0, -1),
            _ => {}
        };
        graphics.draw(&mut screen);
        stdout.flush().unwrap();
    }
}
