use super::super::style::*;
use super::super::model::*;
use super::widget::*;

use crate::core::cell::*;
use crate::core::event::*;
use crate::core::surface::*;

pub trait Scrollbar<M: Model>: Widget<M> {
    fn after_scroll(&mut self);
}

pub struct HorizontalScrollbar<M: Model> {
    pub style: Style,
    scrolled_widget: WidgetRef<dyn HorizontalScrollWidget<M>>,
    width: usize
}

impl<M: Model> Widget<M> for HorizontalScrollbar<M> {

    fn get_inner_width(&self) -> usize {
        self.width
    }

    fn get_inner_height(&self) -> usize {
        1
    }

    fn get_style(&self) -> &Style {
        &self.style
    }

    fn init_max_width(&mut self, width: usize) {
        self.width = width;   
    }

    fn handle_left_click(&mut self, click_x: i32, _click_y: i32) -> Option<(MouseCallback<M>, (i32, i32))> {

        let mut widget = self.scrolled_widget.borrow_mut();
        let scroll = widget.get_horizontal_scroll_mut();

        if click_x == 0 {
            scroll.decriment();
        }
        else if click_x as usize == self.width - 1 {
            scroll.increment();
        }
        else {
            scroll.set_scroll_proportion((click_x - 1) as f32 / (self.width - 3) as f32);
        }
        
        None

    }

    fn update(&mut self, _model: &mut M, _context: &FrameContext) {
        // todo
    }

    fn render(&self, _context: &FrameContext, mut surf: SubSurface) {

        surf.fill(Cell::new(
            '═',
            self.style.text_colour,
            self.style.secondary_colour
        ));

        surf.set(0, 0, Cell::new(
            '◄',
            self.style.text_colour,
            self.style.bg_colour
        ));

        surf.set(self.width - 1, 0, Cell::new(
            '►',
            self.style.text_colour,
            self.style.bg_colour
        ));

        let widget = self.scrolled_widget.borrow();
        let scroll = widget.get_horizontal_scroll();

        surf.set_char((scroll.get_scroll_proportion() * (self.width as f32 - 3.0) + 1.0).round() as usize, 0, '+');

    }
}

impl<M: Model> Scrollbar<M> for HorizontalScrollbar<M> {

    fn after_scroll(&mut self) {
        self.scrolled_widget.borrow_mut().after_horizontal_scroll();
    }
}

impl<M: Model> HorizontalScrollbar<M> {

    pub fn new(widget: WidgetRef<dyn HorizontalScrollWidget<M>>) -> Self {
        Self {
            style: Style::default(),
            scrolled_widget: widget,
            width: 1
        }
    }
}

pub struct VerticalScrollbar<M: Model> {
    pub style: Style,
    scrolled_widget: WidgetRef<dyn VerticalScrollWidget<M>>,
    height: usize
}

impl<M: Model> Widget<M> for VerticalScrollbar<M> {

    fn get_inner_width(&self) -> usize {
        1
    }

    fn get_inner_height(&self) -> usize {
        self.height
    }

    fn get_style(&self) -> &Style {
        &self.style
    }

    fn init_max_height(&mut self, height: usize) {
        self.height = height;
    }

    fn handle_left_click(&mut self, _click_x: i32, click_y: i32) -> Option<(MouseCallback<M>, (i32, i32))> {

        let mut widget = self.scrolled_widget.borrow_mut();
        let scroll = widget.get_vertical_scroll_mut();

        if click_y == 0 {
            scroll.decriment();
        }
        else if click_y as usize == self.height - 1 {
            scroll.increment();
        }
        else {
            scroll.set_scroll_proportion((click_y - 1) as f32 / (self.height - 3) as f32);
        }
        
        None

    }

    fn update(&mut self, _model: &mut M, _context: &FrameContext) {
        // todo
    }

    fn render(&self, _context: &FrameContext, mut surf: SubSurface) {

        surf.fill(Cell::new(
            '║',
            self.style.text_colour,
            self.style.secondary_colour
        ));

        surf.set(0, 0, Cell::new(
            '▲',
            self.style.text_colour,
            self.style.bg_colour
        ));

        surf.set(0, self.height - 1, Cell::new(
            '▼',
            self.style.text_colour,
            self.style.bg_colour
        ));

        let widget = self.scrolled_widget.borrow();
        let scroll = widget.get_vertical_scroll();

        surf.set_char(0, (scroll.get_scroll_proportion() * (self.height as f32 - 3.0) + 1.0).round() as usize, '=');

    }
}

impl<M: Model> Scrollbar<M> for VerticalScrollbar<M> {

    fn after_scroll(&mut self) {
        self.scrolled_widget.borrow_mut().after_vertical_scroll();
    }
}

impl<M: Model> VerticalScrollbar<M> {

    pub fn new(widget: WidgetRef<dyn VerticalScrollWidget<M>>) -> Self {
        Self {
            style: Style::default(),
            scrolled_widget: widget,
            height: 1
        }
    }
}