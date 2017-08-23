//! # An example application
//!
//! Draws a rectangle onto the terminal and allows the user to move
//! the rectangle with the arrow keys.

extern crate terminal_graphics;
extern crate termion;
extern crate rand;

use terminal_graphics::Display;
use terminal_graphics::Colour;
use terminal_graphics::graphics::{Graphics, Graphic};
use terminal_graphics::graphics::shapes::{Rect, Ball};
use terminal_graphics::graphics::text::Text;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

use std::thread;
use std::sync::mpsc::channel;

use rand::distributions::{IndependentSample, Range};

fn main() {
    let screen_width = 100;
    let screen_height = 25;

    // Create the screen
    let mut screen = Display::new(screen_width,screen_height);
    screen.clear();

    // create a graphics object to store all the players and other
    // graphical objects
    let mut graphics = Graphics::new();

    // create the players:
    let player_1 = Rect::new(8, 4, 2, 6, Colour::Blue);
    let player_2 = Rect::new(92, 4, 2, 6, Colour::Red);
    graphics.add_named("player_1", Box::new(player_1));
    graphics.add_named("player_2", Box::new(player_2));

    // hold the scores:
    let mut player_1_score_total = 0;
    let mut player_2_score_total = 0;

    // create the player's scores:
    let mut player_1_score = Text::new();
    let mut player_2_score = Text::new();
    player_1_score.set_text(String::from("Player 1: 0"));
    player_2_score.set_text(String::from("Player 2: 0"));
    player_2_score.set_position(85,0);
    graphics.add_named("player_1_score", Box::new(player_1_score));
    graphics.add_named("player_2_score", Box::new(player_2_score));

    // create the ball:
    let ball = Ball::new(50.0, 12.0, 0.3, 0.2, Colour::Black);
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

    // set up the random number genertor:
    let between = Range::new(-2.0, 2.0);
    let mut rng = rand::thread_rng();

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
        graphics.get_mut_named("ball").unwrap().update_position();

        // check for collisions:
        let (x, y) = graphics.get_mut_named("ball").unwrap().get_position();
        if graphics.get_named("player_1").unwrap().check_collision(x, y) {
            let speed_y = between.ind_sample(&mut rng);
            graphics.get_mut_named("ball").unwrap().update_speeds(-1.0, speed_y);
        }
        if graphics.get_named("player_2").unwrap().check_collision(x, y) {
            let speed_y = between.ind_sample(&mut rng);
            graphics.get_mut_named("ball").unwrap().update_speeds(-1.0, speed_y);
        }

        // check if ball is out of the bounds
        match graphics.get_mut_named("ball").unwrap().get_position() {
            (x, _) if x < 0 || x > screen_width as isize => {
                // a point has been scored!
                if x < 0 {
                    // player two scored:
                    player_2_score_total += 1;
                    graphics.remove_named("player_2_score");
                    let mut player_2_score = Text::new();
                    player_2_score.set_text(format!("Player 1: {}", player_2_score_total));
                    player_2_score.set_position(85,0);
                    graphics.add_named("player_2_score", Box::new(player_2_score));
                } else {
                    // player one scored:
                    player_1_score_total += 1;
                    graphics.remove_named("player_1_score");
                    let mut player_1_score = Text::new();
                    player_1_score.set_text(format!("Player 1: {}", player_1_score_total));
                    graphics.add_named("player_1_score", Box::new(player_1_score));
                }
                graphics.get_mut_named("ball").unwrap()
                        .set_position(50, 12);
            },
            (_, y) if y < 0 || y > screen_height as isize => graphics.get_mut_named("ball").unwrap().update_speeds(1.0, -1.0),
            _ => {
                //do nothing
            },
        };

        graphics.draw(&mut screen);
        stdout.flush().unwrap();
    }

}
