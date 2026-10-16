//! Builds the project website into a directory (default `target/site`).
//!
//! ```text
//! cargo build --release --example web --target wasm32-unknown-unknown
//! cargo run --example build_site -- [output-dir]
//! ```
//!
//! The page is generated from Rust so the repository stays 100% Rust: the
//! colours and palette names come straight from `Colour`, and the compiled
//! `web` example (the Snake game) is copied alongside as `game.wasm`. The
//! result is deployed to GitHub Pages by `.github/workflows/pages.yml`.

use std::fs;
use std::path::{Path, PathBuf};

use terminal_graphics::Colour;

const SITE_URL: &str = "https://calum.github.io/terminal_graphics_api/";
const REPO_URL: &str = "https://github.com/calum/terminal_graphics_api";
const WASM_PATH: &str = "target/wasm32-unknown-unknown/release/examples/web.wasm";

const PAGE: &str = r####"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>terminal_graphics - a zero dependency graphics API for the terminal</title>
<meta name="description" content="A zero dependency, 100% Rust graphics API for the terminal. Play a Snake game built with it, right in your browser.">
<link rel="canonical" href="{{SITE_URL}}">
<style>
{{CSS}}
</style>
</head>
<body>
<div class="crt">
<main>

<p class="dim">TERMINAL GRAPHICS BBS v{{VERSION}} - CONNECTED 64000 BAUD</p>
<h1 aria-label="TERMINAL GRAPHICS">{{TITLE}}</h1>
<p class="subtitle">zero dependencies | 100% Rust | 16 ANSI colours</p>
<p class="nav">
<a href="{{REPO_URL}}">[ GitHub ]</a>
<a href="https://crates.io/crates/terminal_graphics">[ crates.io ]</a>
<a href="https://docs.rs/terminal_graphics">[ docs.rs ]</a>
</p>

<pre class="prompt"><span class="c10">$</span> cargo add terminal_graphics<span class="cursor">_</span></pre>

<section>
<h2>&gt; PLAY SNAKE</h2>
<p>Everything on this screen is drawn by the library: each frame a Rust
program fills a <code>Display</code> with <code>Container</code>,
<code>Rect</code>, <code>Text</code> and <code>Graphics</code>, and this page
only paints the cells. Click the screen, then use the arrow keys or WASD.
ENTER starts.</p>
<div class="screen">
<canvas id="screen" tabindex="0" width="728" height="624" aria-label="Snake game, built with terminal_graphics"></canvas>
<p id="status" class="dim">loading game.wasm ...</p>
</div>
<p class="pad" aria-label="on-screen controls">
<button data-key="3">&lt;</button>
<button data-key="1">^</button>
<button data-key="2">v</button>
<button data-key="4">&gt;</button>
<button data-key="5">ENTER</button>
</p>
</section>

<section>
<h2>&gt; WHAT IT IS GOOD AT</h2>
<pre class="box">
+--------------------------------------------------------------+
| A Display is a grid of Pixels: a char, a fg and a bg colour. |
| Draw into it, then print it. Output is plain ANSI codes.     |
+--------------------------------------------------------------+
</pre>
<ul>
<li><b class="c10">No dependencies.</b> The crate and its tests are 100% Rust, nothing else is pulled in.</li>
<li><b class="c11">Small API.</b> <code>Display</code>, <code>Graphic</code>, <code>Graphics</code>, <code>Container</code>, <code>Text</code>, <code>Rect</code> and <code>Ball</code>. Implement <code>Graphic</code> to draw your own shapes.</li>
<li><b class="c12">Flicker free animation.</b> Each frame is redrawn in place. <code>Graphics</code> keeps named graphics and draws them in order.</li>
<li><b class="c13">Colour.</b> The 16 ANSI colours, plus <code>Colour::from_rgb</code> to pick the nearest one for gradients.</li>
<li><b class="c14">Safe by design.</b> Anything off screen is clipped, and nothing panics or hangs on bad coordinates.</li>
<li><b class="c15">Easy to test.</b> <code>Display::render</code> and <code>write_to</code> give you the output as a string or on any writer.</li>
</ul>
</section>

<section>
<h2>&gt; HELLO, TERMINAL</h2>
<pre class="code">{{CODE}}</pre>
<p>Run the full animated tour yourself with
<code>cargo run --example demo</code>.</p>
</section>

