//! Integration tests: public API only, asserting on rendered output.

use terminal_graphics::prelude::*;

fn strip_ansi(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars();
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

    assert!(out.contains("\x1b[31;44mx"), "got {:?}", out);
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
    Container::new(3, 3, 10, 10, Colour::Blue, Colour::Black)
        .with_content(Text::new(0, 0, "hi", Colour::Red, Colour::Black))
        .draw(&mut display);
}

#[test]
fn container_entirely_outside_display_does_not_panic() {
    let mut display = Display::new(5, 5);
    Container::new(-50, 100, 10, 10, Colour::Blue, Colour::Black).draw(&mut display);
    assert_eq!(display, Display::new(5, 5));
}

#[test]
fn a_moved_container_takes_its_text_with_it() {
    let mut container = Container::new(0, 0, 8, 3, Colour::Blue, Colour::Black)
        .with_content(Text::new(0, 0, "hi", Colour::Red, Colour::Black));
    container.move_by(2, 1);

    let mut display = Display::new(12, 6);
    container.draw(&mut display);

    let rows: Vec<String> = strip_ansi(&display.render())
        .replace('\x20', ".")
        .chars()
        .collect::<Vec<_>>()
        .chunks(12)
        .map(|row| row.iter().collect())
        .collect();
    assert_eq!(rows[1], "..┌──────┐..");
    assert_eq!(rows[2], "..│hi....│..");
    assert_eq!(rows[3], "..└──────┘..");
}

#[test]
fn text_is_clipped_at_display_edge() {
    let mut display = Display::new(4, 1);
    Text::new(2, 0, "abcdef", Colour::Red, Colour::Black).draw(&mut display);
    assert_eq!(strip_ansi(&display.render()), "  ab");
}

#[test]
fn later_added_graphics_draw_on_top_deterministically() {
    for _ in 0..20 {
        let mut graphics = Graphics::new();
        graphics
            .add("a", Rect::new(0, 0, 1, 1, Colour::Red))
            .add("b", Rect::new(0, 0, 1, 1, Colour::Blue))
            .add("c", Rect::new(0, 0, 1, 1, Colour::Green));

        let mut display = Display::new(1, 1);
        graphics.draw(&mut display);

        assert_eq!(display.pixel(0, 0).unwrap().background(), Colour::Green);
    }
}

#[test]
fn re_adding_a_name_replaces_but_keeps_one_entry() {
    let mut graphics = Graphics::new();
    graphics.add("a", Rect::new(0, 0, 1, 1, Colour::Red));
    graphics.add("a", Rect::new(5, 5, 1, 1, Colour::Red));
    assert_eq!(graphics.len(), 1);
    assert_eq!(graphics.get("a").unwrap().position(), (5, 5));
    assert!(graphics.remove("a").is_some());
    assert!(graphics.is_empty());
}

#[test]
fn moving_a_missing_graphic_is_a_no_op() {
    let mut graphics = Graphics::new();
    assert!(!graphics.move_by("ghost", 1, 1));
}

#[test]
fn an_animation_loop_moves_balls_and_redraws_cleanly() {
    let mut graphics = Graphics::new();
    graphics.add("ball", Ball::new(0.0, 0.0, 1.0, 0.0, Colour::Red));
    let mut display = Display::new(6, 1);

    for frame in 1..=3 {
        graphics.update();
        display.clear();
        graphics.draw(&mut display);
        // exactly the two ball cells are red, and they move one cell a frame
        let red: Vec<isize> = (0..6)
            .filter(|&x| display.pixel(x, 0).unwrap().background() == Colour::Red)
            .collect();
        assert_eq!(red, vec![frame, frame + 1]);
    }
}

#[test]
fn a_run_of_same_coloured_cells_sets_the_colours_once() {
    let mut display = Display::new(5, 2);
    for y in 0..2 {
        for x in 0..5 {
            display.set_pixel(x, y, '#', Colour::Red, Colour::Blue);
        }
    }

    let out = display.render();

    // once per row, not once per cell
    assert_eq!(out.matches("\x1b[31;44m").count(), 2, "got {:?}", out);
    assert_eq!(out.matches("#####").count(), 2, "got {:?}", out);
}

#[test]
fn a_colour_change_inside_a_row_sets_the_new_colours() {
    let mut display = Display::new(3, 1);
    display.set_pixel(0, 0, 'a', Colour::Red, Colour::Blue);
    display.set_pixel(1, 0, 'b', Colour::Red, Colour::Blue);
    display.set_pixel(2, 0, 'c', Colour::Green, Colour::Blue);

    let out = display.render();

    assert!(out.contains("\x1b[31;44mab\x1b[32;44mc"), "got {:?}", out);
}

#[test]
fn colours_are_reset_before_each_row_is_erased() {
    // erasing with a colour still set would paint the rest of the line in it
    let mut display = Display::new(2, 2);
    display.set_pixel(1, 0, 'x', Colour::Red, Colour::Blue);

    let out = display.render();

    assert_eq!(out.matches("\x1b[0;0m\x1b[K").count(), 2, "got {:?}", out);
}

#[test]
fn colours_do_not_carry_over_from_one_row_to_the_next() {
    let mut display = Display::new(1, 2);
    display.set_pixel(0, 0, 'a', Colour::Red, Colour::Blue);
    display.set_pixel(0, 1, 'b', Colour::Red, Colour::Blue);

    let out = display.render();

    assert_eq!(out.matches("\x1b[31;44m").count(), 2, "got {:?}", out);
}
