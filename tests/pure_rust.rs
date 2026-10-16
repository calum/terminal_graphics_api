//! Repo hygiene: the project stays 100% Rust with zero dependencies.

use std::fs;
use std::path::Path;

/// Extensions allowed in the repo. Anything else (.py, .js, .sh, .c, ...) fails the build.
const ALLOWED_EXTENSIONS: &[&str] = &["rs", "toml", "md", "yml", "png", "gif", "lock"];
const ALLOWED_FILE_NAMES: &[&str] = &[".gitignore", ".gitattributes", "LICENSE", "CLAUDE.md"];
const IGNORED_DIRS: &[&str] = &["target", ".git"];

fn walk(dir: &Path, found: &mut Vec<String>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if path.is_dir() {
            if !IGNORED_DIRS.contains(&name.as_str()) {
                walk(&path, found);
            }
        } else {
            found.push(
                path.to_string_lossy()
                    .replace(std::path::MAIN_SEPARATOR, "/"),
            );
        }
    }
}

#[test]
fn only_rust_and_allowed_config_files_are_in_the_repo() {
    let mut files = Vec::new();
    walk(Path::new(env!("CARGO_MANIFEST_DIR")), &mut files);

    let offenders: Vec<_> = files
        .iter()
        .filter(|f| {
            let path = Path::new(f);
            let name = path.file_name().unwrap().to_string_lossy();
            let ext = path.extension().map(|e| e.to_string_lossy().to_string());
            !ALLOWED_FILE_NAMES.contains(&name.as_ref())
                && !ext.is_some_and(|e| ALLOWED_EXTENSIONS.contains(&e.as_str()))
        })
        .collect();

    assert!(
        offenders.is_empty(),
        "non-Rust files found: {:?}",
        offenders
    );
}

#[test]
fn there_is_no_build_script() {
    assert!(!Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("build.rs")
        .exists());
}

#[test]
fn manifest_declares_no_dependencies() {
    let manifest =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml")).unwrap();
    for line in manifest.lines().map(str::trim) {
        assert!(
            !line.starts_with("[dependencies")
                && !line.starts_with("[dev-dependencies")
                && !line.starts_with("[build-dependencies"),
            "unexpected dependency section: {}",
            line
        );
    }
}
