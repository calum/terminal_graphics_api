//! The game that runs on the project website, built using only this library.
//!
//! This example is compiled to WebAssembly (`cargo build --release --example
//! web --target wasm32-unknown-unknown`). The page calls the exported
//! functions below once per animation frame and paints the `Display` it gets
//! back onto a canvas. All the drawing and game logic is plain Rust using
//! `terminal_graphics`; the page only forwards key presses and shows cells.
//!
//! The exports are:
//!
//! - `init(seed)`: start up with a random seed
//! - `key(code)`: 1 up, 2 down, 3 left, 4 right, 5 start / restart
//! - `tick(ms)`: advance the game by `ms` milliseconds and redraw
//! - `width()`, `height()`, `frame_ptr()`: the frame, two `u32`s per cell:
//!   the character, then `foreground | background << 8` as palette indexes
//! - `palette(index)`: the `0xRRGGBB` value of a palette colour

use std::cell::RefCell;
use std::collections::VecDeque;

use terminal_graphics::prelude::*;

const WIDTH: u32 = 52;
const HEIGHT: u32 = 26;

// the playfield: each snake cell is two display columns wide so it looks square
const COLUMNS: isize = 24;
const ROWS: isize = 20;
const FIELD_X: isize = 1;
const FIELD_Y: isize = 3;

const FRAME_MS: u32 = 33;
const FIRST_STEP_MS: u32 = 140;
const FASTEST_STEP_MS: u32 = 60;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Key {
    Move(Direction),
    Start,
}

