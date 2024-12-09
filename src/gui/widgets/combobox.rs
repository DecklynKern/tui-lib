use super::super::style::*;
use super::super::model::*;
use super::widget::*;

use crate::core::cell::*;
use crate::core::event::*;
use crate::core::surface::*;

pub struct ComboBox<M: Model> {
    pub style: Style,
    pub items: Vec<Box<str>>,
    pub selected_item_idx: usize,
    pub expanded: bool,
    pub mouse_over: bool,
    width: usize,
    pub on_item_click: MouseCallback<M>
}

impl<M: Model> Widget<M> for ComboBox<M> {

    fn get_inner_width(&self) -> usize {
        self.width
    }

    fn get_inner_height(&self) -> usize {
        if self.expanded {
            self.items.len()
        }
        else {
            1
        }
    }

    fn get_style(&self) -> &Style {
        &self.style
    }

    fn remove_focus(&mut self) {
        self.expanded = false;
    }

    fn handle_left_click(&mut self, click_x: i32, click_y: i32) -> Option<(MouseCallback<M>, (i32, i32))> {

        if self.expanded && click_y < self.items.len() as i32 {
            self.expanded = false;
            Some((self.on_item_click.clone(), (click_x, click_y)))
        }
        else {
            self.expanded = true;
            None
        }
    }

    fn handle_mouse_over(&mut self, _mouse_x: i32, _mouse_y: i32) {
        self.mouse_over = true;
    }

    fn handle_mouse_over_release(&mut self, _mouse_x: i32, _mouse_y: i32) {
        self.mouse_over = false;
    }

    fn handle_key_press(&mut self, key: Key) -> Option<KeyCallback<M>> {
    
        self.selected_item_idx = match key {
            Key::Up => self.selected_item_idx.saturating_sub(1),
            Key::Down => (self.selected_item_idx + 1).min(self.items.len() - 1),
            _ => self.selected_item_idx
        };

        None

    }

    fn update(&mut self, _model: &mut M, context: &FrameContext) {

        if self.expanded && self.mouse_over && context.mouse_pos.cell_y < self.items.len() as i32 {
            self.selected_item_idx = context.mouse_pos.cell_y as usize;
        }
    }

    fn render(&self, _context: &FrameContext, mut surf: SubSurface) {

        if self.expanded {

            surf.fill_range(
                0..self.width,
                0..self.items.len(),
                Cell::new(
                    ' ',
                    BLACK,
                    self.style.bg_colour
                )
            );

            surf.write_lines(0, 0, &self.items);
            surf.fill_range_bg(0..self.width, self.selected_item_idx..(self.selected_item_idx + 1), self.style.highlight_bg_colour);

            surf.set_char(self.width - 1, 0, '▼');

        }
        else {
            surf.write_line_colour(0, 0, self.items[self.selected_item_idx].as_ref(), BLACK, self.style.bg_colour);
            surf.set_char(self.width - 1, 0, '▲');
        }
    }
}

impl<M: Model> ComboBox<M> {

    pub fn new(width: usize) -> Self {
        Self::with_items(width, Vec::new())
    }

    pub fn with_items(width: usize, items: Vec<Box<str>>) -> Self {

        let mut style = Style::default();

        style.bg_colour = WHITE;
        style.highlight_bg_colour = CYAN;

        Self {
            style,
            items,
            selected_item_idx: 0,
            expanded: false,
            mouse_over: false,
            width,
            on_item_click: MouseCallback::noop()
        }
    }
}