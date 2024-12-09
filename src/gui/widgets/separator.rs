use super::super::style::*;
use super::super::model::*;
use super::widget::*;

use crate::core::cell::*;
use crate::core::event::*;
use crate::core::surface::*;

pub struct Separator {
    pub style: Style,
    orientation: Orientation,
    width: usize,
    height: usize
}

impl<M: Model> Widget<M> for Separator {

    fn get_inner_width(&self) -> usize {
        self.width
    }

    fn get_inner_height(&self) -> usize {
        self.height
    }

    fn get_style(&self) -> &Style {
        &self.style
    }

    fn init_max_width(&mut self, width: usize) {
        if self.orientation == Orientation::Horizontal {
            self.width = width;
        }
    }

    fn init_max_height(&mut self, height: usize) {
        if self.orientation == Orientation::Vertical {
            self.height = height;
        }
    }

    fn render(&self, _context: &FrameContext, mut surf: SubSurface) {
        surf.fill_char(
            if self.orientation == Orientation::Vertical {
                '│'
            }
            else {
                '─'
            }
        )
    }
}

impl Separator {
    pub fn new(orientation: Orientation) -> Separator {
        Self {
            style: Style::default(),
            orientation,
            width: 1,
            height: 1
        }
    }
}