<section>
<h2>&gt; THE PALETTE</h2>
<div class="palette">{{PALETTE}}</div>
</section>

<section>
<h2>&gt; LINKS</h2>
<ul class="links">
<li><a href="{{REPO_URL}}">GitHub repository</a> - source, issues, changelog</li>
<li><a href="https://crates.io/crates/terminal_graphics">crates.io/crates/terminal_graphics</a></li>
<li><a href="https://docs.rs/terminal_graphics">docs.rs/terminal_graphics</a> - API documentation</li>
</ul>
</section>

<footer>
<p class="dim">terminal_graphics v{{VERSION}} - MIT OR Apache-2.0 - <a href="{{REPO_URL}}">view the source on GitHub</a></p>
<p class="dim">NO CARRIER<span class="cursor">_</span></p>
</footer>

</main>
</div>
<script>
{{JS}}
</script>
</body>
</html>
"####;

const CSS: &str = r####"
:root { {{VARS}} }
* { box-sizing: border-box; }
html { background: #000; }
body { margin: 0; background: #000; color: var(--c7); font: 16px/1.5 "Courier New", Courier, monospace; }
.crt { position: relative; min-height: 100vh; }
.crt::after { content: ""; position: fixed; inset: 0; pointer-events: none;
  background: repeating-linear-gradient(to bottom, rgba(0,0,0,0) 0, rgba(0,0,0,0) 2px, rgba(0,0,0,.28) 3px);
  box-shadow: inset 0 0 140px rgba(0,0,0,.8); }
main { max-width: 800px; margin: 0 auto; padding: 24px 16px 64px; text-shadow: 0 0 4px currentColor; }
a { color: var(--c14); }
a:hover { background: var(--c14); color: #000; text-shadow: none; }
h1 { margin: 8px 0 0; font-size: clamp(22px, 6vw, 44px); letter-spacing: .12em; line-height: 1.2; }
h1 span { animation: rainbow 1.4s steps(1) infinite; }
@keyframes rainbow { {{KEYFRAMES}} }
h2 { color: var(--c11); font-size: 18px; margin: 40px 0 8px; }
.subtitle { color: var(--c8); margin: 4px 0 12px; }
.dim { color: var(--c8); font-size: 13px; }
.nav a { margin-right: 12px; }
.prompt { background: #000; border: 1px solid var(--c8); padding: 8px 12px; overflow-x: auto; }
.cursor { animation: blink 1s steps(1) infinite; }
@keyframes blink { 50% { opacity: 0; } }
code { color: var(--c10); }
ul { padding-left: 20px; list-style: none; }
li::before { content: "> "; color: var(--c8); margin-left: -20px; }
.box, .code { color: var(--c10); overflow-x: auto; }
.code { background: #080808; border: 1px solid var(--c8); padding: 12px; color: var(--c7); font-size: 14px; }
.screen { border: 2px solid var(--c10); padding: 8px; background: #000; box-shadow: 0 0 24px rgba(85,255,85,.25); }
canvas { display: block; width: 100%; height: auto; image-rendering: pixelated; outline: none; cursor: pointer; }
canvas:focus { box-shadow: 0 0 0 2px var(--c11); }
#status { margin: 6px 0 0; }
.pad { display: flex; gap: 8px; justify-content: center; flex-wrap: wrap; }
button { font: inherit; background: #000; color: var(--c10); border: 1px solid var(--c10); padding: 6px 14px; cursor: pointer; }
button:hover, button:active { background: var(--c10); color: #000; }
.palette { display: grid; grid-template-columns: repeat(8, 1fr); gap: 4px; }
.palette div { padding: 14px 2px 4px; font-size: 11px; text-align: center; border: 1px solid var(--c8); text-shadow: none; }
{{CLASSES}}
@media (max-width: 560px) { .palette { grid-template-columns: repeat(4, 1fr); } }
@media (prefers-reduced-motion: reduce) { h1 span, .cursor { animation: none; } }
"####;

const JS: &str = r####"
(function () {
  var canvas = document.getElementById("screen");
  var ctx = canvas.getContext("2d");
  var status = document.getElementById("status");
  var CELL_W = 14, CELL_H = 24;
  var keys = {
    ArrowUp: 1, w: 1, W: 1, ArrowDown: 2, s: 2, S: 2, ArrowLeft: 3, a: 3, A: 3,
    ArrowRight: 4, d: 4, D: 4, Enter: 5, " ": 5
  };
  var game = null, colours = [], last = 0;

  function css(rgb) { return "#" + ("000000" + rgb.toString(16)).slice(-6); }

  function draw() {
    var w = game.width(), h = game.height();
    var cells = new Uint32Array(game.memory.buffer, game.frame_ptr(), w * h * 2);
    ctx.font = "bold 20px 'Courier New', monospace";
    ctx.textAlign = "center";
    ctx.textBaseline = "middle";
    for (var y = 0; y < h; y++) {
      for (var x = 0; x < w; x++) {
        var i = (y * w + x) * 2, ch = cells[i], colour = cells[i + 1];
        ctx.fillStyle = colours[(colour >> 8) & 255];
        ctx.fillRect(x * CELL_W, y * CELL_H, CELL_W, CELL_H);
        if (ch !== 32) {
          ctx.fillStyle = colours[colour & 255];
          ctx.fillText(String.fromCodePoint(ch), x * CELL_W + CELL_W / 2, y * CELL_H + CELL_H / 2 + 1);
        }
      }
    }
  }

  function frame(now) {
    game.tick(Math.round(now - last));
    last = now;
    draw();
    requestAnimationFrame(frame);
  }

  canvas.addEventListener("click", function () { canvas.focus({ preventScroll: true }); });
  window.addEventListener("keydown", function (event) {
    if (!game || event.ctrlKey || event.metaKey || event.altKey) return;
    var code = keys[event.key];
    if (!code) return;
    // only take the keys while the screen has focus, so the page still scrolls
    if (document.activeElement !== canvas) return;
    event.preventDefault();
    game.key(code);
  });
  Array.prototype.forEach.call(document.querySelectorAll("button[data-key]"), function (button) {
    button.addEventListener("click", function () {
      if (game) game.key(Number(button.dataset.key));
    });
  });

  fetch("game.wasm")
    .then(function (response) { return response.arrayBuffer(); })
    .then(function (bytes) { return WebAssembly.instantiate(bytes, {}); })
    .then(function (result) {
      game = result.instance.exports;
      for (var i = 0; i < 16; i++) colours.push(css(game.palette(i)));
      game.init((Math.random() * 4294967295) >>> 0);
      status.textContent = "click the screen, then play with arrows / WASD. ENTER to start.";
      requestAnimationFrame(function (now) { last = now; frame(now); });
    })
    .catch(function (error) {
      status.textContent = "could not start the game: " + error;
    });
})();
"####;

const CODE: &str = r#"use terminal_graphics::{Colour, Container, Display, Graphic, Text};

fn main() -> std::io::Result<()> {
    // A 40 column by 10 row display.
    let mut display = Display::new(40, 10);
    display.reserve_space()?;

    // A bordered box holding some text.
    let container = Container::new(5, 2, 24, 5, Colour::BrightGreen, Colour::Blue)
        .with_content(Text::new(1, 1, "Hello, terminal!", Colour::BrightYellow, Colour::Blue));

    container.draw(&mut display);
    display.print()
}"#;

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// The page title with every letter in its own span, so each one can cycle
/// through the bright colours.
fn rainbow_title(text: &str) -> String {
    text.chars()
        .enumerate()
        .map(|(i, letter)| {
            if letter == ' ' {
                " ".to_string()
            } else {
                format!(
                    "<span style=\"animation-delay:-{}s\">{letter}</span>",
                    i as f32 * 0.2
                )
            }
        })
        .collect()
}

fn hex((r, g, b): (u8, u8, u8)) -> String {
    format!("#{r:02x}{g:02x}{b:02x}")
}

fn css_variables() -> String {
    Colour::ALL
        .iter()
        .enumerate()
        .map(|(i, colour)| format!("--c{i}:{};", hex(colour.rgb())))
        .collect()
}

fn colour_classes() -> String {
    (0..Colour::ALL.len())
        .map(|i| format!(".c{i}{{color:var(--c{i});}}"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Cycles `color` through the 7 bright colours (the last 7 in `Colour::ALL`).
fn rainbow_keyframes() -> String {
    let bright = &Colour::ALL[9..];
    bright
        .iter()
        .enumerate()
        .map(|(i, colour)| {
            let percent = i as f32 * 100.0 / bright.len() as f32;
            format!("{percent:.1}% {{ color: {}; }}", hex(colour.rgb()))
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn palette_swatches() -> String {
    Colour::ALL
        .iter()
        .enumerate()
        .map(|(i, colour)| {
            let (r, g, b) = colour.rgb();
            let light = (r as u32 * 299 + g as u32 * 587 + b as u32 * 114) / 1000 > 120;
            format!(
                "<div style=\"background:var(--c{i});color:{}\">{}</div>",
                if light { "#000" } else { "#fff" },
                colour.name()
            )
        })
        .collect()
}

fn build_page() -> String {
    let css = CSS
        .replace("{{VARS}}", &css_variables())
        .replace("{{KEYFRAMES}}", &rainbow_keyframes())
        .replace("{{CLASSES}}", &colour_classes());
    PAGE.replace("{{CSS}}", &css)
        .replace("{{JS}}", JS)
        .replace("{{TITLE}}", &rainbow_title("TERMINAL GRAPHICS"))
        .replace("{{CODE}}", &escape(CODE))
        .replace("{{PALETTE}}", &palette_swatches())
        .replace("{{VERSION}}", env!("CARGO_PKG_VERSION"))
        .replace("{{SITE_URL}}", SITE_URL)
        .replace("{{REPO_URL}}", REPO_URL)
}

/// Writes the site into `out`. Returns false if the compiled game was missing.
fn build_site(out: &Path, wasm: &Path) -> std::io::Result<bool> {
    fs::create_dir_all(out)?;
    fs::write(out.join("index.html"), build_page())?;
    fs::write(out.join(".nojekyll"), "")?;
    if wasm.exists() {
        fs::copy(wasm, out.join("game.wasm"))?;
        Ok(true)
    } else {
        Ok(false)
    }
}

fn main() -> std::io::Result<()> {
    let out = std::env::args()
        .nth(1)
        .map_or_else(|| PathBuf::from("target/site"), PathBuf::from);
    let found = build_site(&out, Path::new(WASM_PATH))?;
    println!("site written to {}", out.display());
    if !found {
        eprintln!(
            "warning: {WASM_PATH} not found, the game will not load. Build it with:\n  \
             cargo build --release --example web --target wasm32-unknown-unknown"
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_placeholder_is_filled_in() {
        let page = build_page();
        assert!(!page.contains("{{"), "left over placeholder in the page");
    }

    #[test]
    fn the_page_links_back_to_the_repo_and_to_the_docs() {
        let page = build_page();
        assert!(page.contains(&format!("href=\"{REPO_URL}\"")));
        assert!(page.contains("https://crates.io/crates/terminal_graphics"));
        assert!(page.contains("https://docs.rs/terminal_graphics"));
    }

    #[test]
    fn the_page_has_the_game_screen_and_loads_the_wasm() {
        let page = build_page();
        assert!(page.contains("<canvas id=\"screen\""));
        assert!(page.contains("fetch(\"game.wasm\")"));
    }

    #[test]
    fn all_sixteen_colours_are_in_the_palette_with_their_names() {
        let page = build_page();
        for colour in Colour::ALL {
            assert!(page.contains(&format!(">{}</div>", colour.name())));
        }
        assert_eq!(css_variables().matches("--c").count(), 16);
    }

    #[test]
    fn code_samples_are_escaped() {
        assert_eq!(escape("a<b>&"), "a&lt;b&gt;&amp;");
        assert!(!build_page().contains("-> std::io::Result"));
    }

    #[test]
    fn the_site_is_written_with_and_without_the_game() {
        let dir = std::env::temp_dir().join(format!("tg_site_{}", std::process::id()));
        let wasm = dir.join("fake.wasm");
        fs::create_dir_all(&dir).unwrap();

        let out = dir.join("out");
        assert!(!build_site(&out, &wasm).unwrap());
        assert!(out.join("index.html").exists());
        assert!(!out.join("game.wasm").exists());

        fs::write(&wasm, b"\0asm").unwrap();
        assert!(build_site(&out, &wasm).unwrap());
        assert_eq!(fs::read(out.join("game.wasm")).unwrap(), b"\0asm");

        fs::remove_dir_all(&dir).unwrap();
    }
}
