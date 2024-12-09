use super::super::style::*;
use super::super::model::*;
use super::widget::*;

use crate::core::cell::*;
use crate::core::event::*;
use crate::core::surface::*;

pub struct Checkbox {
    pub style: Style,
    checked: bool
}

impl<M: Model> Widget<M> for Checkbox {

    fn get_inner_width(&self) -> usize {
        1
    }

    fn get_inner_height(&self) -> usize {
        1
    }

    fn get_style(&self) -> &Style {
        &self.style
    }

    fn handle_left_click(&mut self, _: i32, _: i32) -> Option<(MouseCallback<M>, (i32, i32))> {
        self.checked = !self.checked;
        None
    }

    fn render(&self, context: &FrameContext, mut surf: SubSurface) {

        let chr = if self.checked {
            '√'
        }
        else if context.mouse_pos.cell_x == 0 && context.mouse_pos.cell_y == 0 {
            surf.set_fg(0, 0, DARK_GREY);
            '√'
        }
        else {
            MIDDLE_BLOCK
        };

        surf.set_char(
            0,
            0,
            chr
        );
    }
}

impl Checkbox {
    pub fn new() -> Self {

        let mut style = Style::default();
        style.bg_colour = WHITE;

        Self {
            style,
            checked: false
        }
    }
}