use super::super::style::*;
use super::super::model::*;
use super::scrollbar::*;
use super::widget::*;

use crate::core::cell::*;
use crate::core::event::*;
use crate::core::surface::*;

pub struct Text<M: Model> {
    pub width: usize,
    pub height: usize,
    text: Vec<String>,
    pub editable: bool,
    pub style: Style,
    pub cursor_x: usize,
    pub cursor_y: usize,
    horizontal_scroll: Scroll<M, HorizontalScrollbar<M>>,
    vertical_scroll: Scroll<M, VerticalScrollbar<M>>,
    has_focus: bool,
    focus_timer: f32,
    pub on_key_press: KeyCallback<M>,
    pub on_left_click: MouseCallback<M>
}

impl<M: Model> Widget<M> for Text<M> {

    fn get_inner_width(&self) -> usize {
        self.width
    }

    fn get_inner_height(&self) -> usize {
        self.height
    }

    fn get_style(&self) -> &Style {
        &self.style
    }

    fn handle_left_click(&mut self, click_x: i32, click_y: i32) -> Option<(MouseCallback<M>, (i32, i32))> {

        if self.editable {

            self.cursor_y = (click_y as usize + self.vertical_scroll.scroll_pos).clamp(
                0,
                self.text.len() - 1
            );
    
            self.cursor_x = (click_x as usize + self.horizontal_scroll.scroll_pos).clamp(
                0,
                self.text[self.cursor_y].chars().count()
            );

            self.has_focus = true;

        }
        
        Some((self.on_left_click.clone(), (click_x, click_y)))

    }

    fn handle_key_press(&mut self, key: Key) -> Option<KeyCallback<M>> {

        if !self.editable {
            return Some(self.on_key_press.clone());
        }

        match key {
            Key::Backspace => {

                if self.cursor_x != 0 {
                    self.text[self.cursor_y].remove(self.cursor_x - 1);
                    self.cursor_x -= 1;
                }
                else if self.cursor_y != 0 {

                    let prev_str = self.text.remove(self.cursor_y);
                    
                    self.cursor_y -= 1;
                    self.cursor_x = self.text[self.cursor_y].chars().count();
                    self.text[self.cursor_y].push_str(prev_str.as_str());

                    self.vertical_scroll.total_size -= 1;
                
                }
            }
            Key::Left => {

                if self.cursor_x != 0 {
                    self.cursor_x -= 1;
                }
                else if self.cursor_y != 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = self.text[self.cursor_y].chars().count();
                }
            }
            Key::Right => {

                if self.cursor_x != self.text[self.cursor_y].chars().count() {
                    self.cursor_x += 1;
                }
                else if self.cursor_y != self.text.len() - 1 {
                    self.cursor_x = 0;
                    self.cursor_y += 1;
                }
            }
            Key::Up => {

                if self.cursor_y == 0 {
                    self.cursor_x = 0;
                }
                else {
                    self.cursor_y -= 1;
                    self.cursor_x = self.text[self.cursor_y].chars().count().min(self.cursor_x);
                }
            }
            Key::Down => {

                if self.text.len() - 1 == self.cursor_y {
                    self.cursor_x = self.text[self.cursor_y].chars().count();
                }
                else {
                    self.cursor_y += 1;
                    self.cursor_x = self.text[self.cursor_y].chars().count().min(self.cursor_x);
                }
            }
            Key::Enter => {

                let before = self.text[self.cursor_y][..self.cursor_x].to_string();
                let after = self.text[self.cursor_y][self.cursor_x..].to_string();

                self.text[self.cursor_y] = before;

                self.cursor_x = 0;
                self.cursor_y += 1;
                self.vertical_scroll.total_size += 1;

                self.text.insert(self.cursor_y, after);
                
            }
            _ => if let Some(chr) = key.to_chr() {
                self.text[self.cursor_y].insert(self.cursor_x, chr);
                self.cursor_x += 1;
            }
        }

        self.horizontal_scroll.total_size = self.text[self.cursor_y].chars().count();

        self.after_vertical_scroll();
        self.after_horizontal_scroll();

        Some(self.on_key_press.clone())

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

        for (y, line) in self.text[self.vertical_scroll.scroll_pos..].iter().enumerate() {
            
            if self.horizontal_scroll.scroll_pos >= line.len() || y >= self.height {
                continue;
            }

            surf.write_line(0, y, &line[self.horizontal_scroll.scroll_pos..].to_string());

        }
        
