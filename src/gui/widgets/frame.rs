use super::super::style::*;
use super::super::model::*;
use super::scrollbar::*;
use super::widget::*;

use std::cell::{RefCell, Ref};
use std::collections::HashMap;
use std::rc::Rc;

use crate::core::cell::*;
use crate::core::event::*;
use crate::core::surface::*;

pub enum AlignHorizontal {
    Left,
    Middle,
    Right
}

pub enum AlignVertical {
    Top,
    Middle,
    Bottom
}

struct WidgetPosition {
    pub row: i32,
    pub col: i32,
    pub x: i32,
    pub y: i32,
    pub horizontal_align: AlignHorizontal,
    pub vertical_align: AlignVertical
}

impl WidgetPosition {

    pub fn new(row: i32, col: i32) -> Self {
        Self {
            row,
            col,
            x: 0,
            y: 0,
            horizontal_align: AlignHorizontal::Middle,
            vertical_align: AlignVertical::Middle
        }
    }

    pub fn with_align(row: i32, col: i32, horizontal_align: AlignHorizontal, vertical_align: AlignVertical) -> Self {
        Self {
            row,
            col,
            x: 0,
            y: 0,
            horizontal_align,
            vertical_align
        }
    }
}

pub struct Frame<M: Model + 'static> {
    pub style: Style,
    width: usize,
    height: usize,
    widgets: Vec<(WidgetRef<dyn Widget<M>>, WidgetPosition)>,
    focus: Option<usize>,
    mouse_over: Option<usize>
}

impl<M: Model> Widget<M> for Frame<M> {
    
    fn get_inner_width(&self) -> usize {
        self.width
    }

    fn get_inner_height(&self) -> usize {
        self.height
    }

    fn get_style(&self) -> &Style {
        &self.style
    }

    fn setup(&mut self) {
        
        // very precious code that i hope to never have to touch

        self.widgets.sort_by_key(
            |(widget_ref, _)| widget_ref.borrow().get_render_priority()
        );

        for (widget, _) in &mut self.widgets {
            widget.borrow_mut().setup();
        }

        let mut unique_cols = Vec::new();
        let mut unique_rows = Vec::new();

        let mut col_widgets: HashMap<i32, Vec<Ref<dyn Widget<M>>>> = HashMap::new();
        let mut row_widgets: HashMap<i32, Vec<Ref<dyn Widget<M>>>> = HashMap::new();

        for (widget_ref, pos) in &self.widgets {

            let widget = widget_ref.borrow();

            if let Some(vec) = col_widgets.get_mut(&pos.col) {
                vec.push(widget);
            }
            else {
                col_widgets.insert(pos.col, vec![widget]);
                unique_cols.push(pos.col);
            }

            let widget = widget_ref.borrow();

            if let Some(vec) = row_widgets.get_mut(&pos.row) {
                vec.push(widget);
            }
            else {
                row_widgets.insert(pos.row, vec![widget]);
                unique_rows.push(pos.row);
            }
        }

        unique_cols.sort();
        unique_rows.sort();

        let widths: HashMap<i32, i32> = HashMap::from_iter(unique_cols.iter().map(
            |col| (
                *col,
                col_widgets[col].iter().map(
                    |widget| widget.get_padded_width() as i32
                ).max().unwrap()
            )
        ));

        self.width = widths.values().sum::<i32>() as usize;
        
        let heights: HashMap<i32, i32> = HashMap::from_iter(unique_rows.iter().map(
            |row| (
                *row,
                row_widgets[row].iter().map(
                    |widget| widget.get_padded_height() as i32
                ).max().unwrap()
            )
        ));

        self.height = heights.values().sum::<i32>() as usize;
        
        drop(col_widgets);
        drop(row_widgets);

        for (widget_ref, pos) in &self.widgets {

            let mut widget = widget_ref.borrow_mut();

            let border_size = if widget.get_style().border_style == Border::None {0} else {2};

            widget.init_max_width(widths[&pos.col] as usize - border_size);
            widget.init_max_height(heights[&pos.row] as usize - border_size);

        }

        let x_starts: HashMap<i32, i32> = HashMap::from_iter(unique_cols.iter().map(
            |col| (*col, unique_cols.iter().filter(
                |c| *c < col
            ).map(
                |c| widths[c]
            ).sum())
        ));

        let y_starts: HashMap<i32, i32> = HashMap::from_iter(unique_rows.iter().map(
            |row| (*row, unique_rows.iter().filter(
                |r| *r < row
            ).map(
                |r| heights[r]
            ).sum())
        ));

        for (widget_ref, pos) in &mut self.widgets {

            let widget = widget_ref.borrow();
            let style = widget.get_style();

            let x_start = x_starts[&pos.col];
            let cell_width = widths[&pos.col];
            let width = widget.get_padded_width() as i32;
            let pad_left = style.pad_left as i32;

            pos.x = match pos.horizontal_align {
                AlignHorizontal::Left => x_start,
                AlignHorizontal::Middle => x_start + pad_left + (cell_width - width) / 2,
                AlignHorizontal::Right => x_start + cell_width - width + pad_left
            };

            let y_start = y_starts[&pos.row];
            let cell_height = heights[&pos.row];
            let height = widget.get_padded_height() as i32;
            let pad_upper = style.pad_top as i32;

            pos.y = match pos.vertical_align {
                AlignVertical::Top => y_start,
                AlignVertical::Middle => y_start + pad_upper + (cell_height - height) / 2,
                AlignVertical::Bottom => y_start + cell_height - height + pad_upper
            };
        }
    }

