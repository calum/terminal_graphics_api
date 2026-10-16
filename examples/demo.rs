//! An animated tour of every feature in the library.
//!
//! Run with `cargo run --example demo`. Options (after `--`):
//!
//! - `--frames N`  stop after N frames (default 300, about 10 seconds)
//! - `--no-delay`  don't wait between frames (used by the tests)
//!
//! What is on screen:
//!
//! - **Rainbow title**: `Display::set_pixel` drawn directly.
//! - **Palette strip**: all 16 `Colour::variants()` as `Rect`s, with
//!   `Colour::to_str` / `Colour::of` round-tripping the highlighted name.
//! - **Plasma** (left): a custom `Graphic` implemented in this file, shaded
//!   with `Colour::from_rgb` and held in a `Container`.
//! - **Arena** (right): bouncing `Ball`s in a `Graphics` collection, steered
//!   with `update_speeds` / `update_position`, a paddle `Rect` that uses
//!   `check_collision`, and a burst of extra balls that come and go with
//!   `add_named` / `remove_named`.
//! - **Rainbow river**: `from_rgb` mapped over the whole screen width.
//! - **Marquee**: `Text` scrolled with `move_named_graphic`, clipped at both
//!   screen edges.
//! - **HUD**: `Container`s holding `Text` updated with `set_text`, including
//!   the live byte size of a frame from `Display::write_to`.

use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use terminal_graphics::graphics::{Graphic, Graphics};
use terminal_graphics::shapes::{Ball, Rect};
use terminal_graphics::{Colour, Container, Display, Text};

const WIDTH: u32 = 64;
const HEIGHT: u32 = 28;
const DEFAULT_FRAMES: u32 = 300;
const FRAME_TIME: Duration = Duration::from_millis(33);

// the arena: a filled rectangle on the right half of the screen
const ARENA_X: isize = 33;
const ARENA_Y: isize = 6;
const ARENA_W: isize = 30;
const ARENA_H: isize = 11;
const PADDLE_W: isize = 10;

const BURST_START: u32 = 90;
const BURST_END: u32 = 200;

const MARQUEE: &str = "<<< anything off-screen is clipped, never a panic >>>";

/// A custom [`Graphic`]: a plasma effect shaded with `Colour::from_rgb`.
///
/// Any type that can draw itself and report a position can be dropped into a
/// `Container` or a `Graphics` collection.
struct Plasma {
    x: isize,
    y: isize,
    width: isize,
    height: isize,
    time: f32,
}

impl Plasma {
    /// The colour of the plasma at one cell, for a phase offset in radians.
    fn shade(&self, cx: f32, cy: f32, phase: f32) -> Colour {
        let t = self.time;
        let v = (cx * 0.35 + t).sin()
            + (cy * 0.55 + t * 1.3).sin()
            + ((cx + cy) * 0.22 + t * 0.7).sin()
            + phase;
        let wave = |shift: f32| ((v * 1.1 + shift).sin() * 127.0 + 128.0) as u8;
        Colour::from_rgb(wave(0.0), wave(2.1), wave(4.2))
    }
}

impl Graphic for Plasma {
    fn draw(&self, display: &mut Display) {
        for dy in 0..self.height {
            for dx in 0..self.width {
                // a shaded block with two different colours looks like a
                // colour between them, which squeezes more tones out of 16
                let (cx, cy) = (dx as f32, dy as f32 * 2.0);
                let front = self.shade(cx, cy, 0.0);
                let back = self.shade(cx, cy, 0.9);
                display.set_pixel(self.x + dx, self.y + dy, '▒', front, back);
            }
        }
    }

    fn get_position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    fn set_position(&mut self, pos_x: isize, pos_y: isize) {
        self.x = pos_x;
        self.y = pos_y;
    }
}

/// Which way a ball is travelling. `Ball` keeps its own speed private, so the
/// demo remembers the signs to know when to bounce.
struct Bouncer {
    name: String,
    dx: f32,
    dy: f32,
}

