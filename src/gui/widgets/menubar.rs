use super::widget::*;
use super::super::style::*;
use super::super::model::*;

use crate::core::cell::*;
use crate::core::event::*;
use crate::core::math::*;
use crate::core::surface::*;

pub struct Menubar<M: Model> {
    style: Style,
    width: usize,
    drop_down_titles: Vec<String>,
    drop_down_items: Vec<Vec<(String, MouseCallback<M>)>>,
    selected_index: Option<(usize, i32, i32)>
}

impl<M: Model> Widget<M> for Menubar<M> {

    fn get_style(&self) -> &Style {
        &self.style
    }

    fn get_render_priority(&self) -> i32 {
        1
    }

    fn get_inner_width(&self) -> usize {
        self.width.max(self.drop_down_titles.iter().map(
            |title| title.chars().count()
        ).sum::<usize>() + self.drop_down_titles.len() - 1)
    }

    fn get_inner_height(&self) -> usize {
        1
    }

    fn init_max_width(&mut self, width: usize) {
        self.width = width;
    }

    fn try_focus(&mut self, click_x: i32, click_y: i32) -> bool {

        if let Some((idx, x, width)) = self.selected_index {
            click_y == 0 || point_rect_collision((click_x, click_y), (x, 1, width, self.drop_down_items[idx].len() as i32))
        }
        else {
            true
        }
    }

    fn remove_focus(&mut self) {
        self.selected_index = None;
    }

    fn handle_left_click(&mut self, click_x: i32, click_y: i32) -> Option<(MouseCallback<M>, (i32, i32))> {

        if click_y == 0 {

            if self.selected_index.is_some() {
                self.selected_index = None;
                return None;
            }

            self.selected_index = None;

            let mut x = 0;
            
            for (idx, title) in self.drop_down_titles.iter().enumerate() {

                let len = title.chars().count() as i32;
    
                if x <= click_x && (click_y) < x + len {
         
                    let width = self.drop_down_items[idx].iter().map(
                        |item| item.0.chars().count()
                    ).max().unwrap() as i32;

                    self.selected_index = Some((idx, x, width));
                }
    
                if idx == self.drop_down_titles.len() - 1 {
                    break;
                }
    
                x += len + 1;
            
            }
        }
        else if let Some((selected_idx, x, width)) = self.selected_index {

            let height = self.drop_down_items[selected_idx].len() as i32;

            let callback = point_rect_collision(
                (click_x, click_y),
                (x, 1, width, height)
            ).then(|| (self.drop_down_items[selected_idx][click_y as usize - 1].1.clone(), (click_x, click_y)));

            self.selected_index = None;

            return callback;

        }

        None
        
    }

    fn render(&self, context: &FrameContext, mut surf: SubSurface) {
        
        let mut x = 0;

        for (idx, title) in self.drop_down_titles.iter().enumerate() {

            let len = title.chars().count();
            
            surf.write_line(x, 0, title);

            if context.mouse_pos.cell_y == 0 && x <= context.mouse_pos.cell_x as usize && (context.mouse_pos.cell_x as usize) < x + len {
                surf.fill_range_bg(x..x + len, 0..1, CYAN);
            }

            if let Some((selected_idx, x, width)) = self.selected_index {

                if selected_idx == idx {

                    let items = &self.drop_down_items[idx];

                    let mouse_in_x_range = x <= context.mouse_pos.cell_x && context.mouse_pos.cell_x < x + width;

                    let xs = (x as usize)..(x + width) as usize;
                    surf.fill_range(xs.clone(), 1..items.len() + 1, Cell::new(' ', BLACK, GREY));

                    for (idx, (name, _)) in items.iter().enumerate() {

                        if mouse_in_x_range && context.mouse_pos.cell_y == idx as i32 + 1 {
                            surf.fill_range_bg(xs.clone(), idx + 1..idx + 2, CYAN);
                        }

                        surf.write_line(x as usize, idx + 1, name);

                    }
                }
            }

            if idx == self.drop_down_titles.len() - 1 {
                break;
            }

            x += len;
            surf.set_char(x, 0, '│');
            x += 1;
        
        }
    }
}

impl<M: Model + 'static> Menubar<M> {

    pub fn new<const L: usize>(drop_down_titles: [Box<str>; L], drop_down_items: [Vec<(Box<str>, fn(&mut M))>; L]) -> Menubar<M> {
        Menubar {
            style: Style::default(),
            width: 0,
            drop_down_titles: drop_down_titles.into_iter().map(|title| title.into()).collect(),
            drop_down_items: drop_down_items.into_iter().map(
                |drop_down| drop_down.into_iter().map(
                    |(string, func)| (string.into(), func.into())
                ).collect()
            ).collect(),
            selected_index: None
        }
    }
}

#[macro_export]
macro_rules! menubar {
    ($(($t: expr, $($i: expr, $f: expr),+)),+) => {
        {
            gui::Menubar::new(
                [$($t.into()),+],
                [$(vec![$(($i.into(), $f)),+]),+]
            )
        }
    }
}