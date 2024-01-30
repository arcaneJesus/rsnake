use raylib::prelude::*;
use crate::WINDOW_SIZE;

enum EndBehavior {
    Wrap,
    Clamp
}

type MenuItems<T> = Vec<MenuItem<T>>;

pub struct Menu<T> {
    index: i32,
    left_sel: String,
    right_sel: String,
    end_behavior: EndBehavior,
    items: MenuItems<T>,
    font_size: i32,
    color: Color,
}

struct MenuItem<T> {
    display: String,
    func: Box<dyn Fn() -> T>
}

impl<T> MenuItem<T> {
    fn from<F>(display: &str, func: F) -> Self
    where
        F: Fn() -> T
    {
        Self {
            display: display.to_string(),
            func: Box::new(func),
        }
    }
}

impl<T> Menu<T> {
    pub fn new() -> Self {
        Self {
            index: 0,
            left_sel: "> ".to_string(),
            right_sel: " <".to_string(),
            end_behavior: EndBehavior::Wrap,
            items: Vec::new(),
            font_size: 40,
            color: Color::RAYWHITE,
        }
    }

    pub fn from<F>(items: Vec<(&str, F)>) -> Self
    where
        F: Fn() -> T
    {
        Self {
            index: 0,
            left_sel: "> ".to_string(),
            right_sel: " <".to_string(),
            end_behavior: EndBehavior::Wrap,
            items: items
                .iter()
                .map(|(d, f)| MenuItem::from(d,f))
                .collect(),
            font_size: 40,
            color: Color::RAYWHITE,
        }
    }

    pub fn change_end_behavior(self) -> Self {
        match self.end_behavior {
            EndBehavior::Wrap => {
                Self {
                    end_behavior: EndBehavior::Clamp,
                    ..self
                }
            }
            EndBehavior::Clamp => {
                Self {
                    end_behavior: EndBehavior::Wrap,
                    ..self
                }
            }
        }
    }

    pub fn set_font_size(self, size: i32) -> Self {
        Self {
            font_size: size,
            ..self
        }
    }

    pub fn set_color(self, color: Color) -> Self {
        Self {
            color,
            ..self
        }
    }

    pub fn set_selection_indicators(self, left: &str, right: &str) -> Self {
        Self {
            left_sel: left.to_string(),
            right_sel: right.to_string(),
            ..self
        }
    }

    fn index_clamp(&self, val: i32) -> i32 {
        if val < 0 {
            0
        } else {
            self.index
        }
    }

    fn index_wrap(&self, val: i32) -> i32 {
        let len: i32 = self.items.len() as i32 - 1;
        if val > len {
            0
        } else if val < 0 {
            len
        } else {
            self.index
        }
    }

    pub fn next(&mut self) {
        self.index = match self.end_behavior {
            EndBehavior::Wrap => {self.index_wrap(self.index + 1)}
            EndBehavior::Clamp => {self.index_clamp(self.index + 1)}
        };
    }

    pub fn prev(&mut self) {
        self.index = match self.end_behavior {
            EndBehavior::Wrap => {self.index_wrap(self.index - 1)}
            EndBehavior::Clamp => {self.index_clamp(self.index - 1)}
        };
    }

    pub fn select(&self) -> T {
        (self.items.get(self.index as usize).unwrap().func)()
    }

    pub fn draw<RD: RaylibDraw>(&self, rndr: &mut RD) {
        // Calculate the x origins for each line
        let x: Vec<(&MenuItem<T>, i32)> = self.items
            .iter()
            .map(|x| (x, WINDOW_SIZE / 2 - measure_text(&*x.display, self.font_size)/2))
            .collect();
        //  Calculate the y origin for the first line
        let y: f32 = WINDOW_SIZE as f32 / 2.0 - self.font_size as f32 * 1.5;
        // Iterate over menu options with an index
        for (i, (op, pos)) in x.iter().enumerate() {
            if self.index == i as i32 {
                rndr.draw_text(
                    &*format!("{}{}{}", &self.left_sel, &op.display, &self.right_sel),
                    *pos - measure_text("> ", self.font_size), // Adjust for selection indicators
                    (y + (self.font_size as f32 * 1.5 * i as f32)) as i32, // Adjust by line number
                    self.font_size,
                    self.color,
                );
            } else {
                rndr.draw_text(
                    &*op.display,
                    *pos,
                    (y+(self.font_size as f32 * 1.5 * i as f32)) as i32, // Adjust by line number
                    self.font_size,
                    self.color,
                );
            }
        }
    }
}