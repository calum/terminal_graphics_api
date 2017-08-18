extern crate terminal_graphics;
use terminal_graphics::display::display::Display;
use terminal_graphics::display::colour::Colour;
use terminal_graphics::graphics;
use terminal_graphics::graphics::shapes;
use terminal_graphics::graphics::text::Text;

use std::io;
use std::io::Read;

fn main() {
    let mut screen = Display::new(100,25);
    screen.clear();

    screen.set_pixel(5,5,'o',Colour::Blue, Colour::Black);
    screen.set_pixel(4,4,'\u{2602}',Colour::Magenta, Colour::White);


    // create a graphics object
    let mut graphics = graphics::Graphics::new();

    // create a rectangle
    let rectangle = shapes::Rect::new(10, 4, 2,3,Colour::Magenta);

    // add the rectangle to the graphics
    graphics.add(Box::new(rectangle));

    // draw the graphics:
    graphics.draw(&mut screen);

    // add some text:
    let mut text = Text::new();
    text.set_text(String::from("Hello!"));
    graphics.add(Box::new(text));

    // read input from keyboard
    let mut input = [0; 512];
    loop {
        match io::stdin().read(&mut input) {
            Ok(n) => {
                // create a new rectangle and add to the screen
                let new_rect = shapes::Rect::new(15, 4, 2,3,Colour::Magenta);
                graphics.add(Box::new(new_rect));
                graphics.draw(&mut screen);
            },
            Err(e) => {
                // nothing
            },
        };
    }
}
