use super::widgets::*;
use super::model::*;

use crate::core::core::event::*;
use crate::core::core::surface::*;

pub type SimpleRoot = Root<EmptyModel>;

pub struct Root<M: Model + 'static> {
    title: String,
    frame: Frame<M>,
    model: M
}

impl<M: Model + 'static> WindowEventHandler for Root<M> {

    fn start(&mut self) -> Vec<ProcessResponse> {
        self.frame.setup();
        Vec::new()
    }

    fn handle_event(&mut self, event: Event, _context: &Context) {

        if self.model.preprocess_event(&event) {
            return;
        }
        
        match event {
            Event::MouseDown(MouseButton::Left) => {

                let mouse_pos = &context.mouse_pos;

                self.frame.try_focus(mouse_pos.cell_x, mouse_pos.cell_y);

                if let Some(
                    (callback,
                    (click_rel_x, click_rel_y)
                )) = self.frame.handle_left_click(
                    mouse_pos.cell_x,
                    mouse_pos.cell_y
                ) {
                    callback.call(&mut self.model, (click_rel_x, click_rel_y))
                }
            },
            Event::MouseUp(MouseButton::Left) => {
                if let Some(callback) = self.frame.handle_left_click_release() {
                    callback.call(&mut self.model, ());
                }
            },
            Event::MouseDown(MouseButton::Right) => {

                let mouse_pos = &context.mouse_pos;

                if let Some(
                    (callback,
                        (click_rel_x, click_rel_y)
                    )
                ) = self.frame.handle_right_click(
                    mouse_pos.cell_x,
                    mouse_pos.cell_y
                ) {
                    callback.call(&mut self.model, (click_rel_x, click_rel_y))
                }
            },
            Event::KeyDown(key) => {
                if let Some(callback) = self.frame.handle_key_press(key) {
                    callback.call(&mut self.model, key);
                }
            }
            _ => ()
        }
    }

    fn remove_focus(&mut self) {
        self.frame.remove_focus();
    }

    fn update(&mut self, context: &Context) -> Vec<ProcessResponse> {
        
        self.frame.update(&mut self.model, context);
        
        match self.model.update(context) {
            ModelResponse::None => Vec::new(),
            ModelResponse::NewPopup(popup_type) => vec![
                ProcessResponse::SpawnChild(popup_type.construct(context))
            ],
            ModelResponse::ChangeTitle(new_title) => {
                self.title = new_title;
                Vec::new()
            }
            ModelResponse::RunCmd(command) => vec![ProcessResponse::RunCmd(command)],
            // implement eventually
            ModelResponse::Quit => vec![ProcessResponse::QuitProcess]
        }
    }

    fn draw(&self, context: &Context, surf: Subsurface) {
        (&self.frame as &dyn Widget<M>).render_full(context, surf);
    }
}

impl<M: Model> Root<M> {

    pub fn with_model(model: M, title: impl Into<String>, frame: Frame<M>) -> Root<M> {
        Self {
            title: title.into(),
            frame,
            model
        }
    }
}

impl SimpleRoot {
    pub fn new(title: impl Into<String>, frame: Frame<EmptyModel>) -> SimpleRoot {
        SimpleRoot::with_model(EmptyModel{}, title, frame)
    }
}