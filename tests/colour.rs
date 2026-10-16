//! Integration tests for `Colour`, using only the public API.

use std::collections::HashMap;
use terminal_graphics::{Colour, ParseColourError};

#[test]
fn names_round_trip_through_display_and_parse() {
    for colour in Colour::ALL {
        let name = colour.to_string();
        assert_eq!(name.parse::<Colour>(), Ok(colour));
    }
}

#[test]
fn unknown_names_are_an_error_not_a_silent_fallback() {
    let err: ParseColourError = "nonsense".parse::<Colour>().unwrap_err();
    assert!(err.to_string().contains("nonsense"));
    // a real error type, so `?` into `Box<dyn Error>` works
    let _: Box<dyn std::error::Error> = Box::new(err);
}

#[test]
fn every_colour_maps_to_itself_by_rgb() {
    for colour in Colour::ALL {
        let (r, g, b) = colour.rgb();
        assert_eq!(Colour::from_rgb(r, g, b), colour);
    }
}

#[test]
fn colours_can_key_a_hash_map() {
    let mut counts = HashMap::new();
    for colour in [Colour::Red, Colour::Blue, Colour::Red] {
        *counts.entry(colour).or_insert(0) += 1;
    }
    assert_eq!(counts[&Colour::Red], 2);
    assert_eq!(counts[&Colour::Blue], 1);
}