    fn try_focus(&mut self, click_x: i32, click_y: i32) -> bool {

        for (idx, (widget_ref, pos)) in self.widgets.iter().enumerate().rev() {

            let mut widget = widget_ref.borrow_mut();

            let rel_click_x = click_x - pos.x;
            let rel_click_y = click_y - pos.y;

            let border_offset = widget.get_style().get_border_offset();

            if widget.is_mouse_over(rel_click_x, rel_click_y) && widget.try_focus(
                rel_click_x + border_offset,
                rel_click_y + border_offset
            ) {

                if let Some(focus) = self.focus {
                    if focus == idx {
                        return true;
                    }
                } 

                drop(widget);
                self.try_send_unfocus();
                self.focus = Some(idx);
                return true;

            }
        }
        
        self.try_send_unfocus();
        self.focus = None;
        false

    }

    fn handle_left_click(&mut self, click_x: i32, click_y: i32) -> Option<(MouseCallback<M> ,(i32, i32))> {

        self.focus.and_then(|idx| {

            let mut widget = self.widgets[idx].0.borrow_mut();

            let offset = widget.get_style().get_border_offset();

            widget.handle_left_click(
                click_x - self.widgets[idx].1.x + offset,
                click_y - self.widgets[idx].1.y + offset
            )
        })
    }

    fn handle_left_click_release(&mut self) -> Option<SimpleCallback<M>> {
        self.focus.and_then(|idx| {
            self.widgets[idx].0.borrow_mut().handle_left_click_release()
        })
    }

    fn handle_right_click(&mut self, click_x: i32, click_y: i32) -> Option<(MouseCallback<M>, (i32, i32))> {

        self.focus.and_then(|idx| {

            let mut widget = self.widgets[idx].0.borrow_mut();

            let offset = widget.get_style().get_border_offset();

            widget.handle_right_click(
                click_x - self.widgets[idx].1.x - offset,
                click_y - self.widgets[idx].1.y - offset
            )
        })
    }

    fn handle_key_press(&mut self, key: Key) -> Option<KeyCallback<M>> {
        self.focus.and_then(|idx| {
            self.widgets[idx].0.borrow_mut().handle_key_press(key)
        })
    }

    fn remove_focus(&mut self) {
        if let Some(idx) = self.focus {
            self.widgets[idx].0.borrow_mut().remove_focus();
        }
    }
    
