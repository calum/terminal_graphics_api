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
//! - **Palette strip**: all 16 `Colour::ALL` as `Rect`s, with
//!   `Colour::name` / `str::parse` round-tripping the highlighted name.
//! - **Plasma** (left): a custom `Graphic` implemented in this file, shaded
//!   with `Colour::from_rgb` and held in a `Container`.
//! - **Arena** (right): bouncing `Ball`s in a `Graphics` collection, steered
//!   with `scale_speed` / `update`, a paddle `Rect` that uses `contains`, and
//!   a burst of extra balls that come and go with `add` / `remove`.
//! - **Rainbow river**: `from_rgb` mapped over the whole screen width.
//! - **Marquee**: `Text` scrolled with `Graphics::move_by`, clipped at both
//!   screen edges.
//! - **HUD**: `Container`s holding `Text`, including the live byte size of a
//!   frame from `Display::write_to`.

use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use terminal_graphics::shapes::{Ball, Rect};
use terminal_graphics::{Colour, Container, Display, Graphic, Graphics, Text};

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

    fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    fn set_position(&mut self, pos_x: isize, pos_y: isize) {
        self.x = pos_x;
        self.y = pos_y;
    }
}

/// Which way a ball is travelling. The demo remembers the signs to know when
/// to bounce.
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
    // the last 7 colours are the bright ones (BrightBlack is the grey)
    Colour::ALL[9..].to_vec()
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
    graphics.add(name, Ball::new(x, y, dx, dy, colour));
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
        let Some(ball) = graphics.get_mut(&bouncer.name) else {
            continue;
        };
        ball.update();
        let (x, y) = ball.position();

        if (x <= left && bouncer.dx < 0.0) || (x >= right && bouncer.dx > 0.0) {
            ball.scale_speed(-1.0, 1.0);
            bouncer.dx = -bouncer.dx;
        }
        if (y <= top && bouncer.dy < 0.0) || (y >= floor && bouncer.dy > 0.0) {
            ball.scale_speed(1.0, -1.0);
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
            let area = ball.area();
            let on_paddle = graphics
                .get("paddle")
                .is_some_and(|paddle| area.iter().any(|&(ax, ay)| paddle.contains(ax, ay)));
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
        if let Some(ball) = graphics.get(&bouncer.name) {
            let (x, y) = ball.position();
            if y > lowest {
                lowest = y;
                target = Some(x);
            }
        }
    }
    let Some(target) = target else { return };
    let Some(paddle) = graphics.get("paddle") else {
        return;
    };
    let (paddle_x, _) = paddle.position();
    let want = (target - PADDLE_W / 2 + 1).clamp(ARENA_X, ARENA_X + ARENA_W - PADDLE_W);
    graphics.move_by("paddle", (want - paddle_x).clamp(-2, 2), 0);
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
    Text::new(
        start as isize,
        1,
        subtitle,
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

/// A labelled HUD box: a bordered `Container` holding one line of `Text`.
fn hud_box(x: isize, text: String, colour: Colour) -> Container {
    Container::new(x, 23, 31, 4, colour, Colour::Black).with_content(Text::new(
        0,
        0,
        text,
        colour,
        Colour::Black,
    ))
}

fn main() {
    let options = parse_options();

    let mut display = Display::new(WIDTH, HEIGHT);
    let mut graphics = Graphics::new();

    // drawing order is insertion order: the backdrop first, then the rest
    graphics.add(
        "backdrop",
        Rect::new(0, 0, WIDTH as isize, HEIGHT as isize, Colour::Black),
    );
    for (i, colour) in Colour::ALL.into_iter().enumerate() {
        graphics.add(
            format!("swatch{i}"),
            Rect::new(i as isize * 4, 2, 4, 2, colour),
        );
    }
    graphics
        .add(
            "arena",
            Rect::new(ARENA_X, ARENA_Y, ARENA_W, ARENA_H, Colour::BrightBlack),
        )
        .add(
            "paddle",
            Rect::new(
                ARENA_X + 10,
                ARENA_Y + ARENA_H - 1,
                PADDLE_W,
                1,
                Colour::BrightWhite,
            ),
        )
        .add(
            "marquee",
            Text::new(
                WIDTH as isize,
                21,
                MARQUEE,
                Colour::BrightYellow,
                Colour::Black,
            ),
        )
        .add(
            "caption_plasma",
            Text::new(
                1,
                5,
                "custom Graphic + from_rgb",
                Colour::BrightCyan,
                Colour::Black,
            ),
        )
        .add(
            "caption_arena",
            Text::new(
                ARENA_X,
                5,
                "Graphics + contains",
                Colour::BrightGreen,
                Colour::Black,
            ),
        )
        .add(
            "footer",
            Text::new(
                1,
                27,
                "cargo add terminal_graphics    |    Ctrl-C to quit",
                Colour::BrightBlack,
                Colour::Black,
            ),
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

    let mut bounces = 0;
    let mut frame_bytes = 0;

    // make room for the display: each frame moves the cursor back up over the
    // previous one, so there must be that many lines to move up over
    if display.reserve_space().is_err() {
        return;
    }

    for frame in 0..options.frames {
        // --- update ---
        steer_paddle(&mut graphics, &bouncers);
        bounces += step_balls(&mut graphics, &mut bouncers);
        graphics.move_by("marquee", -1, 0);
        if let Some(marquee) = graphics.get("marquee") {
            if marquee.position().0 < -(MARQUEE.len() as isize) {
                graphics.move_by("marquee", WIDTH as isize + MARQUEE.len() as isize, 0);
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
                graphics.remove(&format!("burst{i}"));
            }
            bouncers.retain(|b| !b.name.starts_with("burst"));
        }

        // --- draw ---
        // clear and draw every named graphic, but don't print yet, so we can
        // keep painting on top before printing
        display.clear();
        graphics.draw(&mut display);

        draw_title(&mut display, frame);

        // the palette marker: Colour::name and str::parse agree
        let highlighted = (frame as usize / 4) % 16;
        let highlight_colour = Colour::ALL[highlighted];
        let name = highlight_colour.name();
        assert_eq!(name.parse(), Ok(highlight_colour));
        let marker_x = highlighted as isize * 4 + 1;
        Text::new(marker_x, 4, "^", Colour::BrightWhite, Colour::Black).draw(&mut display);
        // the label follows the marker, on whichever side has room
        let label = format!("Colour::{name}");
        let label_x = if marker_x + 2 + label.len() as isize <= WIDTH as isize {
            marker_x + 2
        } else {
            marker_x - 1 - label.len() as isize
        };
        Text::new(label_x, 4, label, Colour::BrightWhite, Colour::Black).draw(&mut display);

        draw_river(&mut display, frame);

        // the plasma lives in a Container; its position is relative to the
        // inside of the container
        let plasma = Plasma {
            x: 0,
            y: 0,
            width: 29,
            height: 9,
            time: frame as f32 * 0.09,
        };
        Container::new(1, 6, 31, 11, Colour::BrightCyan, Colour::Black)
            .with_content(plasma)
            .draw(&mut display);

        let status = format!("frame {frame:>4}   bounces {bounces:>3}");
        hud_box(1, status, Colour::BrightGreen).draw(&mut display);
        let bytes = format!("{frame_bytes:>6} bytes/frame");
        hud_box(33, bytes, Colour::BrightYellow).draw(&mut display);

        // --- show ---
        // print() is just write_to(stdout); here we do the same through
        // write_to so we can measure the frame
        let mut bytes = Vec::new();
        let _ = display.write_to(&mut bytes);
        frame_bytes = bytes.len();
        if display.print().is_err() {
            // stdout was closed (e.g. a broken pipe): stop quietly
            return;
        }

        if options.delay {
            sleep(FRAME_TIME);
        }
    }

    // leave the cursor below the picture and the terminal colours reset
    let mut stdout = std::io::stdout();
    let _ = writeln!(stdout, "\x1b[0m");
    let _ = stdout.flush();
}
