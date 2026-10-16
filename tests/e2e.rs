//! End-to-end: run the real demo example and inspect what it prints.

use std::process::Command;

/// Height of the demo display, which is also how far each frame moves the
/// cursor up.
const HEIGHT: usize = 28;

fn demo_command(frames: u32) -> Command {
    let mut cmd = Command::new(env!("CARGO"));
    cmd.args(["run", "--quiet", "--example", "demo", "--"])
        .args(["--frames", &frames.to_string(), "--no-delay"])
        .current_dir(env!("CARGO_MANIFEST_DIR"));
    cmd
}

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

fn run_demo(frames: u32) -> String {
    let output = demo_command(frames)
        .output()
        .expect("failed to run demo example");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    String::from_utf8(output.stdout).unwrap()
}

/// Splits stdout into its frames; each frame starts with the cursor-up move.
fn frames(stdout: &str) -> Vec<&str> {
    let marker = format!("\x1b[{HEIGHT}F");
    stdout.split(marker.as_str()).skip(1).collect()
}

#[test]
fn demo_prints_the_requested_number_of_full_frames() {
    let stdout = run_demo(4);

    let frames = frames(&stdout);
    assert_eq!(frames.len(), 4);
    for frame in frames {
        assert_eq!(frame.matches("\x1b[K").count(), HEIGHT);
    }
}

#[test]
fn demo_shows_the_title_boxes_and_live_stats() {
    let stdout = run_demo(3);
    let text = strip_ansi(&stdout);

    assert!(text.contains("TERMINAL GRAPHICS"));
    assert!(stdout.contains('┌') && stdout.contains('┘'));
    assert!(stdout.contains('│') && stdout.contains('─'));
    assert!(text.contains("frame"));
    assert!(text.contains("bounces"));
}

#[test]
fn demo_shows_the_whole_colour_palette() {
    let stdout = run_demo(1);

    // background codes: 40-47 standard, 100-107 bright
    for code in (40..=47).chain(100..=107) {
        assert!(
            stdout.contains(&format!(";{code}m")),
            "background colour code {code} never appears"
        );
    }
}

#[test]
fn demo_animates_between_frames() {
    let stdout = run_demo(6);

    let frames = frames(&stdout);
    assert_ne!(frames[0], frames[5]);
}

#[test]
fn demo_survives_a_closed_stdout() {
    // `| head -c 0` style consumers close the pipe early; we must not panic
    let child = demo_command(2)
        .stdout(std::process::Stdio::null())
        .status()
        .unwrap();
    assert!(child.success());
}
