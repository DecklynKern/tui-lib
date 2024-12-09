use crate::core::cell::*;
use crate::core::surface::*;

#[derive(PartialEq)]
pub enum Border {
    None,
    Bevel(Colour, Colour),
    BevelHalf(Colour, Colour),
    Solid(Colour),
    Line(Colour, Colour),
    DoubleLine(Colour, Colour),
    Dotted(Colour, Colour),
    Dashed(Colour, Colour)
}

pub const BORDER_RAISED_BEVEL: Border = Border::Bevel(WHITE, DARK_GREY);
pub const BORDER_SUNKEN_BEVEL: Border = Border::Bevel(DARK_GREY, WHITE);

pub const BORDER_RAISED_BEVEL_HALF: Border = Border::BevelHalf(WHITE, DARK_GREY);
pub const BORDER_SUNKEN_BEVEL_HALF: Border = Border::BevelHalf(DARK_GREY, WHITE);

pub const BORDER_LINE: Border = Border::Line(BLACK, GREY);
pub const BORDER_DOUBLE_LINE: Border = Border::DoubleLine(BLACK, GREY);
pub const BORDER_DOTTED: Border = Border::Dotted(BLACK, GREY);
pub const BORDER_DASHED: Border = Border::Dashed(BLACK, GREY);

pub struct Style {
    pub border_style: Border,
    pub text_colour: Colour,
    pub bg_colour: Colour,
    pub secondary_colour: Colour,
    pub highlight_fg_colour: Colour,
    pub highlight_bg_colour: Colour,
    pub pad_left: usize,
    pub pad_right: usize,
    pub pad_top: usize,
    pub pad_bottom: usize
}

impl Style {

    pub fn default() -> Style {
        Style {
            border_style: Border::None,
            text_colour: BLACK,
            bg_colour: GREY,
            secondary_colour: DARK_GREY,
            highlight_fg_colour: WHITE,
            highlight_bg_colour: BLACK,
            pad_left: 0,
            pad_right: 0,
            pad_top: 0,
            pad_bottom: 0
        }
    }

    pub fn set_pad_x(&mut self, pad: usize) {
        self.pad_left = pad;
        self.pad_right = pad;
    }

    pub fn set_pad_y(&mut self, pad: usize) {
        self.pad_top = pad;
        self.pad_bottom = pad;
    }

    pub fn set_pad(&mut self, pad: usize) {
        self.pad_left = pad;
        self.pad_right = pad;
        self.pad_top = pad;
        self.pad_bottom = pad;
    }

    pub fn get_widget_top_left(&self) -> (usize, usize) {
        if self.border_style == Border::None {
            (0, 0)
        }
        else {
            (1, 1)
        }
    }

    pub fn get_border_offset(&self) -> i32 {
        if self.border_style == Border::None {
            0
        }
        else {
            -1
        }
    }
    
    pub fn apply_border(&self, surf: &mut SubSurface) {

        let (top, left) = self.get_widget_top_left();

        // slight optimization
        if left == 0 && top == 0 && self.pad_right == 0 && self.pad_bottom == 0 {
            return;
        }

        let horizontal_inner = 1..(surf.width - 1);
        let horizontal_full = 0..surf.width;

        let vertical_inner = 1..(surf.height - 1);
        let _vertical_full = 0..surf.height;

        let left = 0..1;
        let right = (surf.width - 1)..surf.width;
        let top = 0..1;
        let bottom = (surf.height - 1)..surf.height;

        match self.border_style {
            Border::None => {}
            Border::BevelHalf(upper_colour, lower_colour) => {

                surf.fill_range_chr_and_fg(
                    horizontal_inner.clone(),
                    top,
                    LOWER_HALF_BLOCK,
                    upper_colour
                );

                surf.fill_range_chr_and_fg(
                    left,
                    vertical_inner.clone(),
                    RIGHT_HALF_BLOCK,
                    upper_colour
                );

                surf.fill_range_chr_and_fg(
                    horizontal_inner,
                    bottom,
                    UPPER_HALF_BLOCK,
                    lower_colour
                );

                surf.fill_range_chr_and_fg(
                    right,
                    vertical_inner,
                    LEFT_HALF_BLOCK,
                    lower_colour
                );
            }
            Border::Bevel(upper_colour, lower_colour) => {

                surf.fill_range(
                    horizontal_full.clone(),
                    top,
                    Cell::new(' ', BLACK, upper_colour)
                );

                surf.fill_range(
                    left,
                    vertical_inner.clone(),
                    Cell::new(' ', BLACK, upper_colour)
                );

                surf.fill_range(
                    horizontal_full,
                    bottom,
                    Cell::new(' ', BLACK, lower_colour)
                );

                surf.fill_range(
                    right,
                    vertical_inner,
                    Cell::new(' ', BLACK, lower_colour)
                );
            }
            Border::Solid(colour) => surf.fill_bg(colour),
            Border::Line(fg, bg) | Border::DoubleLine(fg, bg) | Border::Dashed(fg, bg) => {

                let pipe = if let Border::Line(_, _) = self.border_style {
                    SINGLE_PIPE
                }
                else if let Border::DoubleLine(_, _) = self.border_style {
                    DOUBLE_PIPE
                }
                else {
                    DASHED_SINGLE_PIPE
                };

                let mut cell = Cell::new(pipe[PIPE_RD], fg, bg);
                surf.set(left.start, top.start, cell);

                cell.chr = pipe[PIPE_LD];
                surf.set(right.start, top.start, cell);

                cell.chr = pipe[PIPE_UL];
                surf.set(right.start, bottom.start, cell);

                cell.chr = pipe[PIPE_UR];
                surf.set(left.start, bottom.start, cell);

                cell.chr = pipe[PIPE_LR];
                surf.fill_range(horizontal_inner.clone(), top, cell);
                surf.fill_range(horizontal_inner, bottom, cell);

                cell.chr = pipe[PIPE_UD];
                surf.fill_range(left, vertical_inner.clone(), cell);
                surf.fill_range(right, vertical_inner, cell);

            }
            Border::Dotted(fg, bg) => surf.fill(Cell::new('∙', fg, bg)),
        }
    }

}