use raylib::prelude::{*, KeyboardKey::{KEY_UP, KEY_DOWN, KEY_LEFT, KEY_RIGHT}};
use std::collections::VecDeque;

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
    Help,
    Credits,
}

#[derive(Clone)]
enum EndBehavior {
    Wrap,
    Clamp,
}

// ---------- STRUCTS ---------

#[derive(PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

/// Currently selected menu item and the selection indicators.
#[derive(Clone)]
struct MenuIndex {
    index: i32,
    items: i32,
    left_sel: String,
    right_sel: String,
    end_behavior: EndBehavior,
}

struct Snake {
    body: Body,
    head: Point,
}

/// Individual option in a menu.
// Maybe add a closure for actions
// Not entirely sure how I want that to work
#[derive(Clone)]
struct MenuOption {
    display: String,
}

struct Menu { // TODO: Extract Menu implementations to separate file
    index: MenuIndex,
    options: Vec<MenuOption>,
    font_size: i32,
    color: Color,
}

struct MenuBuilder {
    options: Vec<MenuOption>,
    font_size: Option<i32>,
    color: Option<Color>,
    left_sel: Option<String>,
    right_sel: Option<String>,
    end_behavior: Option<EndBehavior>,
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

impl MenuIndex {
    fn new(items: i32, left_sel: String, right_sel: String, end_behavior: EndBehavior) -> Self {
        Self {
            index: 0,
            items,
            left_sel,
            right_sel,
            end_behavior,
        }
    }

    /// Select the next menu item.
    fn next(&mut self) {
        self.index = match self.end_behavior {
            EndBehavior::Wrap => { self.index_wrap(self.index + 1) }
            EndBehavior::Clamp => { self.index_clamp(self.index + 1) }
        };
    }

    /// Select the previous menu item.
    fn prev(&mut self) {
        self.index = match self.end_behavior {
            EndBehavior::Wrap => { self.index_wrap(self.index - 1) }
            EndBehavior::Clamp => { self.index_clamp(self.index - 1) }
        };
    }

    /// Clamp index to list.
    fn index_clamp(&self, val: i32) -> i32 {
        if val < 0 {
            0
        } else {
            self.index
        }
    }

    /// Wrap index to list.
    fn index_wrap(&self, val: i32) -> i32 {
        if val > self.items - 1 {
            0
        } else if val < 0 {
            self.items - 1
        } else {
            self.index
        }
    }
}

impl MenuOption {
    fn new(display: &str) -> Box<Self> {
        Box::new(Self {
            display: display.to_string(),
        })
    }
}

impl Menu {
    /// Generate empty MenuBuilder.
    fn init() -> MenuBuilder {
        MenuBuilder {
            options: Vec::new(),
            color: None,
            font_size: None,
            right_sel: None,
            left_sel: None,
            end_behavior: None,
        }
    }

    fn next(&mut self) {
        self.index.next();
    }

    fn prev(&mut self) {
        self.index.prev();
    }

    /// Draw menu to the renderer provided.
    fn draw<T: RaylibDraw>(&self, rndr: &mut T) {
        // Calculate the x origins for each line
        let x: Vec<(&MenuOption, i32)> = self.options
            .iter()
            .map(|x| (x, WINDOW_SIZE / 2 - measure_text(&*x.display, self.font_size) / 2))
            .collect::<Vec<(&MenuOption, i32)>>();
        // Calculate the y origin for the first line
        let y: f32 = WINDOW_SIZE as f32 / 2.0 - self.font_size as f32 * 1.5;
        // Iterate over menu options with an index
        for (i, (op, pos)) in x.iter().enumerate() {
            if self.index.index == i as i32 { // If current option is selected
                rndr.draw_text(
                    &*format!("{}{}{}", &self.index.left_sel, &op.display, &self.index.right_sel),
                    *pos - measure_text("> ", self.font_size), // Adjust for selection indicator
                    (y + (self.font_size as f32 * 1.5 * i as f32)) as i32, // Adjust by line number
                    self.font_size,
                    self.color,
                );
            } else {
                rndr.draw_text(
                    &*op.display,
                    *pos,
                    (y + (self.font_size as f32 * 1.5 * i as f32)) as i32, // Adjust by line number
                    self.font_size,
                    self.color,
                );
            }
        }
    }
}

impl MenuBuilder {
    /// Define a new menu item.
    fn item(mut self, display: &str) -> Self {
        self.options.push(*MenuOption::new(display));
        self
    }

    /// Define the text color.
    fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Define the font size.
    fn font_size(mut self, font_size: i32) -> Self {
        self.font_size = Some(font_size);
        self
    }

    // Define the selector appearance.
    fn selector(mut self, left: &str, right: &str) -> Self {
        self.left_sel = Some(left.to_string());
        self.right_sel = Some(right.to_string());
        self
    }

    // Build the final Menu.
    fn build(&self) -> Menu {
        Menu {
            index: MenuIndex::new(
                self.options.len() as i32,
                self.left_sel
                    .clone()
                    .unwrap_or("> ".to_string()),
                self.right_sel
                    .clone()
                    .unwrap_or(" <".to_string()),
                self.end_behavior
                    .clone()
                    .unwrap_or(EndBehavior::Wrap)
            ),
            options: self.options
                .clone(),
            color: self.color
                .unwrap_or(Color::RAYWHITE),
            font_size: self.font_size
                .unwrap_or(40),
        }
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

fn draw_grid_rect<T: RaylibDraw>(draw_handle: &mut T, point: &Point, color: Color) {
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
    let mut main_menu: Menu = Menu::init()
        .item("Start Game")
        .item("Help")
        .item("Credits")
        .color(TEXT_COLOR)
        .font_size(FONT_SIZE)
        .build();

    // While window is open
    while !rl.window_should_close() {
        // -------- LOGIC ---------
        match game.game_state {
            GameState::Start => {
                match rl.get_key_pressed() {
                    KEY_UP => {
                        main_menu.next()
                    }
                    KEY_DOWN => {
                        main_menu.prev()
                    }
                    KEY_RIGHT => {
                    }
                }
            }
            GameState::Help => {

            }
            GameState::Credits => {

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
        match game.game_state {
            GameState::Start => {
                main_menu.draw(&mut rndr);
            }
            GameState::Run => {
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
            }
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

// --------- TESTS -----------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mi_clamp() {
        assert_eq!(MenuIndex { index: 2, items: 3, left_sel: "".to_string(), right_sel: "".to_string(), end_behavior: EndBehavior::Clamp }.next().index, 2);
        assert_eq!(MenuIndex { index: 0, items: 3, left_sel: "".to_string(), right_sel: "".to_string(), end_behavior: EndBehavior::Clamp }.prev().index, 0);
    }

    #[test]
    fn test_mi_wrap() {
        assert_eq!(MenuIndex { index: 2, items: 3, left_sel: "".to_string(), right_sel: "".to_string(), end_behavior: EndBehavior::Wrap }.next().index, 0);
        assert_eq!(MenuIndex { index: 0, items: 3, left_sel: "".to_string(), right_sel: "".to_string(), end_behavior: EndBehavior::Wrap }.prev().index, 2);
    }
}