use terminal_graphics::Colour;

#[test]
fn bright_black_is_grey_not_dark_red() {
    assert_eq!(Colour::BrightBlack.get_rbg(), (85, 85, 85));
}

#[test]
fn name_round_trips_for_every_colour() {
    for colour in Colour::variants().iter() {
        assert_eq!(Colour::of(colour.to_str()), *colour);
    }
}

#[test]
fn unknown_name_falls_back_to_white() {
    assert_eq!(Colour::of("nonsense"), Colour::White);
}

#[test]
fn every_colour_maps_to_itself_by_rgb() {
    for colour in Colour::variants().iter() {
        let (r, g, b) = colour.get_rbg();
        assert_eq!(Colour::from_rgb(r, g, b), *colour);
    }
}

#[test]
fn ansi_codes_are_unique_and_background_is_foreground_plus_ten() {
    let mut seen = std::collections::HashSet::new();
    for colour in Colour::variants().iter() {
        let (fg, bg) = colour.get_codes();
        assert_eq!(bg, fg + 10);
        assert!(seen.insert(fg), "duplicate code {}", fg);
    }
}