    fn update(&mut self, model: &mut M, context: &FrameContext) {

        for (idx, (widget_ref, pos)) in self.widgets.iter_mut().enumerate() {

            let mut widget = widget_ref.borrow_mut();
            let offset_context = context.with_mouse_offset(-pos.x, -pos.y);

            if widget.is_mouse_over(offset_context.mouse_pos.cell_x, offset_context.mouse_pos.cell_y) {
                if self.mouse_over.is_none() || self.mouse_over.is_some_and(|mouse_over| mouse_over != idx) {
                    widget.handle_mouse_over(
                        offset_context.mouse_pos.cell_x,
                        offset_context.mouse_pos.cell_y
                    );
                    self.mouse_over = Some(idx);
                }
            }
            else if self.mouse_over.is_some_and(|mouse_over| mouse_over == idx) {
                widget.handle_mouse_over_release(
                    offset_context.mouse_pos.cell_x,
                    offset_context.mouse_pos.cell_y
                );
                self.mouse_over = None;
            }

            widget.update(model, &offset_context);

        }
    }

fn render(&self, context: &FrameContext, mut surf: SubSurface) {

        for (widget_ref, pos) in &self.widgets {

            let widget = widget_ref.borrow();

            let subsurface = surf.get_sub_surface(pos.x as usize, pos.y as usize, widget.get_bordered_width(), widget.get_bordered_height());
            widget.render_full(context, subsurface);

        }
    }
}

impl<M: Model + 'static> Frame<M> {

    pub fn new() -> Self {
        Self {
            style: Style::default(),
            width: 1,
            height: 1,
            widgets: Vec::new(),
            focus: None,
            mouse_over: None
        }
    }

    fn try_send_unfocus(&mut self) {
        if let Some(focus_widget) = self.focus {
            self.widgets[focus_widget].0.borrow_mut().remove_focus();
        }
    }

    pub fn add_widget<W: Widget<M> + 'static>(&mut self, widget: W, col: i32, row: i32) -> WidgetRef<W> {
        let widget_ref = Rc::new(RefCell::new(widget));
        self.widgets.push((
            widget_ref.clone(),
            WidgetPosition::new(row, col)
        ));
        widget_ref
    }

    pub fn add_widget_with_align<W: Widget<M> + 'static>(&mut self, widget: W, col: i32, row: i32, horizontal_align: AlignHorizontal, vertical_align: AlignVertical) -> WidgetRef<W> {
        let widget_ref = Rc::new(RefCell::new(widget));
        self.widgets.push((
            widget_ref.clone(),
            WidgetPosition::with_align(
                row,
                col,
                horizontal_align,
                vertical_align
            )
        ));
        widget_ref
    }

    pub fn add_widget_with_horizontal_scrollbar<W: HorizontalScrollWidget<M> + 'static>(&mut self, widget: W, col: i32, row: i32) -> WidgetRef<W> {
    
        let mut frame = Frame::new();

        let widget_ref = frame.add_widget(widget, 0, 0);

        let scrollbar = HorizontalScrollbar::new(widget_ref.clone());
        frame.add_widget(scrollbar, 0, 1);

        self.add_widget(frame, col, row);

        widget_ref

    }

    pub fn add_widget_with_vertical_scrollbar<W: VerticalScrollWidget<M> + 'static>(&mut self, widget: W, col: i32, row: i32) -> WidgetRef<W> {
    
        let mut frame = Frame::new();

        let widget_ref = frame.add_widget(widget, 0, 0);

        let scrollbar = VerticalScrollbar::new(widget_ref.clone());
        frame.add_widget(scrollbar, 1, 0);

        self.add_widget(frame, col, row);

        widget_ref

    }

    pub fn add_widget_with_both_scrollbars<W: HorizontalScrollWidget<M> + VerticalScrollWidget<M> + 'static>(&mut self, widget: W, col: i32, row: i32) -> WidgetRef<W> {
    
        let mut frame = Frame::new();

        let widget_ref = frame.add_widget(widget, 0, 0);

        let horizontal_scrollbar = HorizontalScrollbar::new(widget_ref.clone());
        frame.add_widget(horizontal_scrollbar, 0, 1);

        let vertical_scrollbar = VerticalScrollbar::new(widget_ref.clone());
        frame.add_widget(vertical_scrollbar, 1, 0);

        self.add_widget(frame, col, row);

        widget_ref

    }
}