struct Options {
    frames: u32,
    delay: bool,
}

fn parse_options() -> Options {
    let mut options = Options {
        frames: DEFAULT_FRAMES,
        delay: true,
    };
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--frames" => {
                if let Some(frames) = args.next().and_then(|n| n.parse().ok()) {
                    options.frames = frames;
                }
            }
            "--no-delay" => options.delay = false,
            _ => {}
        }
    }
    options
}

fn bright_colours() -> Vec<Colour> {
    // the last 7 variants are the bright colours (BrightBlack is the grey)
    Colour::variants()[9..].to_vec()
}

fn new_bouncer(
    graphics: &mut Graphics,
    name: &str,
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    colour: Colour,
) -> Bouncer {
    graphics.add_named(name, Box::new(Ball::new(x, y, dx, dy, colour)));
    Bouncer {
        name: name.to_string(),
        dx,
        dy,
    }
}

/// Moves every ball one step and bounces it off the arena walls. Returns how
/// many balls were stopped by the paddle.
fn step_balls(graphics: &mut Graphics, bouncers: &mut [Bouncer]) -> u32 {
    let mut paddle_hits = 0;
    let left = ARENA_X;
    let right = ARENA_X + ARENA_W - 2; // a ball is two cells wide
    let top = ARENA_Y;
    let floor = ARENA_Y + ARENA_H - 1; // the paddle's row

    for bouncer in bouncers.iter_mut() {
        let Some(ball) = graphics.get_mut_named(&bouncer.name) else {
            continue;
        };
        ball.update_position();
        let (x, y) = ball.get_position();

        if (x <= left && bouncer.dx < 0.0) || (x >= right && bouncer.dx > 0.0) {
            ball.update_speeds(-1.0, 1.0);
            bouncer.dx = -bouncer.dx;
        }
        if (y <= top && bouncer.dy < 0.0) || (y >= floor && bouncer.dy > 0.0) {
            ball.update_speeds(1.0, -1.0);
            bouncer.dy = -bouncer.dy;
        }
        // rounding can leave a ball one step outside the walls; pull it back in
        let (cx, cy) = (x.clamp(left, right), y.clamp(top, floor));
        if (cx, cy) != (x, y) {
            // set_position drops the fractional part, so only use it when needed
            ball.set_position(cx, cy);
        }
        let y = cy;

        if y == floor {
            let area = ball.get_area();
            let on_paddle = graphics
                .get_named("paddle")
                .is_some_and(|paddle| area.iter().any(|&(ax, ay)| paddle.check_collision(ax, ay)));
            if on_paddle {
                paddle_hits += 1;
            }
        }
    }
    paddle_hits
}

/// Slides the paddle towards the lowest ball so it keeps catching things.
fn steer_paddle(graphics: &mut Graphics, bouncers: &[Bouncer]) {
    let mut target = None;
    let mut lowest = isize::MIN;
    for bouncer in bouncers {
        if let Some(ball) = graphics.get_named(&bouncer.name) {
            let (x, y) = ball.get_position();
            if y > lowest {
                lowest = y;
                target = Some(x);
            }
        }
    }
    let Some(target) = target else { return };
    let Some(paddle) = graphics.get_named("paddle") else {
        return;
    };
    let (paddle_x, _) = paddle.get_position();
    let want = (target - PADDLE_W / 2 + 1).clamp(ARENA_X, ARENA_X + ARENA_W - PADDLE_W);
    graphics.move_named_graphic("paddle", (want - paddle_x).clamp(-2, 2), 0);
}

/// Paints the rainbow title one letter at a time with `set_pixel`.
fn draw_title(display: &mut Display, frame: u32) {
    let title = "TERMINAL GRAPHICS";
    let colours = bright_colours();
    let start = (WIDTH as usize - title.len()) / 2;
    for (i, letter) in title.chars().enumerate() {
        let colour = colours[(i + frame as usize / 3) % colours.len()];
        display.set_pixel((start + i) as isize, 0, letter, colour, Colour::Black);
    }
    let subtitle = "zero dependencies | 100% Rust | 16 ANSI colours";
    let start = (WIDTH as usize - subtitle.len()) / 2;
    Text::from(
        start as isize,
        1,
        subtitle.to_string(),
        Colour::BrightBlack,
        Colour::Black,
    )
    .draw(display);
}

