use super::super::style::*;
use super::super::model::*;
use super::widget::*;

use crate::core::cell::*;
use crate::core::event::*;
use crate::core::surface::*;

struct Node {
    pub name: Box<str>,
    pub children: Vec<Node>,
    pub expanded: bool,
    pub x: usize,
    pub y: usize
}

impl Node {

    pub fn new(name: Box<str>) -> Self {
        Self {
            name,
            children: Vec::new(),
            expanded: false,
            x: 0,
            y: 0
        }
    }

    pub fn get_child(&mut self, mut keys: Vec<&str>) -> Option<&mut Self> {

        let key = match keys.pop() {
            Some(key) => key,
            None => return None
        };
        
        for child in &mut self.children {
            
            if *child.name == *key {
                return if keys.is_empty() {
                    Some(child)
                }
                else {
                    child.get_child(keys)
                };
            }
        }

        None
        
    }

    pub fn get_child_by_y(&mut self, y: usize) -> Option<&mut Self> {

        if !self.expanded {
            return None;
        }

        for child in &mut self.children {

            if child.y == y {
                return Some(child);
            }

            if let Some(node) = child.get_child_by_y(y) {
                return Some(node);
            }
        }

        None

    }

    pub fn get_keys_by_y(&self, y: usize) -> Option<Vec<Box<str>>> {

        if y == self.y {
            return Some(Vec::new());
        }

        if !self.expanded {
            return None
        }

        for child in &self.children {
            if let Some(mut keys) = child.get_keys_by_y(y) {
                keys.push(child.name.clone().into());
                return Some(keys);
            }
        }

        None

    }

    pub fn update_position(&mut self, x: usize, mut y: usize) -> usize {

        self.x = x;
        self.y = y;

        if !self.expanded {
            return y;
        }

        for child in &mut self.children {
            y = child.update_position(x + 2, y + 1);
        }

        y + 1

    }

    pub fn draw(&self, surf: &mut SubSurface, selected_item_y: usize) {

        let name = self.name.clone();

        if selected_item_y == self.y {
            surf.fill_range_bg(self.x..surf.width, self.y..self.y + 1, CYAN);
        }

        if self.children.is_empty() {
            surf.write_line(self.x, self.y, name);
            return;        
        }

        surf.write_line(self.x + 1, self.y, name);

        if !self.expanded {
            surf.set_char(self.x, self.y, '+');
            return;
        }

        surf.set_char(self.x, self.y, '-');

        for child in &self.children {
            child.draw(surf, selected_item_y);
        }
    }
}

pub struct Treeview<M: Model> {
    style: Style,
    width: usize,
    height: usize,
    root: Node,
    selected_item_y: usize,
    pub on_item_select: Callback<M, Vec<Box<str>>>
}

impl<M: Model> Widget<M> for Treeview<M> {

    fn get_style(&self) -> &Style {
        &self.style
    }

    fn get_inner_width(&self) -> usize {
        self.width
    }

    fn get_inner_height(&self) -> usize {
        self.height
    }

    fn handle_left_click(&mut self, click_x: i32, click_y: i32) -> Option<(Callback<M, (i32, i32)>, (i32, i32))> {

        let x = click_x as usize;
        let y = click_y as usize;

        if let Some(node) = self.get_node_by_y(y) {
            if x == node.x && !node.children.is_empty() {
                node.expanded = !node.expanded;
                self.root.update_position(0, 0);
            }
            else if x >= node.x {
                self.selected_item_y = y;
            }
        }

        None
    
    }

    fn handle_key_press(&mut self, key: Key) -> Option<KeyCallback<M>> {
        
        match key {
            Key::Up => {
                if self.selected_item_y != 0 {        
                    if self.get_node_by_y(self.selected_item_y - 1).is_some() {
                        self.selected_item_y -= 1;
                    }
                }
            },
            Key::Down => {
                if self.get_node_by_y(self.selected_item_y + 1).is_some() {
                    self.selected_item_y += 1;
                }
            },
            Key::Left => {
                if let Some(node) = self.get_node_by_y(self.selected_item_y) {
                    node.expanded = false;
                    self.root.update_position(0, 0);
                }
            },
            Key::Right => {
                if let Some(node) = self.get_node_by_y(self.selected_item_y) {
                    node.expanded = true;
                    self.root.update_position(0, 0);
                }
            },
            _ => {}
        }
        
        None

    }

    fn render(&self, _context: &FrameContext, mut surf: SubSurface) {
        self.root.draw(&mut surf, self.selected_item_y);
    }
}

impl<M: Model> Treeview<M> {

    pub fn new(width: usize, height: usize, root_name: impl Into<Box<str>>) -> Self {
        Self {
            style: Style::default(),
            width,
            height,
            root: Node::new(root_name.into()),
            selected_item_y: 0,
            on_item_select: Callback::noop()
        }
    }

    fn clear(&mut self) {
        self.root.children.clear();
        self.root.expanded = false;
    }

    fn get_node(&mut self, mut keys: Vec<&str>) -> Option<&mut Node> {

        if keys.is_empty() {
            return Some(&mut self.root);
        }

        keys.reverse();

        self.root.get_child(keys)

    }

    fn get_node_by_y(&mut self, y: usize) -> Option<&mut Node> {

        if y == 0 {
            return Some(&mut self.root);
        }

        self.root.get_child_by_y(y)

    }

    fn get_keys_by_y(&self, y: usize) -> Option<Vec<Box<str>>> {
        
        let keys = self.root.get_keys_by_y(y);
        
        if let Some(mut vec) = keys {
            vec.reverse();
            return Some(vec);
        }

        None
        
    }

    pub fn get_selected_keys(&self) -> Option<Vec<Box<str>>> {
        self.get_keys_by_y(self.selected_item_y)
    }

    pub fn add_node(&mut self, keys: Vec<&str>, name: impl Into<Box<str>>) {
        if let Some(node) = self.get_node(keys) {
            node.children.push(Node::new(name.into()));
            self.root.update_position(0, 0);
        }
    }

    // pub fn set_to_folder(&mut self, folder: &Vec<File>, extension_filter: Option<Box<str>>) {
            
    //     self.clear();

    //     self.root.name = "/".into();

    //     fn add_folder(tree: &mut Treeview<impl Model>, folder: &Vec<File>, current_folder: &mut Vec<Box<str>>, extension_filter: &Option<Box<str>>) {

    //         for file in folder {

    //             if let Some(filter) = &extension_filter {
    //                 if file.file_name.contains('.') && !file.file_name.ends_with(filter.as_ref()) {
    //                     continue;
    //                 }
    //             }

    //             tree.add_node((&*current_folder).to_str_list(), file.file_name.clone());

    //             if let FileType::Folder(files) = &file.file_type {

    //                 current_folder.push(file.file_name.clone().into());

    //                 add_folder(tree, files, current_folder, extension_filter);

    //                 current_folder.pop();

    //             }

    //         }
    //     }

    //     add_folder(self, folder, &mut Vec::new(), &extension_filter);

    // }

    // pub fn set_to_file_tree(&mut self, extension_filter: Option<Box<str>>) {
    //     unsafe {
    //         if let FileType::Folder(folder) = &FILE_TREE.file_type {
    //             self.set_to_folder(folder, extension_filter);
    //         } 
    //     }
    // }
}