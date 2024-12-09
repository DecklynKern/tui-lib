use super::super::style::*;
use super::super::model::*;
use super::scrollbar::*;
use super::widget::*;

use crate::core::cell::*;
use crate::core::surface::*;
use crate::core::event::*;

pub struct Canvas<M: Model> {
    pub style: Style,
    pub surf: CellSurf,
    pub view_width: usize,
    pub view_height: usize,
    horizontal_scroll: Scroll<M, HorizontalScrollbar<M>>,
    vertical_scroll: Scroll<M, VerticalScrollbar<M>>,
    pub on_left_click: Callback<M, (i32, i32)>,
    pub on_left_click_release: SimpleCallback<M>
}

impl<M: Model> Widget<M> for Canvas<M> {

    fn get_inner_width(&self) -> usize {
        self.view_width
    }
    
    fn get_inner_height(&self) -> usize {
        self.view_height
    }

    fn get_style(&self) -> &Style {
        &self.style
    }

    fn handle_left_click(&mut self, click_x: i32, click_y: i32) -> Option<(MouseCallback<M>, (i32, i32))> {
        Some((self.on_left_click.clone(), (click_x, click_y)))
    }

    fn handle_left_click_release(&mut self) -> Option<SimpleCallback<M>> {
        Some(self.on_left_click_release.clone())
    }

    fn render(&self, _context: &FrameContext, mut surf: SubSurface) {
        surf.blit(&self.surf, -(self.horizontal_scroll.scroll_pos as i32), -(self.vertical_scroll.scroll_pos as i32));
    }
}

impl<M: Model> Canvas<M> {

    pub fn new(width: usize, height: usize) -> Self {

        let mut surf = CellSurf::new(width, height);

        surf.fill_bg(WHITE);

        Self {
            style: Style::default(),
            surf,
            horizontal_scroll: Scroll::new(
                0,
                width,
                1
            ),
            vertical_scroll: Scroll::new(
                0,
                height,
                1
            ),
            view_width: width,
            view_height: height,
            on_left_click: Callback::noop(),
            on_left_click_release: Callback::noop()
        }
    }

    pub fn from_surf(surf: CellSurf) -> Self {

        let width = surf.width;
        let height = surf.height;

        Self {
            style: Style::default(),
            surf,
            horizontal_scroll: Scroll::new(
                0,
                width,
                1
            ),
            vertical_scroll: Scroll::new(
                0,
                height,
                1
            ),
            view_width: width,
            view_height: height,
            on_left_click: Callback::noop(),
            on_left_click_release: Callback::noop()
        }
    }
}

impl<M: Model> HorizontalScrollWidget<M> for Canvas<M> {

    fn get_horizontal_scroll(&self) -> &Scroll<M, HorizontalScrollbar<M>> {
        &self.horizontal_scroll
    }

    fn get_horizontal_scroll_mut(&mut self) -> &mut Scroll<M, HorizontalScrollbar<M>> {
        &mut self.horizontal_scroll
    }

    fn reload_horizontal_scroll_size(&mut self) {
        self.horizontal_scroll.total_size = self.surf.width;
    }
}

impl<M: Model> VerticalScrollWidget<M> for Canvas<M> {

    fn get_vertical_scroll(&self) -> &Scroll<M, VerticalScrollbar<M>> {
        &self.vertical_scroll
    }

    fn get_vertical_scroll_mut(&mut self) -> &mut Scroll<M, VerticalScrollbar<M>> {
        &mut self.vertical_scroll
    }

    fn reload_vertical_scroll_size(&mut self) {
        self.vertical_scroll.total_size = self.surf.height;
    }
}