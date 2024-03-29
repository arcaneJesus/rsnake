use raylib::prelude::{*, KeyboardKey::{KEY_UP, KEY_DOWN, KEY_LEFT, KEY_RIGHT}};
use std::collections::VecDeque;
use std::env::current_dir;
use raylib::consts::guiIconName::RICON_FOLDER_OPEN;

// -------- CONSTANTS ---------
// ---- SIZE ----
const WINDOW_SIZE: i32 = 800;
const GRID_SIZE: i32 = 20;
const FONT_SIZE: i32 = 40;
// ---- STRING ----
const START_STRING: &str = "PRESS ANY KEY TO BEGIN";
const END_STRING: &str = "GAME OVER\nPRESS ANY KEY TO PLAY AGAIN";
// ---- COLOR ----
const BACKGROUND_COLOR: Color = Color::BLACK;
const BODY_COLOR: Color = Color::DARKGREEN;
const HEAD_COLOR: Color = Color::GREEN;
const TEXT_COLOR: Color = Color::LIGHTGRAY;
const APPLE_COLOR: Color = Color::RED;
// ---- MISC ----
const TICK_DELAY: i32 = 10;
// Lower tick delay increases game speed
const GRID_PX: i32 = WINDOW_SIZE / GRID_SIZE;

// --------- TYPE ALIASES ---------

type Body = VecDeque<Point>;

type Apple = Point;

// -------- ENUMS -----------

enum GameState {
    Start,
    Run,
    End,
}

// ---------- STRUCTS ---------

#[derive(PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct Snake {
    body: Body,
    head: Point,
}

struct Game {
    snake: Snake,
    apple: Apple,
    tick: i32,
    game_state: GameState,
    current_dir: KeyboardKey,
    score: i32,
}

// ---------- IMPLEMENTATIONS -----------

impl Apple {
    // There's probably a better way to do this but I'm lazy.
    fn new(snake: &Snake) -> Self {
        // Generate first location
        let mut apple: Self = Self {
            x: get_random_value(0, GRID_SIZE - 1),
            y: get_random_value(0, GRID_SIZE - 1),
        };
        // Check if location is inside snake
        // If true generate new location
        while snake.body.contains(&apple) || &apple == &snake.head {
            apple = Self {
                x: get_random_value(0, GRID_SIZE - 1),
                y: get_random_value(0, GRID_SIZE - 1),
            };
        }
        apple
    }
}

impl Snake {
    fn new(len: usize) -> Self {
        // Place head at center of grid
        let head = Point {
            x: GRID_SIZE / 2,
            y: GRID_SIZE / 2,
        };
        // Create empty body
        let mut body = Body::new();
        // Fill with segments each one lower than the previous
        for i in 1..len {
            body.push_back(Point {
                x: head.x,
                y: head.y + i as i32,
            })
        }
        Self {
            head,
            body,
        }
    }

    fn mov(&mut self, dir: KeyboardKey) -> bool {
        // Add head location to front of body
        self.body.push_front(self.head);
        // Check movement direction and wall collision
        match dir {
            KEY_UP => {
                self.head.y -= 1;
                if self.head.y < 0 {
                    return false;
                }
            }
            KEY_DOWN => {
                self.head.y += 1;
                if self.head.y > GRID_SIZE - 1 {
                    return false;
                }
            }
            KEY_LEFT => {
                self.head.x -= 1;
                if self.head.x < 0 {
                    return false;
                }
            }
            KEY_RIGHT => {
                self.head.x += 1;
                if self.head.x > GRID_SIZE - 1 {
                    return false;
                }
            }
            _ => {}
        }
        // Check body collision
        !self.body.contains(&self.head)
    }
}

impl Clone for Snake {
    fn clone(&self) -> Self {
        Self {
            head: self.head,
            body: self.body.clone(),
        }
    }
}

impl Game {
    fn new() -> Self {
        // Bind snake early so apple constructor can reference it
        let snake = Snake::new(3);
        Self {
            snake: snake.clone(),
            apple: Apple::new(&snake),
            tick: 0,
            game_state: GameState::Start,
            current_dir: KEY_UP,
            score: 0,
        }
    }

    // Perform a tick and check if its a action tick
    fn tick(&mut self) -> bool {
        // If tick equals tick delay reset tick
        // Or increment tick
        if self.tick == TICK_DELAY {
            self.tick = 0;
            true
        } else {
            self.tick += 1;
            false
        }
    }
}

// ---------- FUNCTIONS ----------

fn draw_grid_rect(draw_handle: &mut RaylibDrawHandle, point: &Point, color: Color) {
    draw_handle.draw_rectangle(
        GRID_PX * point.x,
        GRID_PX * point.y,
        GRID_PX,
        GRID_PX,
        color,
    );
}

fn main() {
    // ------- WINDOW INIT -------
    let (mut rl, thread) = init()
        .size(WINDOW_SIZE, WINDOW_SIZE)
        .title("rsnake")
        .build();
    rl.set_target_fps(60);

    let mut game = Game::new();

    // While window is open
    while !rl.window_should_close() {
        // -------- LOGIC ---------
        match game.game_state {
            GameState::Start => {
                // If a key is pressed start game
                if rl.get_key_pressed().is_some() {
                    game.game_state = GameState::Run;
                }
            }
            GameState::Run => {
                // Get last key pressed
                // If no key pressed use previous key
                game.current_dir = rl.get_key_pressed().unwrap_or(game.current_dir);
                // Do a game tick
                if game.tick() {
                    // On action tick move snake and check score
                    if !game.snake.mov(game.current_dir) {
                        game.game_state = GameState::End;
                    } else if game.snake.head == game.apple {
                        game.score += 1;
                        game.apple = Apple::new(&game.snake)
                    } else {
                        game.snake.body.pop_back();
                    }
                }
            }
            GameState::End => {
                // If a key is pressed restart game
                if rl.get_key_pressed().is_some() {
                    game = Game::new();
                    game.game_state = GameState::Run;
                }
            }
        }

        // -------- RENDERING ---------
        let mut rndr = rl.begin_drawing(&thread);
        // Background color
        rndr.clear_background(BACKGROUND_COLOR);
        // Draw head
        draw_grid_rect(&mut rndr, &game.snake.head, HEAD_COLOR);
        // Draw each body segment
        for seg in &game.snake.body {
            draw_grid_rect(&mut rndr, seg, BODY_COLOR);
        }
        // Draw apple
        draw_grid_rect(&mut rndr, &game.apple, APPLE_COLOR);
        // Draw score
        rndr.draw_text(&game.score.to_string(), 40, 40, FONT_SIZE * 2, TEXT_COLOR);
        // Draw start or end text
        match game.game_state {
            GameState::Start => {
                rndr.draw_text(START_STRING,
                               WINDOW_SIZE / 2 - measure_text(START_STRING, FONT_SIZE) / 2,
                               WINDOW_SIZE / 2 - FONT_SIZE / 2,
                               FONT_SIZE,
                               TEXT_COLOR);
            }
            GameState::Run => {}
            GameState::End => {
                rndr.draw_text(END_STRING,
                               WINDOW_SIZE / 2 - measure_text(END_STRING, FONT_SIZE) / 2,
                               WINDOW_SIZE / 2 - FONT_SIZE / 2,
                               FONT_SIZE,
                               TEXT_COLOR);
            }
        }
    }
}