//! `llms.txt` is the page AI coding agents read to decide whether to use this
//! crate and how. These tests keep it from drifting away from the API.

use std::fs;
use std::path::Path;

fn llms() -> String {
    fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("llms.txt"))
        .expect("llms.txt must exist at the repo root")
}

#[test]
fn starts_with_the_crate_name_and_a_one_line_summary() {
    let text = llms();
    let mut lines = text.lines();
    assert_eq!(lines.next(), Some("# terminal_graphics"));
    assert!(lines.next().unwrap_or("").is_empty());
    assert!(lines.next().unwrap_or("").starts_with("> "));
}

#[test]
fn names_every_public_type_so_agents_can_find_them() {
    let text = llms();
    for name in [
        "Display",
        "Pixel",
        "Colour",
        "Graphic",
        "Graphics",
        "Container",
        "Text",
        "Rect",
        "Ball",
    ] {
        assert!(text.contains(name), "llms.txt does not mention {name}");
    }
}

#[test]
fn says_when_to_use_and_when_not_to_use_the_crate() {
    let text = llms();
    assert!(text.contains("## Use it when"));
    assert!(text.contains("## Do not use it when"));
}

#[test]
fn includes_a_complete_example_that_matches_the_real_api() {
    let text = llms();
    assert!(text.contains("```rust"));
    for call in [
        "Display::new(",
        "reserve_space()",
        ".add(",
        "graphics.update()",
        "graphics.show(",
    ] {
        assert!(text.contains(call), "example is missing {call}");
    }
}

#[test]
fn links_to_the_docs_and_the_crate() {
    let text = llms();
    assert!(text.contains("https://docs.rs/terminal_graphics"));
    assert!(text.contains("https://crates.io/crates/terminal_graphics"));
}

#[test]
fn points_to_the_interactive_companion_for_keyboard_and_mouse_input() {
    let text = llms();
    assert!(text.contains("terminal_graphics_interactive"));
    assert!(text.contains("https://github.com/calum/terminal_graphics_interactive"));
    assert!(
        !text.contains("mouse and keyboard events. Use a TUI framework"),
        "mouse and keyboard input now has a better answer than a TUI framework"
    );
}

#[test]
fn the_readme_and_crate_docs_point_to_the_interactive_companion_too() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for file in ["README.md", "src/lib.rs"] {
        let text = fs::read_to_string(root.join(file)).unwrap();
        assert!(
            text.contains("terminal_graphics_interactive"),
            "{file} does not mention terminal_graphics_interactive"
        );
    }
}
