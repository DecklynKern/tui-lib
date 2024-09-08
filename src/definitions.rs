pub const CODE_PAGE_STRING: &'static str = 
" ☺☻♥♦♣♠•◘○◙♂♀♪♫☼\
►◄↕‼¶§▬↨↑↓→←∟↔▲\
▼ !\"#$%&'()*+,-./\
0123456789:;<=>?\
@ABCDEFGHIJKLMNO\
PQRSTUVWXYZ[\\]^_\
`abcdefghijklmno\
pqrstuvwxyz{|}~⌂\
ÇüéâäàåçêëèïîìÄÅ\
ÉæÆôöòûùÿÖÜ¢£¥₧ƒ\
áíóúñÑªº¿⌐¬½¼¡«»\
░▒▓│┤╡╢╖╕╣║╗╝╜╛┐\
└┴┬├─┼╞╟╚╔╩╦╠═╬╧\
╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀\
αßΓπΣσµτΦΘΩδ∞φε∩\
≡±≥≤⌠⌡÷≈°∙·√ⁿ²■□";

pub const CODE_PAGE: [char; 256] = [' ', '☺', '☻', '♥', '♦', '♣', '♠', '•', '◘', '○', '◙', '♂', '♀', '♪', '♫', '☼', '►', '◄', '↕', '‼', '¶', '§', '▬', '↨', '↑', '↓', '→', '←', '∟', '↔', '▲', '▼', ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', '⌂', 'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å', 'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', '¢', '£', '¥', '₧', 'ƒ', 'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬', '½', '¼', '¡', '«', '»', '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐', '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧', '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀', 'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩', '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', '□'];

pub const TRANSPARENT_CHAR: char = ' ';

pub const FULL_BLOCK: char = '█';
pub const LOWER_HALF_BLOCK: char = '▄';
pub const UPPER_HALF_BLOCK: char = '▀';
pub const LEFT_HALF_BLOCK: char = '▌';
pub const RIGHT_HALF_BLOCK: char = '▐';

pub const QUARTER_SHADED_BLOCK: char = '░';
pub const HALF_SHADED_BLOCK: char = '▒';
pub const THREE_QUARTERS_SHADED_BLOCK: char = '▓';

pub const MIDDLE_BLOCK: char = '■';
pub const MIDDLE_BLOCK_OUTLINE: char = '□';

pub const SINGLE_PIPE: [char; 11] = ['│', '─', '└', '┌', '┐', '┘', '┴', '├', '┬', '┤', '┼'];
pub const DOUBLE_PIPE: [char; 11] = ['║', '═', '╚', '╔', '╗', '╝', '╩', '╠', '╦', '╣', '╬'];

pub const PIPE_UD: usize = 0;
pub const PIPE_LR: usize = 1;
pub const PIPE_UR: usize = 2;
pub const PIPE_RD: usize = 3;
pub const PIPE_LD: usize = 4;
pub const PIPE_UL: usize = 5;
pub const PIPE_ULR: usize = 6;
pub const PIPE_URD: usize = 7;
pub const PIPE_LRD: usize = 8;
pub const PIPE_ULD: usize = 9;
pub const PIPE_ULRD: usize = 10;

pub fn char_to_code_point(chr: char) -> u8 {
    match chr {
        ' ' => 0,
        '☺' => 1,
        '☻' => 2,
        '♥' => 3,
        '♦' => 4,
        '♣' => 5,
        '♠' => 6,
        '•' => 7,
        '◘' => 8,
        '○' => 9,
        '◙' => 10,
        '♂' => 11,
        '♀' => 12,
        '♪' => 13,
        '♫' => 14,
        '☼' => 15,
        '►' => 16,
        '◄' => 17,
        '↕' => 18,
        '‼' => 19,
        '¶' => 20,
        '§' => 21,
        '▬' => 22,
        '↨' => 23,
        '↑' => 24,
        '↓' => 25,
        '→' => 26,
        '←' => 27,
        '∟' => 28,
        '↔' => 29,
        '▲' => 30,
        '▼' => 31,
        ' ' => 32,
        '!' => 33,
        '"' => 34,
        '#' => 35,
        '$' => 36,
        '%' => 37,
        '&' => 38,
        '\'' => 39,
        '(' => 40,
        ')' => 41,
        '*' => 42,
        '+' => 43,
        ',' => 44,
        '-' => 45,
        '.' => 46,
        '/' => 47,
        '0' => 48,
        '1' => 49,
        '2' => 50,
        '3' => 51,
        '4' => 52,
        '5' => 53,
        '6' => 54,
        '7' => 55,
        '8' => 56,
        '9' => 57,
        ':' => 58,
        ';' => 59,
        '<' => 60,
        '=' => 61,
        '>' => 62,
        '?' => 63,
        '@' => 64,
        'A' => 65,
        'B' => 66,
        'C' => 67,
        'D' => 68,
        'E' => 69,
        'F' => 70,
        'G' => 71,
        'H' => 72,
        'I' => 73,
        'J' => 74,
        'K' => 75,
        'L' => 76,
        'M' => 77,
        'N' => 78,
        'O' => 79,
        'P' => 80,
        'Q' => 81,
        'R' => 82,
        'S' => 83,
        'T' => 84,
        'U' => 85,
        'V' => 86,
        'W' => 87,
        'X' => 88,
        'Y' => 89,
        'Z' => 90,
        '[' => 91,
        '\\' => 92,
        ']' => 93,
        '^' => 94,
        '_' => 95,
        '`' => 96,
        'a' => 97,
        'b' => 98,
        'c' => 99,
        'd' => 100,
        'e' => 101,
        'f' => 102,
        'g' => 103,
        'h' => 104,
        'i' => 105,
        'j' => 106,
        'k' => 107,
        'l' => 108,
        'm' => 109,
        'n' => 110,
        'o' => 111,
        'p' => 112,
        'q' => 113,
        'r' => 114,
        's' => 115,
        't' => 116,
        'u' => 117,
        'v' => 118,
        'w' => 119,
        'x' => 120,
        'y' => 121,
        'z' => 122,
        '{' => 123,
        '|' => 124,
        '}' => 125,
        '~' => 126,
        '⌂' => 127,
        'Ç' => 128,
        'ü' => 129,
        'é' => 130,
        'â' => 131,
        'ä' => 132,
        'à' => 133,
        'å' => 134,
        'ç' => 135,
        'ê' => 136,
        'ë' => 137,
        'è' => 138,
        'ï' => 139,
        'î' => 140,
        'ì' => 141,
        'Ä' => 142,
        'Å' => 143,
        'É' => 144,
        'æ' => 145,
        'Æ' => 146,
        'ô' => 147,
        'ö' => 148,
        'ò' => 149,
        'û' => 150,
        'ù' => 151,
        'ÿ' => 152,
        'Ö' => 153,
        'Ü' => 154,
        '¢' => 155,
        '£' => 156,
        '¥' => 157,
        '₧' => 158,
        'ƒ' => 159,
        'á' => 160,
        'í' => 161,
        'ó' => 162,
        'ú' => 163,
        'ñ' => 164,
        'Ñ' => 165,
        'ª' => 166,
        'º' => 167,
        '¿' => 168,
        '⌐' => 169,
        '¬' => 170,
        '½' => 171,
        '¼' => 172,
        '¡' => 173,
        '«' => 174,
        '»' => 175,
        '░' => 176,
        '▒' => 177,
        '▓' => 178,
        '│' => 179,
        '┤' => 180,
        '╡' => 181,
        '╢' => 182,
        '╖' => 183,
        '╕' => 184,
        '╣' => 185,
        '║' => 186,
        '╗' => 187,
        '╝' => 188,
        '╜' => 189,
        '╛' => 190,
        '┐' => 191,
        '└' => 192,
        '┴' => 193,
        '┬' => 194,
        '├' => 195,
        '─' => 196,
        '┼' => 197,
        '╞' => 198,
        '╟' => 199,
        '╚' => 200,
        '╔' => 201,
        '╩' => 202,
        '╦' => 203,
        '╠' => 204,
        '═' => 205,
        '╬' => 206,
        '╧' => 207,
        '╨' => 208,
        '╤' => 209,
        '╥' => 210,
        '╙' => 211,
        '╘' => 212,
        '╒' => 213,
        '╓' => 214,
        '╫' => 215,
        '╪' => 216,
        '┘' => 217,
        '┌' => 218,
        '█' => 219,
        '▄' => 220,
        '▌' => 221,
        '▐' => 222,
        '▀' => 223,
        'α' => 224,
        'ß' => 225,
        'Γ' => 226,
        'π' => 227,
        'Σ' => 228,
        'σ' => 229,
        'µ' => 230,
        'τ' => 231,
        'Φ' => 232,
        'Θ' => 233,
        'Ω' => 234,
        'δ' => 235,
        '∞' => 236,
        'φ' => 237,
        'ε' => 238,
        '∩' => 239,
        '≡' => 240,
        '±' => 241,
        '≥' => 242,
        '≤' => 243,
        '⌠' => 244,
        '⌡' => 245,
        '÷' => 246,
        '≈' => 247,
        '°' => 248,
        '∙' => 249,
        '·' => 250,
        '√' => 251,
        'ⁿ' => 252,
        '²' => 253,
        '■' => 254,
        '□' => 255,
        _ => 31 // ? 
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Colour {
    Black,
    DarkBlue,
    DarkGreen,
    DarkCyan,
    DarkRed,
    DarkMagenta,
    DarkYellow,
    Grey,
    DarkGrey,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Yellow,
    White,
    Transparent
}

pub const BLACK: Colour = Colour::Black;
pub const DARK_BLUE: Colour = Colour::DarkBlue;
pub const DARK_GREEN: Colour = Colour::DarkGreen;
pub const DARK_CYAN: Colour = Colour::DarkCyan;
pub const DARK_RED: Colour = Colour::DarkRed;
pub const DARK_MAGENTA: Colour = Colour::DarkMagenta;
pub const DARK_YELLOW: Colour = Colour::DarkYellow;
pub const GREY: Colour = Colour::Grey;
pub const DARK_GREY: Colour = Colour::DarkGrey;
pub const BLUE: Colour = Colour::Blue;
pub const GREEN: Colour = Colour::Green;
pub const CYAN: Colour = Colour::Cyan;
pub const RED: Colour = Colour::Red;
pub const MAGENTA: Colour = Colour::Magenta;
pub const YELLOW: Colour = Colour::Yellow;
pub const WHITE: Colour = Colour::White;
pub const TRANSPARENT: Colour = Colour::Transparent;

impl Colour {

    pub fn from_char(chr: char) -> Colour {
        match chr {
            '0' => BLACK,
            '1' => DARK_BLUE,
            '2' => DARK_GREEN,
            '3' => DARK_CYAN,
            '4' => DARK_RED,
            '5' => DARK_MAGENTA,
            '6' => DARK_YELLOW,
            '7' => GREY,
            '8' => DARK_GREY,
            '9' => BLUE,
            'A' => GREEN,
            'B' => CYAN,
            'C' => RED,
            'D' => MAGENTA,
            'E' => YELLOW,
            'F' => WHITE,
            _ => TRANSPARENT,
        }
    }

    pub fn from_num(num: u8) -> Colour {
        match num {
            0x0 => BLACK,
            0x1 => DARK_BLUE,
            0x2 => DARK_GREEN,
            0x3 => DARK_CYAN,
            0x4 => DARK_RED,
            0x5 => DARK_MAGENTA,
            0x6 => DARK_YELLOW,
            0x7 => GREY,
            0x8 => DARK_GREY,
            0x9 => BLUE,
            0xA => GREEN,
            0xB => CYAN,
            0xC => RED,
            0xD => MAGENTA,
            0xE => YELLOW,
            0xF => WHITE,
            _ => TRANSPARENT,
        }
    }

    pub fn darken(self) -> Colour {
        match self {
            BLACK => BLACK,
            DARK_BLUE => BLACK,
            DARK_GREEN => BLACK,
            DARK_CYAN => BLACK,
            DARK_RED => BLACK,
            DARK_MAGENTA => BLACK,
            DARK_YELLOW => BLACK,
            GREY => DARK_GREY,
            DARK_GREY => BLACK,
            BLUE => DARK_BLUE,
            GREEN => DARK_GREEN,
            CYAN => DARK_CYAN,
            RED => DARK_RED,
            MAGENTA => DARK_MAGENTA,
            YELLOW => DARK_YELLOW,
            WHITE => GREY,
            TRANSPARENT => TRANSPARENT
        }
    }

    pub fn lighten(self) -> Colour {
        match self {
            BLACK => BLACK,
            DARK_BLUE => BLUE,
            DARK_GREEN => GREEN,
            DARK_CYAN => CYAN,
            DARK_RED => RED,
            DARK_MAGENTA => MAGENTA,
            DARK_YELLOW => YELLOW,
            GREY => WHITE,
            DARK_GREY => GREY,
            BLUE => WHITE,
            GREEN => WHITE,
            CYAN => WHITE,
            RED => WHITE,
            MAGENTA => WHITE,
            YELLOW => WHITE,
            WHITE => WHITE,
            TRANSPARENT => TRANSPARENT
        }
    }

    pub fn invert(self) -> Colour {
        match self {
            BLACK => WHITE,
            DARK_BLUE => YELLOW,
            DARK_GREEN => MAGENTA,
            DARK_CYAN => RED,
            DARK_RED => CYAN,
            DARK_MAGENTA => GREEN,
            DARK_YELLOW => BLUE,
            GREY => DARK_GREY,
            DARK_GREY => GREY,
            BLUE => DARK_YELLOW,
            GREEN => DARK_MAGENTA,
            CYAN => DARK_RED,
            RED => DARK_CYAN,
            MAGENTA => DARK_GREEN,
            YELLOW => DARK_BLUE,
            WHITE => BLACK,
            TRANSPARENT => TRANSPARENT
        }
    }
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub chr: char,
    pub fg: Colour,
    pub bg: Colour
}

impl Cell {

    pub fn new(chr: char, fg: Colour, bg: Colour) -> Self {
        Self{chr, fg, bg}
    }

    pub fn black() -> Self {
        Self{
            chr: ' ',
            fg: WHITE,
            bg: BLACK
        }
    }

    pub fn transparent() -> Self {
        Self{
            chr: TRANSPARENT_CHAR,
            fg: TRANSPARENT,
            bg: TRANSPARENT
        }
    }

    pub fn clear() -> Self {
        Self::new(' ', TRANSPARENT, TRANSPARENT)
    }

    pub fn lighten(self) -> Self {
        Self::new(self.chr, self.fg.lighten(), self.bg.lighten())
    }

    pub fn darken(self) -> Self{
        Self::new(self.chr, self.fg.darken(), self.bg.darken())
    }

    pub fn invert(self) -> Self {
        Self::new(self.chr, self.fg.invert(), self.bg.invert())
    }

    pub fn swap_fg_bg(self) -> Self {
        Self::new(self.chr, self.bg, self.fg)
    }
}