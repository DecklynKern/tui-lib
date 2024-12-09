use super::super::style::*;
use super::super::model::*;
use super::widget::*;

use crate::core::cell::*;
use crate::core::event::*;
use crate::core::surface::*;

pub struct Progressbar {
    pub style: Style,
    width: usize,
    pub progress: f32
}

impl<M: Model> Widget<M> for Progressbar {

    fn get_inner_width(&self) -> usize {
        self.width
    }

    fn get_inner_height(&self) -> usize {
        2
    }

    fn get_style(&self) -> &Style {
        &self.style
    }

    fn init_max_width(&mut self, width: usize) {
        self.width = width;
    }

    fn render(&self, _context: &FrameContext, mut surf: SubSurface) {

        surf.fill(Cell::solid(DARK_GREEN));

        let bar_width = self.progress * self.width as f32;

        surf.fill_range_bg(0..bar_width as usize, 0..2, GREEN);

        if bar_width % 1.0 > 0.5 {
            surf.fill_range(bar_width as usize..(bar_width as usize + 1), 0..2, Cell::new(LEFT_HALF_BLOCK, GREEN, TRANSPARENT));
        }
    }
}

impl Progressbar {
    pub fn new(width: usize) -> Progressbar {

        let mut style = Style::default();
        style.border_style = Border::Line(BLACK, GREY);

        Self {
            style,
            width,
            progress: 0.0
        }
    }
}