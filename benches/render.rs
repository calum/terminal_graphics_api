//! Rendering benchmarks. Run with `cargo bench`.
//!
//! Uses only `std` (no benchmark crate) so the crate keeps zero dependencies.
//! For each scenario it reports the median time to render one frame and the
//! number of bytes of ANSI output that frame produces. Bytes matter as much as
//! time: a real terminal has to receive and parse every one of them.

use std::hint::black_box;
use std::time::{Duration, Instant};
use terminal_graphics::shapes::{Ball, Rect};
use terminal_graphics::{Colour, Container, Display, Graphic, Graphics, Text};

const SAMPLES: usize = 15;
const SAMPLE_TIME: Duration = Duration::from_millis(100);

/// Median time per call of `f`, in nanoseconds.
fn time_per_call(mut f: impl FnMut()) -> f64 {
    // calibrate how many calls fill one sample
    let mut calls = 1u64;
    loop {
        let start = Instant::now();
        for _ in 0..calls {
            f();
        }
        if start.elapsed() >= SAMPLE_TIME / 10 {
            break;
        }
        calls *= 2;
    }
    let calls = calls * 10;

    let mut samples: Vec<f64> = (0..SAMPLES)
        .map(|_| {
            let start = Instant::now();
            for _ in 0..calls {
                f();
            }
            start.elapsed().as_nanos() as f64 / calls as f64
        })
        .collect();
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    samples[SAMPLES / 2]
}

fn fill_uniform(d: &mut Display) {
    let (w, h) = d.size();
    Rect::new(0, 0, w as isize, h as isize, Colour::Blue).draw(d);
}

/// Every cell differs from its neighbours: the worst case for colour reuse.
fn fill_gradient(d: &mut Display) {
    let (w, h) = d.size();
    for y in 0..h {
        for x in 0..w {
            let c = Colour::from_rgb((x * 255 / w.max(1)) as u8, (y * 255 / h.max(1)) as u8, 128);
            d.set_pixel(x as isize, y as isize, '#', c, Colour::Black);
        }
    }
}

/// A typical game or UI frame: background, a bordered box with text, balls.
fn draw_scene(d: &mut Display, graphics: &Graphics) {
    d.clear();
    graphics.draw(d);
}

fn scene(width: isize, height: isize) -> Graphics {
    let mut g = Graphics::new();
    g.add("sky", Rect::new(0, 0, width, height, Colour::Blue))
        .add(
            "box",
            Container::new(
                2,
                1,
                width / 2,
                height / 2,
                Colour::BrightGreen,
                Colour::Black,
            )
            .with_content(Text::new(
                1,
                1,
                "Hello, terminal!",
                Colour::White,
                Colour::Black,
            )),
        );
    for i in 0..10 {
        g.add(
            format!("ball{i}"),
            Ball::new(i as f32 * 3.0, i as f32, 1.0, 0.5, Colour::BrightYellow),
        );
    }
    g
}

fn report(name: &str, size: (u32, u32), ns: f64, bytes: usize) {
    println!(
        "{name:<28} {:>4}x{:<3} {:>10.1} us/frame {:>9} bytes/frame",
        size.0,
        size.1,
        ns / 1000.0,
        bytes
    );
}

fn main() {
    println!("terminal_graphics render benchmarks (median of {SAMPLES} samples)\n");
    for &(w, h) in &[(80u32, 24u32), (200, 60)] {
        let mut blank = Display::new(w, h);
        blank.clear();
        let ns = time_per_call(|| {
            black_box(black_box(&blank).render());
        });
        report("render: blank", (w, h), ns, blank.render().len());

        let mut uniform = Display::new(w, h);
        fill_uniform(&mut uniform);
        let ns = time_per_call(|| {
            black_box(black_box(&uniform).render());
        });
        report("render: one colour", (w, h), ns, uniform.render().len());

        let mut gradient = Display::new(w, h);
        fill_gradient(&mut gradient);
        let ns = time_per_call(|| {
            black_box(black_box(&gradient).render());
        });
        report("render: gradient", (w, h), ns, gradient.render().len());

        let graphics = scene(w as isize, h as isize);
        let mut display = Display::new(w, h);
        let ns = time_per_call(|| {
            draw_scene(&mut display, black_box(&graphics));
        });
        report("draw: scene", (w, h), ns, 0);

        let mut sink: Vec<u8> = Vec::with_capacity(1 << 20);
        let mut frame = Display::new(w, h);
        let mut graphics = scene(w as isize, h as isize);
        let ns = time_per_call(|| {
            graphics.update();
            draw_scene(&mut frame, &graphics);
            sink.clear();
            frame.write_to(&mut sink).unwrap();
            black_box(&sink);
        });
        report("frame: update+draw+write", (w, h), ns, sink.len());
        println!();
    }
}
