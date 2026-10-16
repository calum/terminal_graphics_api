//! End-to-end: run the real demo binary and inspect what it prints.

use std::process::Command;

fn strip_ansi(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '' {
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
fn demo_binary_draws_the_title_inside_its_container() {
    let output = Command::new(env!("CARGO_BIN_EXE_main"))
        .output()
        .expect("failed to run demo binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // 50x50 display: cursor up 50 rows, then 50 rows of output
    assert!(stdout.starts_with("\x1b[50F"));
    assert_eq!(stdout.matches("\x1b[K").count(), 50);
    // the title text and the container border corner both reach the terminal
    assert!(strip_ansi(&stdout).contains("Title!"));
    assert!(stdout.contains('┌') && stdout.contains('┘'));
    assert!(output.stderr.is_empty());
}

#[test]
fn demo_binary_survives_a_closed_stdout() {
    // `| head -c 0` style consumers close the pipe early; we must not panic
    let child = Command::new(env!("CARGO_BIN_EXE_main"))
        .stdout(std::process::Stdio::null())
        .status()
        .unwrap();
    assert!(child.success());
}