impl Key {
    fn from_code(code: u32) -> Option<Key> {
        match code {
            1 => Some(Key::Move(Direction::Up)),
            2 => Some(Key::Move(Direction::Down)),
            3 => Some(Key::Move(Direction::Left)),
            4 => Some(Key::Move(Direction::Right)),
            5 => Some(Key::Start),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scene {
    Title,
    Playing,
    GameOver,
}

struct Game {
    scene: Scene,
    /// Cells of the snake, head first.
    snake: VecDeque<(isize, isize)>,
    heading: Direction,
    next_heading: Direction,
    food: (isize, isize),
    score: u32,
    best: u32,
    seed: u32,
    animation_ms: u32,
    step_ms: u32,
}

impl Game {
    fn new(seed: u32) -> Game {
        let mut game = Game {
            scene: Scene::Title,
            snake: VecDeque::new(),
            heading: Direction::Right,
            next_heading: Direction::Right,
            food: (0, 0),
            score: 0,
            best: 0,
            seed: seed | 1,
            animation_ms: 0,
            step_ms: 0,
        };
        game.reset();
        game
    }

    fn reset(&mut self) {
        let (x, y) = (COLUMNS / 2, ROWS / 2);
        self.snake = (0..3).map(|i| (x - i, y)).collect();
        self.heading = Direction::Right;
        self.next_heading = Direction::Right;
        self.score = 0;
        self.step_ms = 0;
        self.food = self.random_free_cell().unwrap_or((0, 0));
    }

    /// xorshift32: plenty random for choosing where the food goes.
    fn random(&mut self) -> u32 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 17;
        self.seed ^= self.seed << 5;
        self.seed
    }

    fn random_free_cell(&mut self) -> Option<(isize, isize)> {
        let free: Vec<(isize, isize)> = (0..ROWS)
            .flat_map(|y| (0..COLUMNS).map(move |x| (x, y)))
            .filter(|cell| !self.snake.contains(cell))
            .collect();
        if free.is_empty() {
            return None;
        }
        let index = self.random() as usize % free.len();
        Some(free[index])
    }

    fn press(&mut self, key: Key) {
        match (key, self.scene) {
            (Key::Start, Scene::Title | Scene::GameOver) => {
                self.reset();
                self.scene = Scene::Playing;
            }
            (Key::Move(direction), Scene::Playing) if direction != self.heading.opposite() => {
                self.next_heading = direction;
            }
            _ => {}
        }
    }

    fn tick(&mut self, ms: u32) {
        self.animation_ms = self.animation_ms.wrapping_add(ms);
        if self.scene != Scene::Playing {
            return;
        }
        self.step_ms += ms;
        let interval = FIRST_STEP_MS
            .saturating_sub(self.score * 4)
            .max(FASTEST_STEP_MS);
        while self.step_ms >= interval && self.scene == Scene::Playing {
            self.step_ms -= interval;
            self.step();
        }
    }

    fn step(&mut self) {
        self.heading = self.next_heading;
        let (dx, dy) = self.heading.delta();
        let (hx, hy) = self.snake[0];
        let head = (hx + dx, hy + dy);

        let off_field = head.0 < 0 || head.0 >= COLUMNS || head.1 < 0 || head.1 >= ROWS;
        // the tail cell is about to move away, so the head may enter it
        let body = self.snake.len() - 1;
        let hits_self = self.snake.iter().take(body).any(|&cell| cell == head);
        if off_field || hits_self {
            self.scene = Scene::GameOver;
            self.best = self.best.max(self.score);
            return;
        }

        self.snake.push_front(head);
        if head == self.food {
            self.score += 1;
            match self.random_free_cell() {
                Some(cell) => self.food = cell,
                None => {
                    // the snake fills the field: that is a win
                    self.scene = Scene::GameOver;
                    self.best = self.best.max(self.score);
                }
            }
        } else {
            self.snake.pop_back();
        }
    }

    fn frame(&self) -> u32 {
        self.animation_ms / FRAME_MS
    }

    fn draw(&self, display: &mut Display) {
        display.clear();
        for y in 0..HEIGHT as isize {
            for x in 0..WIDTH as isize {
                display.set_pixel(x, y, ' ', Colour::White, Colour::Black);
            }
        }
        draw_title(display, self.frame());
        match self.scene {
            Scene::Title => self.draw_title_screen(display),
            Scene::Playing => self.draw_field(display),
            Scene::GameOver => {
                self.draw_field(display);
                self.draw_game_over(display);
            }
        }
    }

    fn draw_title_screen(&self, display: &mut Display) {
        draw_river(display, self.frame());

        // a few balls bouncing around, positions worked out from the frame
        let mut graphics = Graphics::new();
        let colours = [
            Colour::BrightYellow,
            Colour::BrightMagenta,
            Colour::BrightCyan,
        ];
        for (i, colour) in colours.into_iter().enumerate() {
            let t = self.frame() as isize * (i as isize + 2) / 2 + i as isize * 17;
            let x = 1 + bounce(t, WIDTH as isize - 4);
            let y = 10 + bounce(t * 2 / 3 + 5, 6);
            graphics.add(
                format!("ball{i}"),
                Ball::new(x as f32, y as f32, 0.0, 0.0, colour),
            );
        }
        graphics.draw(display);

        let lines = [
            "A SNAKE GAME BUILT ONLY WITH THE LIBRARY",
            "",
            "arrows or WASD to steer",
            "eat the food, don't hit anything",
            "",
            "press ENTER to play",
        ];
        let panel = Container::new(6, 16, 40, 9, Colour::BrightGreen, Colour::Black);
        display_lines(display, panel, &lines, Colour::BrightGreen);
    }

    fn draw_field(&self, display: &mut Display) {
        let mut hud = format!("SCORE {:03}   BEST {:03}", self.score, self.best);
        hud.truncate(WIDTH as usize);
        Text::new(2, 2, hud, Colour::BrightYellow, Colour::Black).draw(display);

        let width = COLUMNS * 2 + 2;
        let height = ROWS + 2;
        let mut graphics = Graphics::new();
        graphics.add(
            "field",
            Container::new(
                FIELD_X,
                FIELD_Y,
                width,
                height,
                Colour::Green,
                Colour::Black,
            ),
        );

        let (fx, fy) = self.food;
        graphics.add(
            "food",
            Text::new(
                FIELD_X + 1 + fx * 2,
                FIELD_Y + 1 + fy,
                "()",
                Colour::BrightRed,
                Colour::Black,
            ),
        );

        let rainbow = &Colour::ALL[9..];
        for (i, &(x, y)) in self.snake.iter().enumerate() {
            // the head is white, the body walks round the bright colours
            let colour = if i == 0 {
                Colour::BrightWhite
            } else {
                rainbow[(i + self.frame() as usize / 3) % rainbow.len()]
            };
            graphics.add(
                format!("segment{i}"),
                Rect::new(FIELD_X + 1 + x * 2, FIELD_Y + 1 + y, 2, 1, colour),
            );
        }
        graphics.draw(display);
    }

    fn draw_game_over(&self, display: &mut Display) {
        let lines = [
            "GAME OVER",
            "",
            &format!("you ate {} things", self.score),
            "press ENTER to play again",
        ];
        let panel = Container::new(9, 10, 34, 7, Colour::BrightRed, Colour::Black);
        display_lines(display, panel, &lines, Colour::BrightWhite);
    }
}

/// Puts `lines` of text, one per row, inside `panel` and draws it.
fn display_lines(display: &mut Display, panel: Container, lines: &[&str], colour: Colour) {
    panel.draw(display);
    let (x, y) = panel.position();
    for (i, line) in lines.iter().enumerate() {
        Text::new(x + 2, y + 1 + i as isize, *line, colour, Colour::Black).draw(display);
    }
}

/// Bounces `t` back and forth between 0 and `max`.
fn bounce(t: isize, max: isize) -> isize {
    let period = (max * 2).max(1);
    let t = t.rem_euclid(period);
    if t > max {
        period - t
    } else {
        t
    }
}

/// The rainbow title, painted one letter at a time with `set_pixel`.
fn draw_title(display: &mut Display, frame: u32) {
    let title = "TERMINAL GRAPHICS";
    let rainbow = &Colour::ALL[9..];
    let start = (WIDTH as usize - title.len()) / 2;
    for (i, letter) in title.chars().enumerate() {
        let colour = rainbow[(i + frame as usize / 3) % rainbow.len()];
        display.set_pixel((start + i) as isize, 0, letter, colour, Colour::Black);
    }
}

/// The rainbow river: `Colour::from_rgb` mapped across the width.
fn draw_river(display: &mut Display, frame: u32) {
    let t = frame as f32 * 0.12;
    for row in 0..3 {
        for x in 0..WIDTH as isize {
            let hue = x as f32 * 0.16 - t + row as f32 * 0.7;
            let wave = |shift: f32| ((hue + shift).sin() * 127.0 + 128.0) as u8;
            let colour = Colour::from_rgb(wave(0.0), wave(2.1), wave(4.2));
            display.set_pixel(x, 3 + row, ' ', Colour::White, colour);
        }
    }
}

struct Machine {
    game: Game,
    display: Display,
    frame: Vec<u32>,
}

impl Machine {
    fn new(seed: u32) -> Machine {
        let mut machine = Machine {
            game: Game::new(seed),
            display: Display::new(WIDTH, HEIGHT),
            frame: vec![0; (WIDTH * HEIGHT * 2) as usize],
        };
        machine.redraw();
        machine
    }

    fn redraw(&mut self) {
        self.game.draw(&mut self.display);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let Some(pixel) = self.display.pixel(x as isize, y as isize) else {
                    continue;
                };
                let index = ((y * WIDTH + x) * 2) as usize;
                self.frame[index] = pixel.character() as u32;
                self.frame[index + 1] =
                    palette_index(pixel.colour()) | palette_index(pixel.background()) << 8;
            }
        }
    }
}

