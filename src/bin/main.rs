//! # An example application
//!
//! Draws a rectangle onto the terminal and allows the user to move
//! the rectangle with the arrow keys.

extern crate terminal_graphics;
extern crate termion;

use terminal_graphics::Display;
use terminal_graphics::Colour;
use terminal_graphics::graphics::{Graphics, Graphic};
use terminal_graphics::graphics::shapes::{Rect};
use terminal_graphics::graphics::text::Text;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

use std::thread;
use std::sync::mpsc::channel;

fn main() {
    // Create the screen
    let mut screen = Display::new(100,25);
    screen.clear();

    // create a graphics object to store all the players and other
    // graphical objects
    let mut graphics = Graphics::new();

    // create the players:
    let player_1 = Rect::new(8, 4, 2, 6, Colour::Blue);
    let player_2 = Rect::new(92, 4, 2, 6, Colour::Red);
    graphics.add_named("player_1", Box::new(player_1));
    graphics.add_named("player_2", Box::new(player_2));

    // create the player's scores:
    let mut player_1_score = Text::new();
    let mut player_2_score = Text::new();
    player_1_score.set_text(String::from("Player 1: 0"));
    player_2_score.set_text(String::from("Player 2: 0"));
    player_2_score.set_position(85,0);
    graphics.add_named("player_1_score", Box::new(player_1_score));
    graphics.add_named("player_2_score", Box::new(player_2_score));

    // create the ball:
    let ball = Rect::new(50,12,2,1, Colour::Black);
    let mut ball_speed_x = 1;
    let mut ball_speed_y = 0;
    graphics.add_named("ball", Box::new(ball));

    graphics.draw(&mut screen);

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let (tx, rx) = channel();
    thread::spawn(move || {
        // read input from keyboard
        for c in stdin.keys() {
            // send the input down the channel
            tx.send(c).unwrap();
        }
    });

    // the update loop
    let mut running = true;
    while running {
        // handle user inputs:
        for c in rx.try_iter() {
            match c.unwrap() {
                Key::Char('q') => running = false,
                Key::Char('a') => graphics.move_named_graphic("player_1", -1, 0),
                Key::Char('d') => graphics.move_named_graphic("player_1", 1, 0),
                Key::Char('s') => graphics.move_named_graphic("player_1", 0, 1),
                Key::Char('w') => graphics.move_named_graphic("player_1", 0, -1),
                Key::Left => graphics.move_named_graphic("player_2", -1, 0),
                Key::Right => graphics.move_named_graphic("player_2", 1, 0),
                Key::Down => graphics.move_named_graphic("player_2", 0, 1),
                Key::Up => graphics.move_named_graphic("player_2", 0, -1),
                _ => {}
            };
        }

        // move the ball:
        graphics.move_named_graphic("ball", ball_speed_x, ball_speed_y);

        // check for collisions:
        

        graphics.draw(&mut screen);
        stdout.flush().unwrap();
    }

}