        if self.focus_timer % 1.0 > 0.5 {
            surf.set_fg(
                self.cursor_x - self.horizontal_scroll.scroll_pos,
                self.cursor_y - self.vertical_scroll.scroll_pos,
                self.style.highlight_fg_colour
            );
            surf.set_bg(
                self.cursor_x - self.horizontal_scroll.scroll_pos,
                self.cursor_y - self.vertical_scroll.scroll_pos,
                self.style.highlight_bg_colour
            );
        }
    }
}

impl<M: Model> HorizontalScrollWidget<M> for Text<M> {

    fn get_horizontal_scroll(&self) -> &Scroll<M, HorizontalScrollbar<M>> {
        &self.horizontal_scroll
    }

    fn get_horizontal_scroll_mut(&mut self) -> &mut Scroll<M, HorizontalScrollbar<M>> {
        &mut self.horizontal_scroll
    }

    fn after_horizontal_scroll(&mut self) {

        let line_len = self.text[self.cursor_y].chars().count();

        if line_len < self.width {
            self.horizontal_scroll.scroll_pos = 0;
        }
        else if self.cursor_x < self.horizontal_scroll.scroll_pos {
            self.horizontal_scroll.scroll_pos = self.cursor_x;
        }
        else if self.horizontal_scroll.scroll_pos + self.width >= line_len {
            self.horizontal_scroll.scroll_pos = line_len - self.width + 1;
        }
        else if self.cursor_x >= self.horizontal_scroll.scroll_pos + self.width {
            self.horizontal_scroll.scroll_pos = self.cursor_x - self.width + 1;
        }
    }

    fn reload_horizontal_scroll_size(&mut self) {
        self.horizontal_scroll.total_size = self.text.iter().map(|line| line.chars().count()).max().unwrap_or(0);
    }
}

impl<M: Model> VerticalScrollWidget<M> for Text<M> {

    fn get_vertical_scroll(&self) -> &Scroll<M, VerticalScrollbar<M>> {
        &self.vertical_scroll
    }

    fn get_vertical_scroll_mut(&mut self) -> &mut Scroll<M, VerticalScrollbar<M>> {
        &mut self.vertical_scroll
    }

    fn after_vertical_scroll(&mut self) {

        if self.text.len() < self.height {
            self.vertical_scroll.scroll_pos = 0;
        }
        else if self.cursor_y < self.vertical_scroll.scroll_pos {
            self.vertical_scroll.scroll_pos = self.cursor_y;
        }
        else if self.vertical_scroll.scroll_pos + self.height >= self.text.len() {
            self.vertical_scroll.scroll_pos = self.text.len() - self.height + 1;
        }
        else if self.cursor_y >= self.vertical_scroll.scroll_pos + self.height {
            self.vertical_scroll.scroll_pos = self.cursor_y - self.height + 1;
        }
    }

    fn reload_vertical_scroll_size(&mut self) {
        self.vertical_scroll.total_size = self.text.len();
    }
}

impl<M: Model> Text<M> {

    pub fn new(width: usize, height: usize) -> Text<M> {

        let mut style = Style::default();
        style.border_style = BORDER_SUNKEN_BEVEL_HALF;
        style.bg_colour = WHITE;

        Self {
            width,
            height,
            text: vec![String::new()],
            editable: true,
            style,
            cursor_x: 0,
            cursor_y: 0,
            horizontal_scroll: Scroll::new(
                0,
                width,
                1
            ),
            vertical_scroll: Scroll::new(
                0,
                height,
                1
            ),
            has_focus: false,
            focus_timer: 0.0,
            on_key_press: Callback::noop(),
            on_left_click: Callback::noop()
        }
    }

    pub fn get_text(&self) -> &Vec<String> {
        &self.text
    }

    pub fn with_text(&mut self, func: impl Fn(&mut Vec<String>)) {

        func(&mut self.text);

        self.reload_horizontal_scroll_size();
        self.reload_vertical_scroll_size();

        self.cursor_y = self.cursor_y.min(self.text.len() - 1);
        self.cursor_x = self.cursor_x.min(self.text.get(self.cursor_y).map(String::len).unwrap_or(1))

    }

    pub fn set_text(&mut self, lines: Vec<String>) {
        self.clear();
        self.text = lines;
        self.reload_horizontal_scroll_size();
        self.reload_vertical_scroll_size();
    }

    // make sure to add a line after!
    pub fn clear(&mut self) {

        self.text.clear();

        self.cursor_x = 0;
        self.cursor_y = 0;

        self.vertical_scroll.total_size = 0;
        self.horizontal_scroll.set_scroll(0);

        self.vertical_scroll.total_size = 1;
        self.vertical_scroll.set_scroll(0);

    }

    pub fn clear_safe(&mut self) {
        self.clear();
        self.text.push("".into());
    }
}