fn palette_index(colour: Colour) -> u32 {
    Colour::ALL.iter().position(|&c| c == colour).unwrap_or(0) as u32
}

thread_local! {
    static MACHINE: RefCell<Machine> = RefCell::new(Machine::new(1));
}

#[no_mangle]
pub extern "C" fn init(seed: u32) {
    MACHINE.with(|m| *m.borrow_mut() = Machine::new(seed));
}

#[no_mangle]
pub extern "C" fn key(code: u32) {
    if let Some(key) = Key::from_code(code) {
        MACHINE.with(|m| m.borrow_mut().game.press(key));
    }
}

#[no_mangle]
pub extern "C" fn tick(ms: u32) {
    MACHINE.with(|m| {
        let mut machine = m.borrow_mut();
        machine.game.tick(ms.min(250));
        machine.redraw();
    });
}

#[no_mangle]
pub extern "C" fn width() -> u32 {
    WIDTH
}

#[no_mangle]
pub extern "C" fn height() -> u32 {
    HEIGHT
}

#[no_mangle]
pub extern "C" fn frame_ptr() -> usize {
    MACHINE.with(|m| m.borrow().frame.as_ptr() as usize)
}

#[no_mangle]
pub extern "C" fn palette(index: u32) -> u32 {
    let colour = Colour::ALL[index as usize % Colour::ALL.len()];
    let (r, g, b) = colour.rgb();
    (r as u32) << 16 | (g as u32) << 8 | b as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn playing() -> Game {
        let mut game = Game::new(7);
        game.press(Key::Start);
        game
    }

    fn row(display: &Display, y: isize) -> String {
        (0..WIDTH as isize)
            .map(|x| display.pixel(x, y).unwrap().character())
            .collect()
    }

    #[test]
    fn starts_on_the_title_screen_and_enter_starts_the_game() {
        let mut game = Game::new(7);
        assert_eq!(game.scene, Scene::Title);
        game.press(Key::Start);
        assert_eq!(game.scene, Scene::Playing);
    }

    #[test]
    fn the_snake_moves_one_cell_each_step() {
        let mut game = playing();
        let head = game.snake[0];
        let length = game.snake.len();
        game.food = (0, 0);
        game.step();
        assert_eq!(game.snake[0], (head.0 + 1, head.1));
        assert_eq!(game.snake.len(), length);
    }

    #[test]
    fn it_cannot_reverse_into_itself() {
        let mut game = playing();
        game.press(Key::Move(Direction::Left));
        game.step();
        assert_eq!(game.heading, Direction::Right);
        assert_eq!(game.scene, Scene::Playing);
    }

    #[test]
    fn eating_food_grows_the_snake_and_scores() {
        let mut game = playing();
        let head = game.snake[0];
        game.food = (head.0 + 1, head.1);
        let length = game.snake.len();
        game.step();
        assert_eq!(game.score, 1);
        assert_eq!(game.snake.len(), length + 1);
        assert!(!game.snake.contains(&game.food));
    }

    #[test]
    fn hitting_the_wall_ends_the_game_and_records_the_best_score() {
        let mut game = playing();
        game.score = 3;
        game.food = (0, 0);
        for _ in 0..COLUMNS {
            game.step();
        }
        assert_eq!(game.scene, Scene::GameOver);
        assert_eq!(game.best, 3);
    }

    #[test]
    fn hitting_itself_ends_the_game() {
        let mut game = playing();
        game.snake = (0..6).map(|i| (5, 5 + i)).collect();
        game.heading = Direction::Up;
        game.next_heading = Direction::Up;
        game.food = (0, 0);
        game.press(Key::Move(Direction::Right));
        game.step();
        game.press(Key::Move(Direction::Down));
        game.step();
        game.press(Key::Move(Direction::Left));
        game.step();
        assert_eq!(game.scene, Scene::GameOver);
    }

    #[test]
    fn the_head_may_follow_the_tail_into_its_old_cell() {
        let mut game = playing();
        game.snake = VecDeque::from([(1, 1), (1, 2), (2, 2), (2, 1)]);
        game.heading = Direction::Up;
        game.next_heading = Direction::Right;
        game.food = (0, 0);
        game.step();
        assert_eq!(game.scene, Scene::Playing);
    }

    #[test]
    fn game_over_enter_restarts_with_a_fresh_snake() {
        let mut game = playing();
        game.scene = Scene::GameOver;
        game.score = 4;
        game.press(Key::Start);
        assert_eq!(game.scene, Scene::Playing);
        assert_eq!(game.score, 0);
        assert_eq!(game.snake.len(), 3);
    }

    #[test]
    fn time_only_advances_the_snake_while_playing() {
        let mut game = Game::new(7);
        let head = game.snake[0];
        game.tick(1000);
        assert_eq!(game.snake[0], head);

        game.press(Key::Start);
        game.food = (0, 0);
        game.tick(FIRST_STEP_MS);
        assert_ne!(game.snake[0], head);
    }

    #[test]
    fn food_never_lands_on_the_snake() {
        let mut game = playing();
        for _ in 0..200 {
            let cell = game.random_free_cell().unwrap();
            assert!(!game.snake.contains(&cell));
        }
    }

    #[test]
    fn a_full_field_is_a_win_not_a_hang() {
        let mut game = playing();
        game.snake = (0..ROWS * COLUMNS - 1)
            .map(|i| (i % COLUMNS, i / COLUMNS))
            .collect();
        assert!(game.random_free_cell().is_some());
        game.snake.push_back((COLUMNS - 1, ROWS - 1));
        assert!(game.random_free_cell().is_none());
    }

    #[test]
    fn every_scene_draws_the_rainbow_title() {
        let mut display = Display::new(WIDTH, HEIGHT);
        for scene in [Scene::Title, Scene::Playing, Scene::GameOver] {
            let mut game = playing();
            game.scene = scene;
            game.draw(&mut display);
            assert!(row(&display, 0).contains("TERMINAL GRAPHICS"));
        }
    }

    #[test]
    fn the_title_screen_invites_you_to_play() {
        let mut display = Display::new(WIDTH, HEIGHT);
        Game::new(7).draw(&mut display);
        let screen: String = (0..HEIGHT as isize).map(|y| row(&display, y)).collect();
        assert!(screen.contains("press ENTER to play"));
    }

    #[test]
    fn bounce_stays_in_range() {
        for t in -50..200 {
            assert!((0..=10).contains(&bounce(t, 10)));
        }
    }

    #[test]
    fn the_exports_describe_a_full_frame() {
        init(42);
        key(5);
        tick(FRAME_MS);
        assert_eq!((width(), height()), (WIDTH, HEIGHT));
        MACHINE.with(|m| {
            let machine = m.borrow();
            assert_eq!(machine.frame.len(), (WIDTH * HEIGHT * 2) as usize);
            assert_eq!(machine.frame[0], ' ' as u32);
        });
        assert_eq!(palette(0), 0x000000);
    }
}
