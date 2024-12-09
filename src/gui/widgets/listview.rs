use super::super::style::*;
use super::super::model::*;
use super::widget::*;

use crate::core::cell::*;
use crate::core::event::*;
use crate::core::surface::*;

pub struct ListView<M: Model> {
    pub style: Style,
    pub items: Vec<Box<str>>,
    pub selected_item_idx: Option<usize>,
    width: usize,
    height: usize,
    pub on_item_click: MouseCallback<M>
}

impl<M: Model> Widget<M> for ListView<M> {

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

        if click_y < self.items.len() as i32 {
            self.selected_item_idx = Some(click_y as usize);
            Some((self.on_item_click.clone(), (click_x, click_y)))
        }
        else {
            self.selected_item_idx = None;
            None   
        }
    }

    fn handle_key_press(&mut self, key: Key) -> Option<KeyCallback<M>> {
    
        if let Some(idx) = &mut self.selected_item_idx {

            match key {
                Key::Up => *idx = idx.saturating_sub(1),
                Key::Down => *idx = (*idx + 1).min(self.items.len() - 1),
                _ => {}
            }
        }

        None
    
    }

    fn render(&self, _context: &FrameContext, mut surf: SubSurface) {

        surf.write_lines(0, 0, &self.items);

        if let Some(idx) = self.selected_item_idx {
            surf.fill_range_bg(0..self.width, idx..(idx + 1), self.style.highlight_bg_colour);
        }

        surf.fill_range(0
            ..self.width,
            self.items.len()..self.height,
            Cell::new(
                ' ',
                BLACK,
                self.style.bg_colour
            )
        );
    }
}

impl<M: Model> ListView<M> {

    pub fn new(width: usize, height: usize) -> Self {
        Self::with_items(width, height, Vec::new())
    }

    pub fn with_items(width: usize, height: usize, items: Vec<Box<str>>) -> Self {

        let mut style = Style::default();

        style.highlight_bg_colour = CYAN;

        Self {
            style,
            items,
            selected_item_idx: None,
            width,
            height,
            on_item_click: MouseCallback::noop()
        }
    }
}