/// The rainbow river: `from_rgb` mapped across the width of the screen.
fn draw_river(display: &mut Display, frame: u32) {
    let t = frame as f32 * 0.12;
    for row in 0..3 {
        for x in 0..WIDTH as isize {
            let hue = x as f32 * 0.16 - t + row as f32 * 0.7;
            let wave = |shift: f32| ((hue + shift).sin() * 127.0 + 128.0) as u8;
            let colour = Colour::from_rgb(wave(0.0), wave(2.1), wave(4.2));
            display.set_pixel(x, 18 + row, ' ', Colour::White, colour);
        }
    }
}

fn main() {
    let options = parse_options();

    let mut display = Display::new(WIDTH, HEIGHT);
    let mut graphics = Graphics::new();

    // drawing order is insertion order: the backdrop first, then the rest
    graphics.add_named(
        "backdrop",
        Box::new(Rect::new(
            0,
            0,
            WIDTH as isize,
            HEIGHT as isize,
            Colour::Black,
        )),
    );
    for (i, colour) in Colour::variants().into_iter().enumerate() {
        graphics.add_named(
            &format!("swatch{i}"),
            Box::new(Rect::new(i as isize * 4, 2, 4, 2, colour)),
        );
    }
    graphics.add_named(
        "arena",
        Box::new(Rect::new(
            ARENA_X,
            ARENA_Y,
            ARENA_W,
            ARENA_H,
            Colour::BrightBlack,
        )),
    );
    graphics.add_named(
        "paddle",
        Box::new(Rect::new(
            ARENA_X + 10,
            ARENA_Y + ARENA_H - 1,
            PADDLE_W,
            1,
            Colour::BrightWhite,
        )),
    );
    graphics.add_named(
        "marquee",
        Box::new(Text::from(
            WIDTH as isize,
            21,
            MARQUEE.to_string(),
            Colour::BrightYellow,
            Colour::Black,
        )),
    );
    graphics.add_named(
        "caption_plasma",
        Box::new(Text::from(
            1,
            5,
            "custom Graphic + from_rgb".to_string(),
            Colour::BrightCyan,
            Colour::Black,
        )),
    );
    graphics.add_named(
        "caption_arena",
        Box::new(Text::from(
            ARENA_X,
            5,
            "Graphics + check_collision".to_string(),
            Colour::BrightGreen,
            Colour::Black,
        )),
    );
    graphics.add_named(
        "footer",
        Box::new(Text::from(
            1,
            27,
            "cargo add terminal_graphics    |    Ctrl-C to quit".to_string(),
            Colour::BrightBlack,
            Colour::Black,
        )),
    );

    let balls = [
        ("ball0", 36.0, 8.0, 0.9, 0.35, Colour::BrightRed),
        ("ball1", 44.0, 10.0, -0.7, 0.25, Colour::BrightYellow),
        ("ball2", 52.0, 7.0, 0.5, -0.4, Colour::BrightCyan),
        ("ball3", 40.0, 12.0, -1.1, -0.3, Colour::BrightMagenta),
    ];
    let mut bouncers: Vec<Bouncer> = balls
        .iter()
        .map(|&(name, x, y, dx, dy, colour)| new_bouncer(&mut graphics, name, x, y, dx, dy, colour))
        .collect();

    let mut status = Text::from(1, 1, String::new(), Colour::BrightGreen, Colour::Black);
    let mut bytes_text = Text::from(
        1,
        1,
        String::new(),
        Colour::of("BrightYellow"),
        Colour::Black,
    );
    let mut colour_text = Text::from(1, 1, String::new(), Colour::BrightWhite, Colour::Black);

    let mut bounces = 0;
    let mut frame_bytes = 0;

    // make room for the display: each frame moves the cursor back up over the
    // previous one, so there must be that many lines to move up over
    print!("{}", "\n".repeat(HEIGHT as usize));

    for frame in 0..options.frames {
        // --- update ---
        steer_paddle(&mut graphics, &bouncers);
        bounces += step_balls(&mut graphics, &mut bouncers);
        graphics.move_named_graphic("marquee", -1, 0);
        if let Some(marquee) = graphics.get_named("marquee") {
            if marquee.get_position().0 < -(MARQUEE.len() as isize) {
                graphics.move_named_graphic("marquee", WIDTH as isize + MARQUEE.len() as isize, 0);
            }
        }

        // balls join the arena for a while, then are removed again
        if frame == BURST_START {
            for i in 0..3 {
                let name = format!("burst{i}");
                let x = ARENA_X as f32 + 4.0 + i as f32 * 9.0;
                let dx = if i % 2 == 0 { 1.0 } else { -1.0 };
                bouncers.push(new_bouncer(
                    &mut graphics,
                    &name,
                    x,
                    14.0,
                    dx,
                    -0.45,
                    Colour::BrightWhite,
                ));
            }
        }
        if frame == BURST_END {
            for i in 0..3 {
                graphics.remove_named(&format!("burst{i}"));
            }
            bouncers.retain(|b| !b.name.starts_with("burst"));
        }

        // --- draw ---
        // render_to clears the display and draws every named graphic, but
        // doesn't print, so we can keep painting on top before printing
        graphics.render_to(&mut display);

        draw_title(&mut display, frame);

        // the palette marker: Colour::to_str and Colour::of agree
        let highlighted = (frame as usize / 4) % 16;
        let highlight_colour = Colour::variants()[highlighted];
        let name = highlight_colour.to_str();
        assert_eq!(Colour::of(name), highlight_colour);
        Text::from(
            highlighted as isize * 4 + 1,
            4,
            "^".to_string(),
            Colour::BrightWhite,
            Colour::Black,
        )
        .draw(&mut display);
        colour_text.set_text(format!("Colour::{name}"));
        // the label follows the marker, on whichever side has room
        let label_len = "Colour::".len() + name.len();
        let marker_x = highlighted as isize * 4 + 1;
        let label_x = if marker_x + 2 + label_len as isize <= WIDTH as isize {
            marker_x + 2
        } else {
            marker_x - 1 - label_len as isize
        };
        colour_text.set_position(label_x, 4);
        colour_text.draw(&mut display);

        draw_river(&mut display, frame);

        // the plasma lives in a Container; the container offsets it by its
        // own position, so the plasma is positioned relative to the container
        let mut plasma = Plasma {
            x: 0,
            y: 0,
            width: 30,
            height: 10,
            time: frame as f32 * 0.09,
        };
        Container::from(1, 6, 30, 10, &mut plasma, Colour::BrightCyan, Colour::Black)
            .draw(&mut display);

        status.set_text(format!("frame {frame:>4}   bounces {bounces:>3}"));
        status.set_position(1, 1);
        Container::from(
            1,
            23,
            30,
            3,
            &mut status,
            Colour::BrightGreen,
            Colour::Black,
        )
        .draw(&mut display);

        bytes_text.set_text(format!("{frame_bytes:>6} bytes/frame"));
        bytes_text.set_position(1, 1);
        Container::from(
            33,
            23,
            30,
            3,
            &mut bytes_text,
            Colour::BrightYellow,
            Colour::Black,
        )
        .draw(&mut display);

        // --- show ---
        // print() is just write_to(stdout); here we do the same through
        // write_to so we can measure the frame, then keep stdout flushed
        let mut bytes = Vec::new();
        let _ = display.write_to(&mut bytes);
        frame_bytes = bytes.len();
        display.print();

        if options.delay {
            sleep(FRAME_TIME);
        }
    }

    // leave the cursor below the picture and the terminal colours reset
    let _ = std::io::stdout().flush();
    println!("\x1b[0m");
}
