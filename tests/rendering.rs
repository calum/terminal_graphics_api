//! Integration tests: public API only, asserting on rendered output.

use terminal_graphics::graphics::shapes::Rect;
use terminal_graphics::graphics::Graphic;
use terminal_graphics::graphics::Graphics;
use terminal_graphics::{Colour, Container, Display, Text};

fn strip_ansi(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            for n in chars.by_ref() {
                if n.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

#[test]
fn render_includes_pixel_characters_and_colour_codes() {
    let mut display = Display::new(3, 1);
    display.set_pixel(1, 0, 'x', Colour::Red, Colour::Blue);

    let out = display.render();

    assert!(out.contains("\x1b[31;44mx\x1b[0;0m"), "got {:?}", out);
    assert_eq!(strip_ansi(&out), " x ");
}

#[test]
fn write_to_matches_render() {
    let mut display = Display::new(4, 2);
    display.set_pixel(0, 0, 'a', Colour::Green, Colour::Black);

    let mut buf = Vec::new();
    display.write_to(&mut buf).unwrap();

    assert_eq!(String::from_utf8(buf).unwrap(), display.render());
}

#[test]
fn zero_sized_display_ignores_drawing_instead_of_panicking() {
    let mut display = Display::new(0, 0);
    display.set_pixel(0, 0, 'x', Colour::Red, Colour::Blue);
    assert_eq!(strip_ansi(&display.render()), "");
}

#[test]
fn container_partly_outside_display_does_not_panic() {
    let mut display = Display::new(5, 5);
    let mut text = Text::from(0, 0, "hi".to_string(), Colour::Red, Colour::Black);
    let container = Container::from(3, 3, 10, 10, &mut text, Colour::Blue, Colour::Black);
    container.draw(&mut display);
}

#[test]
fn container_entirely_outside_display_does_not_panic() {
    let mut display = Display::new(5, 5);
    let mut container = Container::new();
    container.set_position(-50, 100);
    container.draw(&mut display);
}

#[test]
fn text_is_clipped_at_display_edge() {
    let mut display = Display::new(4, 1);
    Text::from(2, 0, "abcdef".to_string(), Colour::Red, Colour::Black).draw(&mut display);
    assert_eq!(strip_ansi(&display.render()), "  ab");
}

#[test]
fn later_added_graphics_draw_on_top_deterministically() {
    for _ in 0..20 {
        let mut graphics = Graphics::new();
        graphics.add_named("a", Box::new(Rect::new(0, 0, 1, 1, Colour::Red)));
        graphics.add_named("b", Box::new(Rect::new(0, 0, 1, 1, Colour::Blue)));
        graphics.add_named("c", Box::new(Rect::new(0, 0, 1, 1, Colour::Green)));

        let mut display = Display::new(1, 1);
        graphics.render_to(&mut display);

        assert!(
            display.render().contains("\x1b[37;42m"),
            "last added must win"
        );
    }
}

#[test]
fn moving_a_missing_graphic_is_a_no_op() {
    let mut graphics = Graphics::new();
    graphics.move_named_graphic("ghost", 1, 1);
}

#[test]
fn re_adding_a_name_replaces_but_keeps_one_entry() {
    let mut graphics = Graphics::new();
    graphics.add_named("a", Box::new(Rect::new(0, 0, 1, 1, Colour::Red)));
    graphics.add_named("a", Box::new(Rect::new(5, 5, 1, 1, Colour::Red)));
    assert_eq!(graphics.get_named("a").unwrap().get_position(), (5, 5));
    graphics.remove_named("a");
    assert!(graphics.get_named("a").is_none());
}
