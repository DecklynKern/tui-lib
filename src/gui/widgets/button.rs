use super::super::style::*;
use super::super::model::*;
use super::widget::*;

use crate::core::event::*;
use crate::core::surface::*;

pub struct Button<M: Model> {
    pub style: Style,
    on_click: MouseCallback<M>,
    pub text: String
}

impl<M: Model> Widget<M> for Button<M> {

    fn get_inner_width(&self) -> usize {
        self.text.chars().count()
    }

    fn get_inner_height(&self) -> usize {
        1
    }

    fn get_style(&self) -> &Style {
        &self.style
    }

    fn handle_left_click(&mut self, click_x: i32, click_y: i32) -> Option<(MouseCallback<M>, (i32, i32))> {
        self.style.border_style = BORDER_DOUBLE_LINE;
        Some((self.on_click.clone(), (click_x, click_y)))
    }

    fn handle_left_click_release(&mut self) -> Option<SimpleCallback<M>> {
        self.style.border_style = BORDER_DASHED;
        None
    }

    fn handle_mouse_over(&mut self, _mouse_x: i32, _mouse_y: i32) {
        self.style.border_style = BORDER_LINE;
    }

    fn handle_mouse_over_release(&mut self, _mouse_x: i32, _mouse_y: i32) {
        self.style.border_style = BORDER_DASHED;
    }

    fn render(&self, _context: &FrameContext, mut surf: SubSurface) {
        // switch border style if mouse over
        surf.write_line(0, 0, &self.text);
    }
}

impl<M: Model> Button<M> {

    pub fn new(text: impl Into<String>) -> Self {
        Self::with_callback(text, Callback::Unbound)
    }

    pub fn with_fn(text: impl Into<String>, on_click: fn(&mut M)) -> Self {
        Self::with_callback(text, on_click.into())
    }

    pub fn with_callback(text: impl Into<String>, on_click: MouseCallback<M>) -> Self {

        let mut style = Style::default();
        style.border_style = BORDER_DASHED;
        
        Button {
            style,
            on_click,
            text: text.into()
        }
    }
}