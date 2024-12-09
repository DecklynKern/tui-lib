use super::super::style::*;
use super::super::model::*;
use super::widget::*;

use crate::core::event::*;
use crate::core::surface::*;

pub struct Label {
    width: usize,
    pub text: String,
    pub style: Style
}

impl<M: Model> Widget<M> for Label {

    fn get_inner_width(&self) -> usize {
        self.width
    }

    fn get_inner_height(&self) -> usize {
        1
    }

    fn get_style(&self) -> &Style {
        &self.style
    }

    fn render(&self, _context: &FrameContext, mut surf: SubSurface) {
        surf.write_line(0, 0, &self.text);
    }
}

impl Label {
    
    pub fn new(text: impl Into<String>) -> Label {
        let str = text.into();
        Label {
            width: str.chars().count(),
            text: str,
            style: Style::default()
        }
    }

    pub fn with_width(text: impl Into<String>, width: usize) -> Label {
        Label {
            width,
            text: text.into(),
            style: Style::default()
        }
    }
}