//! End-to-end: run the real demo example and inspect what it prints.

use std::process::Command;

fn demo_command() -> Command {
    let mut cmd = Command::new(env!("CARGO"));
    cmd.args(["run", "--quiet", "--example", "demo"])
        .current_dir(env!("CARGO_MANIFEST_DIR"));
    cmd
}

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
fn demo_example_draws_the_title_inside_its_container() {
    let output = demo_command().output().expect("failed to run demo example");

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
fn demo_example_survives_a_closed_stdout() {
    // `| head -c 0` style consumers close the pipe early; we must not panic
    let child = demo_command()
        .stdout(std::process::Stdio::null())
        .status()
        .unwrap();
    assert!(child.success());
}
