use std::any::Any;

// use super::PopupType;
use crate::core::event::*;
use super::Frame;

pub trait Model: Sized {
    fn build(&mut self, _: Frame<Self>) {
    }
    fn quit(&mut self) -> Option<Box<dyn Any>> {
        None
    }
    fn preprocess_event(&mut self, _event: &Event) -> bool {
        false
    }
    fn update(&mut self, _context: &FrameContext) {}
}

pub struct EmptyModel {
}

impl Model for EmptyModel {
}