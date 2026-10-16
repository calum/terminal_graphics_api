//! Library code must never panic or hang on extreme input: it clips instead.

use std::time::{Duration, Instant};

use terminal_graphics::prelude::*;

/// Runs `f` and fails if it takes unreasonably long (e.g. looping over a
/// billion off-screen cells).
fn quickly(f: impl FnOnce()) {
    let start = Instant::now();
    f();
    assert!(
        start.elapsed() < Duration::from_secs(2),
        "took {:?}",
        start.elapsed()
    );
}

#[test]
fn rect_at_extreme_coordinates_does_not_overflow() {
    let mut d = Display::new(3, 3);
    Rect::new(isize::MAX, isize::MAX, isize::MAX, isize::MAX, Colour::Red).draw(&mut d);
    Rect::new(isize::MIN, isize::MIN, isize::MAX, isize::MAX, Colour::Red).draw(&mut d);
}

#[test]
fn huge_rect_is_clipped_without_visiting_every_cell() {
    let mut d = Display::new(3, 3);
    quickly(|| {
        Rect::new(
            -1_000_000_000,
            -1_000_000_000,
            2_000_000_000,
            2_000_000_000,
            Colour::Red,
        )
        .draw(&mut d)
    });
    assert_eq!(d.pixel(1, 1).unwrap().background(), Colour::Red);
}

#[test]
fn container_at_extreme_coordinates_does_not_overflow() {
    let mut d = Display::new(3, 3);
    let mut c = Container::new(0, 0, 1, 1, Colour::Blue, Colour::Green);
    c.set_position(isize::MAX, isize::MAX);
    c.set_width(isize::MAX);
    c.set_height(isize::MAX);
    c.draw(&mut d);
    c.set_position(isize::MIN, isize::MIN);
    c.draw(&mut d);
}

#[test]
fn huge_container_is_clipped_without_visiting_every_cell() {
    let mut d = Display::new(5, 5);
    let mut c = Container::new(0, 0, 1, 1, Colour::Blue, Colour::Green);
    c.set_position(-1_000_000_000, -1_000_000_000);
    c.set_width(2_000_000_000);
    c.set_height(2_000_000_000);
    c.set_background(Colour::Green);
    quickly(|| c.draw(&mut d));
    assert_eq!(d.pixel(2, 2).unwrap().background(), Colour::Green);
}

#[test]
fn text_at_extreme_coordinates_does_not_overflow() {
    let mut d = Display::new(3, 3);
    Text::new(
        isize::MAX - 1,
        0,
        "abcdef".to_string(),
        Colour::Red,
        Colour::Blue,
    )
    .draw(&mut d);
    Text::new(
        isize::MIN,
        isize::MIN,
        "abcdef".to_string(),
        Colour::Red,
        Colour::Blue,
    )
    .draw(&mut d);
}

#[test]
fn ball_at_extreme_positions_does_not_overflow() {
    let mut d = Display::new(3, 3);
    for pos in [f32::MAX, f32::MIN, f32::INFINITY, f32::NAN] {
        let ball = Ball::new(pos, pos, 0.0, 0.0, Colour::Red);
        ball.draw(&mut d);
        assert_eq!(ball.area().len(), 2);
    }
}

#[test]
fn moving_a_graphic_past_the_integer_limits_does_not_overflow() {
    let mut rect = Rect::new(isize::MAX, isize::MIN, 1, 1, Colour::Red);
    rect.move_by(1, -1);
    rect.move_by(isize::MAX, isize::MIN);
}
