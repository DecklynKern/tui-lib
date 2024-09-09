use glium::winit::keyboard::KeyCode as KC;
use glium::winit::event::MouseButton as MB;

#[derive(Clone)]
pub struct FrameContext {
    pub dt_seconds: f32,
    pub mouse_pos: MousePosition,
    pub held_keys: [bool; NUM_KEYS],
    pub screen_width: usize,
    pub screen_height: usize
}

impl FrameContext {
    pub fn key_held(&self, key: Key) -> bool {
        self.held_keys[key as usize]
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Up,
    Left,
    Down,
    Right,
    Enter,
    Space,
    LShift,
    RShift,
    Backspace,
    Escape,
    Tab,
    LBracket,
    RBracket,
    LCtrl,
    LAlt,
    RAlt,
    Unknown
}

impl Key {

    pub fn from_key_code(key_code: KC) -> Self {
        match key_code {
            KC::KeyA => Self::A,
            KC::KeyB => Self::B,
            KC::KeyC => Self::C,
            KC::KeyD => Self::D,
            KC::KeyE => Self::E,
            KC::KeyF => Self::F,
            KC::KeyG => Self::G,
            KC::KeyH => Self::H,
            KC::KeyI => Self::I,
            KC::KeyJ => Self::J,
            KC::KeyK => Self::K,
            KC::KeyL => Self::L,
            KC::KeyM => Self::M,
            KC::KeyN => Self::N,
            KC::KeyO => Self::O,
            KC::KeyP => Self::P,
            KC::KeyQ => Self::Q,
            KC::KeyR => Self::R,
            KC::KeyS => Self::S,
            KC::KeyT => Self::T,
            KC::KeyU => Self::U,
            KC::KeyV => Self::V,
            KC::KeyW => Self::W,
            KC::KeyX => Self::X,
            KC::KeyY => Self::Y,
            KC::KeyZ => Self::Z,
            KC::Digit0 => Self::Num0,
            KC::Digit1 => Self::Num1,
            KC::Digit2 => Self::Num2,
            KC::Digit3 => Self::Num3,
            KC::Digit4 => Self::Num4,
            KC::Digit5 => Self::Num5,
            KC::Digit6 => Self::Num6,
            KC::Digit7 => Self::Num7,
            KC::Digit8 => Self::Num8,
            KC::Digit9 => Self::Num9,
            KC::ArrowUp => Self::Up,
            KC::ArrowLeft => Self::Left,
            KC::ArrowDown => Self::Down,
            KC::ArrowRight => Self::Right,
            KC::Enter => Self::Enter,
            KC::Space => Self::Space,
            KC::Backspace => Self::Backspace,
            KC::ShiftLeft => Self::LShift,
            KC::ShiftRight => Self::RShift,
            KC::Escape => Self::Escape,
            KC::Tab => Self::Tab,
            KC::BracketLeft => Self::LBracket,
            KC::BracketRight => Self::RBracket,
            KC::ControlLeft => Self::LCtrl,
            KC::AltLeft => Self::LAlt,
            KC::AltRight => Self::RAlt,
            _ => Self::Unknown
        }
    }

    pub fn to_chr(self) -> Option<char> {

        // need to implement shift key stuff
        
        Some(match self {
            Self::A => 'a',
            Self::B => 'b',
            Self::C => 'c',
            Self::D => 'd',
            Self::E => 'e',
            Self::F => 'f',
            Self::G => 'g',
            Self::H => 'h',
            Self::I => 'i',
            Self::J => 'j',
            Self::K => 'k',
            Self::L => 'l',
            Self::M => 'm',
            Self::N => 'n',
            Self::O => 'o',
            Self::P => 'p',
            Self::Q => 'q',
            Self::R => 'r',
            Self::S => 's',
            Self::T => 't',
            Self::U => 'u',
            Self::V => 'v',
            Self::W => 'w',
            Self::X => 'x',
            Self::Y => 'y',
            Self::Z => 'z',
            Self::Num0 => '0',
            Self::Num1 => '1',
            Self::Num2 => '2',
            Self::Num3 => '3',
            Self::Num4 => '4',
            Self::Num5 => '5',
            Self::Num6 => '6',
            Self::Num7 => '7',
            Self::Num8 => '8',
            Self::Num9 => '9',
            Self::Space => ' ',
            Self::LBracket => '[',
            Self::RBracket => ']',
            _ => return None
        })
    }
}

pub const NUM_KEYS: usize = Key::Unknown as usize + 1;

pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other
}

impl MouseButton {

    pub fn from_winit(button: glium::winit::event::MouseButton) -> Self {
        match button {
            MB::Left => Self::Left,
            MB::Middle => Self::Middle,
            MB::Right => Self::Right,
            _ => Self::Other
        }
    }
}

#[derive(Clone)]
pub struct MousePosition {
    pub cell_x: i32,
    pub cell_y: i32,
    pub cell_x_rel: i32,
    pub cell_y_rel: i32,
    pub pixel_x: f64,
    pub pixel_y: f64,
    pub pixel_x_rel: f64,
    pub pixel_y_rel: f64
}

impl MousePosition {
    
    pub fn new() -> Self {
        Self{
            cell_x: 0,
            cell_y: 0,
            cell_x_rel: 0,
            cell_y_rel: 0,
            pixel_x: 0.0,
            pixel_y: 0.0,
            pixel_x_rel: 0.0,
            pixel_y_rel: 0.0
        }
    }

    pub fn apply_cell_offset(&mut self, offset_x: i32, offset_y: i32) {
        self.cell_x += offset_x;
        self.cell_y += offset_y;
        self.pixel_x += (offset_x * 8) as f64;
        self.pixel_x += (offset_y * 8) as f64;
    }

    pub fn with_cell_offset(&self, offset_x: i32, offset_y: i32) -> Self {
        Self{
            cell_x: self.cell_x + offset_x,
            cell_y: self.cell_y + offset_y,
            cell_x_rel: self.cell_x_rel,
            cell_y_rel: self.cell_y_rel,
            pixel_x: self.pixel_x + (offset_x * 8) as f64,
            pixel_y: self.pixel_y + (offset_y * 8) as f64,
            pixel_x_rel: self.pixel_x_rel,
            pixel_y_rel: self.pixel_y_rel,
        }
    }

    pub fn new_mouse_pos(&mut self, (mut new_mouse_x, mut new_mouse_y): (f64, f64)) {

        new_mouse_x -= 4.0;
        new_mouse_y -= 4.0;
        
        self.pixel_x_rel = self.pixel_x - new_mouse_x;
        self.pixel_y_rel = self.pixel_y - new_mouse_y;

        self.pixel_x = new_mouse_x;
        self.pixel_y = new_mouse_y;

        let cell_x = new_mouse_x as i32 / 8;
        let cell_y = new_mouse_y as i32 / 8;

        self.cell_x_rel = cell_x - self.cell_x;
        self.cell_y_rel = cell_y - self.cell_y;

        self.cell_x = cell_x;
        self.cell_y = cell_y;
    
    }
}

pub enum Event {
    KeyDown(Key),
    KeyRepeat(Key),
    KeyUp(Key),
    MouseDown(MouseButton),
    MouseUp(MouseButton)
}