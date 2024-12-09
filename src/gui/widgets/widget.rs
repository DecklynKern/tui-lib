use super::super::style::*;
use super::super::model::*;
use super::scrollbar::*;
use crate::core::event::*;

use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::mem;

use crate::core::event::*;
use crate::core::math::*;
use crate::core::surface::*;

#[derive(PartialEq)]
pub enum Orientation {
    Vertical,
    Horizontal
}

pub type WidgetRef<M> = Rc<RefCell<M>>;

pub enum Callback<M: Model, A> {
    Unbound,
    Simple(fn(&mut M)),
    UseArg(fn(&mut M, A))
}

pub type SimpleCallback<M> = Callback<M, ()>;
pub type MouseCallback<M> = Callback<M, (i32, i32)>;
pub type KeyCallback<M> = Callback<M, Key>;

impl<M: Model, A> Callback<M, A> {

    pub fn noop() -> Self {
        Callback::Unbound
    }
    
    pub fn bind_simple(&mut self, new_func: fn(&mut M)) {
        let _ = mem::replace(self, Callback::Simple(new_func));
    }
    
    pub fn bind(&mut self, new_func: fn(&mut M, A)) {
        let _ = mem::replace(self, Callback::UseArg(new_func));
    }

    pub fn call(&self, model: &mut M, arg: A) {

        match self {
            Self::Unbound => {},
            Self::Simple(func) => func(model),
            Self::UseArg(func) => func(model, arg)
        }
    }
}

impl<M: Model, A> From<fn(&mut M)> for Callback<M, A> {
    fn from(value: fn(&mut M)) -> Self {
        Self::Simple(value)
    }
}

impl<M: Model, A> From<fn(&mut M, A)> for Callback<M, A> {
    fn from(value: fn(&mut M, A)) -> Self {
        Self::UseArg(value)
    }
}

impl<M: Model, A> Clone for Callback<M, A> {
    fn clone(&self) -> Self {
        match self {
            Self::Unbound => Self::Unbound,
            Self::Simple(func) => Self::Simple(*func),
            Self::UseArg(func) => Self::UseArg(*func)
        }
    }
}

pub trait Widget<M: Model> {
    fn get_inner_width(&self) -> usize;
    fn get_inner_height(&self) -> usize;
    fn get_render_priority(&self) -> i32 {
        0
    }
    fn get_style(&self) -> &Style;
    fn setup(&mut self) {
    }
    fn init_max_width(&mut self, _width: usize) {
    }
    fn init_max_height(&mut self, _height: usize) {
    }
    fn handle_left_click(&mut self, _click_x: i32, _click_y: i32) -> Option<(MouseCallback<M>, (i32, i32))> {
        None
    }
    fn handle_right_click(&mut self, _click_x: i32, _click_y: i32) -> Option<(MouseCallback<M>, (i32, i32))> {
        None
    }
    fn handle_left_click_release(&mut self) -> Option<SimpleCallback<M>> {
        None
    }
    fn handle_scroll_up(&mut self) {
    }
    fn handle_scroll_down(&mut self) {   
    }
    fn handle_key_press(&mut self, _key: Key) -> Option<KeyCallback<M>> {
        None
    }
    fn handle_mouse_over(&mut self, _mouse_x: i32, _mouse_y: i32) {
    }
    fn handle_mouse_over_release(&mut self, _mouse_x: i32, _mouse_y: i32) {
    }
    fn try_focus(&mut self, _click_x: i32, _click_y: i32) -> bool {
        true
    }
    fn remove_focus(&mut self) {
    }
    fn update(&mut self, _model: &mut M, _context: &FrameContext) {
    }
    fn render(&self, context: &FrameContext, surf: SubSurface);
}

impl<M: Model> dyn Widget<M> {

    pub fn is_mouse_over(&self, mouse_x: i32, mouse_y: i32) -> bool {
        point_rect_collision(
            (mouse_x as usize, mouse_y as usize), 
            (0,
            0,
            self.get_bordered_width(),
            self.get_bordered_height())
        )
    }

    pub fn get_bordered_width(&self) -> usize {
        self.get_inner_width() + 
        if self.get_style().border_style == Border::None {0} else {2}
    }

    pub fn get_bordered_height(&self) -> usize {
        self.get_inner_height() + 
        if self.get_style().border_style == Border::None {0} else {2}
    }

    pub fn get_padded_width(&self) -> usize {
        let style = self.get_style();
        self.get_bordered_width() + style.pad_left + style.pad_right
    }

    pub fn get_padded_height(&self) -> usize {
        let style = self.get_style();
        self.get_bordered_height() + style.pad_top + style.pad_bottom
    }

    pub fn render_full(&self, context: &FrameContext, mut surf: SubSurface) {
        self.get_style().apply_border(&mut surf);
        self.render(context, surf);
    }
}

pub struct Scroll<M: Model, S: Scrollbar<M>> {
    pub scroll_pos: usize,
    pub view_size: usize,
    pub total_size: usize,
    scrollbar: Option<WidgetRef<S>>,
    _p: PhantomData<M>
}

impl<M: Model, S: Scrollbar<M>> Scroll<M, S> {

    pub fn new(scroll_pos: usize, view_size: usize, total_size: usize) -> Self {
        Self {
            scroll_pos,
            view_size,
            total_size,
            scrollbar: None,
            _p: PhantomData
        }
    }

    pub fn decriment(&mut self) {
        self.set_scroll(self.scroll_pos.saturating_sub(1));
    }

    pub fn increment(&mut self) {
        self.set_scroll((self.scroll_pos + 1).min((self.total_size).saturating_sub(self.view_size)));
    }

    pub fn set_scroll(&mut self, scroll_amount: usize) {

        self.scroll_pos = scroll_amount;

        if let Some(widget_ref) = &mut self.scrollbar {
            widget_ref.borrow_mut().after_scroll();
        }
    }

    pub fn set_scroll_proportion(&mut self, prop: f32) {
        self.set_scroll((prop * ((self.total_size as f32 - self.view_size as f32).max(0.0))).round() as usize);
    }

    pub fn get_scroll_proportion(&self) -> f32 {
        self.scroll_pos as f32 / (self.total_size as f32 - self.view_size as f32).max(1.0)
    }

    pub fn reset_scroll(&mut self) {
        self.set_scroll(0);
    }
}

pub trait HorizontalScrollWidget<M: Model>: Widget<M> {
    fn get_horizontal_scroll(&self) -> &Scroll<M, HorizontalScrollbar<M>>;
    fn get_horizontal_scroll_mut(&mut self) -> &mut Scroll<M, HorizontalScrollbar<M>>;
    fn after_horizontal_scroll(&mut self) {}
    fn reload_horizontal_scroll_size(&mut self);
}

pub trait VerticalScrollWidget<M: Model>: Widget<M> {
    fn get_vertical_scroll(&self) -> &Scroll<M, VerticalScrollbar<M>>;
    fn get_vertical_scroll_mut(&mut self) -> &mut Scroll<M, VerticalScrollbar<M>>;
    fn after_vertical_scroll(&mut self) {}
    fn reload_vertical_scroll_size(&mut self);
}