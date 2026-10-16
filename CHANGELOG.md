# Changelog

## 1.0.0

A redesign for consistency, safety and ease of use. This release has breaking
changes; the migration table below covers every one.

### Breaking changes

**Paths.** `Colour`, `Pixel` and `Display` live at the crate root only. The
`terminal_graphics::display::*` modules are gone (`display::display::Display`
is now `Display`). `Graphic`, `Graphics`, `Container` and `Text` are exported at
the root too, and `terminal_graphics::prelude::*` imports everything.

**Naming.** Methods follow Rust conventions (no `get_` prefix) and read
consistently across types.

**Behaviour.**
- `Container` owns its content (a `Box<dyn Graphic>` inside), so it has no
  lifetime parameter, and moving the container moves its content.
- A `Container` of `width` by `height` now covers exactly that many cells,
  border included (before, the border sat one cell *outside* the size).
- Content positions are relative to the inside of the container, one cell in
  from the border.
- `Graphics::draw` now behaves like `Graphic::draw`: it only paints. The old
  clear-paint-print behaviour is `Graphics::show`.
- `Display::print`, `Display::reserve_space` and `Graphics::show` return
  `io::Result<()>` instead of silently ignoring errors.
- Unknown colour names are an error (`FromStr`), not a silent `White`.
- `Display::get_mut_pixel` (which panicked) is removed; use `pixel_mut`.

| 0.x | 1.0 |
| --- | --- |
| `display::display::Display`, `display::colour::Colour`, `display::pixel::Pixel` | `Display`, `Colour`, `Pixel` |
| `graphics::Graphic` | `Graphic` (also `graphics::Graphic`) |
| `Colour::of("Red")` | `"Red".parse::<Colour>()` (returns a `Result`) |
| `colour.to_str()` | `colour.name()` or `colour.to_string()` |
| `Colour::variants()` | `Colour::ALL` |
| `colour.get_codes()` (`(u32, u32)`) | `colour.ansi_codes()` (`(u8, u8)`) |
| `colour.get_rbg()` | `colour.rgb()` |
| `Pixel::generate_pixels`, `Pixels` | removed (internal) |
| `display.get_mut_pixel(x, y)` | `display.pixel_mut(x, y)` (returns `Option`) |
| `display.print()` | `display.print()?` (returns `io::Result`) |
| `Text::from(x, y, s.to_string(), fg, bg)`, `Text::new()` | `Text::new(x, y, "s", fg, bg)` |
| `Text::default()`, `Container::default()` | removed |
| `Container::from(x, y, w, h, &mut g, border, bg)` | `Container::new(x, y, w + 1, h + 1, border, bg).with_content(g)`, with `g`'s position moved one cell up and left (it is now relative to the inside) |
| `container.set_content(&mut g)` | `container.set_content(g)` |
| `graphics.add_named("a", Box::new(g))` | `graphics.add("a", g)` |
| `graphics.get_named("a")`, `get_mut_named("a")` | `graphics.get("a")`, `get_mut("a")` (return `&dyn Graphic`) |
| `graphics.remove_named("a")` | `graphics.remove("a")` (returns the graphic) |
| `graphics.move_named_graphic("a", dx, dy)` | `graphics.move_by("a", dx, dy)` |
| `graphics.draw(&mut display)` (clear, paint, print) | `graphics.show(&mut display)?` |
| `graphics.render_to(&mut display)` (clear, paint) | `display.clear(); graphics.draw(&mut display)` |
| `g.get_position()` | `g.position()` |
| `g.get_area()` | `g.area()` |
| `g.check_collision(x, y)` | `g.contains(x, y)` |
| `g.move_position(dx, dy)` | `g.move_by(dx, dy)` |
| `g.update_speeds(fx, fy)` | `g.scale_speed(fx, fy)` |
| `g.update_position()` | `g.update()` |

### Added

- `terminal_graphics::prelude`, and root exports for `Graphic`, `Graphics`,
  `Pixel` and `ParseColourError`.
- `Display::pixel` (read a pixel), `width`, `height` and `reserve_space`
  (make room before the first frame).
- `Colour` implements `Display`, `FromStr`, `Eq` and `Hash`.
- `Graphics::update`, `len`, `is_empty` and `contains`; `add` returns `&mut Self`
  so calls chain, and takes any `impl Graphic`, so there is no more `Box::new`.
- `Text`, `Rect` and `Ball` accessors and setters for their colours, text, size
  and speed. `Container::content`, `content_mut`, `take_content`, `size` and
  `set_border`.
- `Clone`, `Debug` and `PartialEq` on `Display`, `Pixel`, `Text`, `Rect` and `Ball`.
- A `CHANGELOG`, the README as a tested doc, and a CI docs build.

### Fixed

- Drawing at extreme coordinates (near `isize::MAX` / `MIN`) could overflow and
  panic, and drawing huge off-screen shapes looped over every cell. Everything
  now clips to the display and uses saturating arithmetic.

### Other

- Minimum supported Rust version is 1.70.
