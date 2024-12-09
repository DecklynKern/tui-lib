use crate::core::desktop::{Process, FileType, WindowApp};
use crate::core::desktop::gui::ModelResponse;
use crate::core::Context;

use super::*;

use std::any::Any;
use std::mem::swap;

use crate::core::event::*;

pub enum PopupType {
    YesNo(Box<str>),
    FileSelect(Option<Box<str>>),
    FileSaveAs
}

impl PopupType {
    pub fn construct(self, _context: &FrameContext) -> Box<dyn Process> {
        match self {
            Self::YesNo(message) => Box::new(
                yes_no_dialog(message)
            ),
            Self::FileSelect(extension_filter) => Box::new(
                file_select_dialog(extension_filter)
            ),
            Self::FileSaveAs => Box::new(
                file_save_as_dialog()
            )
        }
    }
}

pub enum PopupStatus {
    Waiting,
    Done(Option<Box<dyn Any>>)
}

pub struct BasicPopupModel {
    status: PopupStatus
}

impl Model for BasicPopupModel {

    fn quit(&mut self) -> Option<Box<dyn Any>> {
    
        match &mut self.status {
            PopupStatus::Waiting => None,
            PopupStatus::Done(response) => {
                let mut new_val: Option<Box<dyn Any>> = None;
                swap(response, &mut new_val);
                new_val
            }
        }
    }

    fn update(&mut self, _context: &FrameContext) -> ModelResponse {
        match self.status {
            PopupStatus::Waiting => ModelResponse::None,
            PopupStatus::Done(_) => ModelResponse::Quit
        }
    }
}

impl BasicPopupModel {

    pub fn new() -> Self {
        Self {
            status: PopupStatus::Waiting
        }
    }

    fn on_yes(&mut self) {
        self.status = PopupStatus::Done(Some(Box::new(true)));
    }

    fn on_no(&mut self) {
        self.status = PopupStatus::Done(Some(Box::new(false)));
    }
}

pub fn yes_no_dialog(message: Box<str>) -> Root<BasicPopupModel> {
    
    let mut frame = Frame::new();

    frame.add_widget(Label::new(message), 0, 0);

    let mut lower_frame = Frame::new();

    lower_frame.add_widget(
        Button::with_fn(
            "Yes",
            BasicPopupModel::on_yes
        ),
        0,
        0
    );

    lower_frame.add_widget(
        Button::with_fn(
            "No",
            BasicPopupModel::on_no
        ),
        1,
        0
    );

    frame.add_widget(lower_frame, 0, 1);

    Root::<BasicPopupModel>::with_model(
        BasicPopupModel::new(),
        " ",
        frame
    )
}

pub struct FileSelectModel {
    loaded_files: bool,
    status: PopupStatus,
    file_treeview: WidgetRef<Treeview<Self>>,
    extension_filter: Option<Box<str>>
}

impl Model for FileSelectModel {

    fn quit(&mut self) -> Option<Box<dyn Any>> {
    
        match &mut self.status {
            PopupStatus::Waiting => None,
            PopupStatus::Done(response) => {
                let mut new_val: Option<Box<dyn Any>> = None;
                swap(response, &mut new_val);
                new_val
            }
        }
    }

    fn update(&mut self, context: &FrameContext) -> ModelResponse {

        if !self.loaded_files {
            self.file_treeview.borrow_mut().set_to_file_tree(self.extension_filter.clone());
            self.loaded_files = true;
        }

        match self.status {
            PopupStatus::Waiting => ModelResponse::None,
            PopupStatus::Done(_) => ModelResponse::Quit
        }
    }
}

impl FileSelectModel {

    pub fn on_select(&mut self) {
        self.status = PopupStatus::Done(match self.file_treeview.borrow().get_selected_keys() {
            Some(keys) => Some(Box::new("files/".to_string() + keys.join("/").as_str())),
            None => None
        });
    }

    pub fn on_cancel(&mut self) {
        self.status = PopupStatus::Done(None)
    }
}

pub fn file_select_dialog(extension_filter: Option<Box<str>>) -> Root<FileSelectModel> {

    let mut frame = Frame::new();

    let treeview = Treeview::new(50, 30, "/");

    let mut button_frame = Frame::new();
    
    let cancel_button = Button::with_fn(
        "Cancel",
        FileSelectModel::on_cancel
    );
    let mut select_button = Button::with_fn(
        "Select",
        FileSelectModel::on_select
    );

    select_button.style.pad_left = 3;

    button_frame.add_widget(cancel_button, 0, 0);
    button_frame.add_widget(select_button, 1, 0);

    frame.add_widget(button_frame, 0, 1);

    let model = FileSelectModel{
        loaded_files: false,
        status: PopupStatus::Waiting,
        file_treeview: frame.add_widget(treeview, 0, 0),
        extension_filter
    };

    Root::<FileSelectModel>::with_model(
        model,
        "Select File",
        frame
    )
}

pub struct FileSaveAsModel {
    loaded_files: bool,
    status: PopupStatus,
    file_treeview: WidgetRef<Treeview<Self>>,
    file_name_entry: WidgetRef<Entry>
}

impl Model for FileSaveAsModel {

    fn quit(&mut self) -> Option<Box<dyn Any>> {
    
        match &mut self.status {
            PopupStatus::Waiting => None,
            PopupStatus::Done(response) => {
                let mut new_val: Option<Box<dyn Any>> = None;
                swap(response, &mut new_val);
                new_val
            }
        }
    }

    fn update(&mut self, context: &FrameContext) -> ModelResponse {

        if !self.loaded_files {
            self.file_treeview.borrow_mut().set_to_file_tree(None);
            self.loaded_files = true;
        }

        match self.status {
            PopupStatus::Waiting => ModelResponse::None,
            PopupStatus::Done(_) => ModelResponse::Quit
        }
    }
}

impl FileSaveAsModel {

    pub fn on_save(&mut self) {
        self.status = PopupStatus::Done(Some(Box::new("files".to_owned() + &self.file_name_entry.borrow().text)));
    }

    pub fn on_file_select(&mut self, keys: Vec<Box<str>>) {
        self.file_name_entry.borrow_mut().text = "/".to_string() + keys.join("/").as_str();
    }

    pub fn on_cancel(&mut self) {
        self.status = PopupStatus::Done(None)
    }
}

pub fn file_save_as_dialog() -> Root<FileSaveAsModel> {

    let mut frame = Frame::new();

    let mut treeview = Treeview::new(50, 30, "/");

    treeview.on_item_select.bind(FileSaveAsModel::on_file_select);

    let mut button_frame = Frame::new();
    
    let cancel_button = Button::with_fn(
        "Cancel",
        FileSaveAsModel::on_cancel
    );
    let mut save_button = Button::with_fn(
        "Save",
        FileSaveAsModel::on_save
    );

    save_button.style.pad_left = 3;

    let file_name_entry = Entry::new(15, "/");

    button_frame.add_widget(cancel_button, 1, 0);
    button_frame.add_widget(save_button, 2, 0);

    frame.add_widget(button_frame, 0, 1);

    let model = FileSaveAsModel{
        loaded_files: false,
        status: PopupStatus::Waiting,
        file_treeview: frame.add_widget(treeview, 0, 0),
        file_name_entry: frame.add_widget(file_name_entry, 0, 0)
    };

    Root::<FileSaveAsModel>::with_model(
        model,
        "Save File As",
        frame
    )
}