use super::super::style::*;
use super::super::model::*;
use super::widget::*;

use crate::core::cell::*;
use crate::core::event::*;
use crate::core::surface::*;

pub struct Entry {
    pub length: usize,
    pub text: String,
    pub style: Style,
    cursor: i32,
    scroll: usize,
    has_focus: bool,
    focus_timer: f32,
    max_chars: Option<u32>
}

impl<M: Model> Widget<M>  for Entry {

    fn get_inner_width(&self) -> usize {
        self.length
    }

    fn get_inner_height(&self) -> usize {
        1
    }

    fn get_style(&self) -> &Style {
        &self.style
    }

    fn handle_left_click(&mut self, click_x: i32, _: i32) -> Option<(MouseCallback<M>, (i32, i32))> {
        self.has_focus = true;
        self.cursor = click_x.clamp(0, self.text.chars().count() as i32);
        None
    }

    fn handle_key_press(&mut self, key: Key) -> Option<KeyCallback<M>> {

        match key {
            Key::Backspace if self.cursor != 0  => {
                self.text.remove(self.cursor as usize - 1);
                self.cursor = (self.cursor - 1).max(0);
            },
            Key::Left => {
                self.cursor = (self.cursor - 1).max(0);
            },
            Key::Right => {
                self.cursor = (self.cursor + 1).min(self.text.chars().count() as i32);
            },
            Key::Up => {
                self.cursor = 0;
            },
            Key::Down => {
                self.cursor = self.text.chars().count() as i32;
            },
            _ => {

                if self.max_chars.is_some_and(|chars| chars == self.text.chars().count() as u32) {
                    return None;
                }

                if let Some(chr) = key.to_chr() {
                    self.text.insert(self.cursor as usize, chr);
                    self.cursor += 1;
                }
            }
        }

        self.clamp_scroll();

        None

    }

    fn remove_focus(&mut self) {
        self.focus_timer = 0.0;
        self.has_focus = false;
    }

    fn update(&mut self, _model: &mut M, context: &FrameContext) {
        if self.has_focus {
            self.focus_timer += context.dt_seconds;
        }
    }

    fn render(&self, _context: &FrameContext, mut surf: SubSurface) {
        
        surf.write_line(0, 0, &self.text[self.scroll..].to_string());

        if self.focus_timer % 1.0 > 0.5 {
            surf.set_fg(self.cursor as usize - self.scroll, 0, self.style.highlight_fg_colour);
            surf.set_bg(self.cursor as usize - self.scroll, 0, self.style.highlight_bg_colour);
        }
    }
}

impl Entry {

    pub fn new(length: usize, text: impl Into<String>) -> Entry {

        let mut style = Style::default();
        style.border_style = BORDER_SUNKEN_BEVEL_HALF;

        Entry {
            length,
            text: text.into(),
            style,
            cursor: 0,
            scroll: 0,
            has_focus: false,
            focus_timer: 0.0,
            max_chars: None
        }
    }

    pub fn with_max_chars(length: usize, text: impl Into<String>, max_chars: u32) -> Entry {

        let mut style = Style::default();
        style.border_style = BORDER_SUNKEN_BEVEL_HALF;

        Entry {
            length,
            text: text.into(),
            style,
            cursor: 0,
            scroll: 0,
            has_focus: false,
            focus_timer: 0.0,
            max_chars: Some(max_chars)
        }
    }

    fn clamp_scroll(&mut self) {

        self.scroll = if self.text.chars().count() < self.length {
            0
        }
        else if (self.cursor as usize) < self.scroll {
            self.cursor as usize
        }
        else if self.scroll + self.length >= self.text.chars().count() {
            self.text.chars().count() - self.length + 1
        }
        else if self.cursor as usize >= self.scroll + self.length {
            self.cursor as usize - self.length + 1
        }
        else {
            self.scroll
        }
    }

    pub fn reset_text(&mut self, text: String) {
        self.cursor = text.chars().count() as i32;
        self.text = text;
        self.scroll = 0;
    }

}