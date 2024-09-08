use super::cell::*;

use std::{ops::Range, rc::Rc};

pub trait Surface {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn get_char(&self, x: usize, y: usize) -> char;
    fn get_fg(&self, x: usize, y: usize) -> Colour;
    fn get_bg(&self, x: usize, y: usize) -> Colour;
    fn set_char(&mut self, x: usize, y: usize, chr: char);
    fn set_fg(&mut self, x: usize, y: usize, fg: Colour);
    fn set_bg(&mut self, x: usize, y: usize, bg: Colour);
    fn fill(&mut self, cell: Cell);

    fn get(&self, x: usize, y: usize) -> Cell {
        Cell::new(self.get_char(x, y), self.get_fg(x, y), self.get_bg(x, y))
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        self.set_char(x, y, cell.chr);
        self.set_fg(x, y, cell.fg);
        self.set_bg(x, y, cell.bg);
    }

    fn blit(&mut self, clip: &CellSurf, start_x: i32, start_y: i32) {

        let width = self.get_width() as i32;
        let height = self.get_height() as i32;
    
        for y in 0..clip.height as i32 {
    
            let copy_y = start_y as i32 + y;

            if copy_y >= height {
                break;
            }

            if copy_y < 0 {
                continue;
            }

            for x in 0..clip.width as i32 {

                let copy_x = start_x as i32 + x;
    
                if copy_x >= width {
                    break;
                }

                if copy_x < 0 {
                    continue;
                }

                let c_x = copy_x as usize;
                let c_y = copy_y as usize;

                let copy_cell = clip.get(x as usize, y as usize);

                if copy_cell.chr != TRANSPARENT_CHAR {
                    self.set_char(c_x, c_y, copy_cell.chr);
                }

                if copy_cell.fg != TRANSPARENT {
                    self.set_fg(c_x, c_y, copy_cell.fg);
                }

                if copy_cell.bg != TRANSPARENT {
                    self.set_bg(c_x, c_y, copy_cell.bg);
                }
            }
        }
    }
}

impl dyn Surface {

    pub fn write_line<T: Into<String>>(&mut self, x: usize, y: usize, line: T) {

        if y >= self.get_height() {
            return;
        }

        let width = self.get_width();

        for (idx, chr) in line.into().chars().enumerate() {

            let cell_x = idx + x;

            if cell_x >= width {
                return
            }

            self.set_char(cell_x, y, chr);

        }
    }

    // make i32 sometime
    pub fn write_line_colour<T: Into<String>>(&mut self, x: usize, y: usize, line: T, fg: Colour, bg: Colour) {

        if y >= self.get_height() {
            return;
        }

        let width = self.get_width();

        for (idx, chr) in line.into().chars().enumerate() {

            let cell_x = idx + x;

            if cell_x >= width {
                return
            }

            self.set(cell_x, y, Cell::new(chr, fg, bg));

        }
    }

    pub fn write_lines<T: Into<String> + Clone>(&mut self, x: usize, y: usize, lines: &[T]) {
        for (y_offset, line) in lines.iter().enumerate() {
            self.write_line(x, y + y_offset, line.clone());
        }
    }

    pub fn write_lines_colour<T: Into<String> + Clone>(&mut self, x: usize, y: usize, lines: &[T], fg: Colour, bg: Colour) {
        for (y_offset, line) in lines.iter().enumerate() {
            self.write_line_colour(x, y + y_offset, line.clone(), fg, bg);
        }
    }
}

#[derive(Clone)]
pub struct ScreenSurface {
    pub width: usize,
    pub height: usize,
    char_data: Box<[u32]>,
    fg_data: Box<[u32]>,
    bg_data: Box<[u32]>
}

impl ScreenSurface {
    
    pub fn new(width: usize, height: usize) -> Self {
        Self::filled_with(width, height, Cell::black())
    }

    fn pack_fill(cell: Cell) -> (u32, u32, u32) {

        let char_code_point = char_to_code_point(cell.chr) as u32;
        let fg_code = cell.fg as u32;
        let bg_code = cell.bg as u32;

        (
            char_code_point << 24 | char_code_point << 16 | char_code_point << 8 | char_code_point,
            fg_code << 28 | fg_code << 24 | fg_code << 20 | fg_code << 16 | fg_code << 12 | fg_code << 8 | fg_code << 4 | fg_code,
            bg_code << 28 | bg_code << 24 | bg_code << 20 | bg_code << 16 | bg_code << 12 | bg_code << 8 | bg_code << 4 | bg_code
        )
    }

    pub fn filled_with(width: usize, height: usize, cell: Cell) -> Self {

        let num_cells = width * height;
        let (char_pack, fg_pack, bg_pack) = Self::pack_fill(cell);

        let char_data = vec![char_pack; num_cells / 4].into_boxed_slice();
        let fg_data = vec![fg_pack; num_cells / 8].into_boxed_slice();
        let bg_data = vec![bg_pack; num_cells / 8].into_boxed_slice();

        Self {
            width,
            height,
            char_data,
            fg_data,
            bg_data
        }
    }

    pub fn get_raw_slices(&self) -> (&[u32], &[u32], &[u32]) {
        (&self.char_data, &self.fg_data, &self.bg_data)
    }
}

impl Surface for ScreenSurface {

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }
    
    fn get_char(&self, x: usize, y: usize) -> char {
        panic!("No reading from screen surf");
    }
    
    fn get_fg(&self, x: usize, y: usize) -> Colour {
        panic!("No reading from screen surf");
    }
    
    fn get_bg(&self, x: usize, y: usize) -> Colour {
        panic!("No reading from screen surf");
    }

    fn set_char(&mut self, x: usize, y: usize, chr: char) {

        let code_point = char_to_code_point(chr) as u32;
        let cell_idx = y * self.width + x;
        let idx = cell_idx / 4;
        let bit_offset = (cell_idx % 4) * 8;

        self.char_data[idx] &= !(0b11111111 << bit_offset);
        self.char_data[idx] |= code_point << bit_offset;
    
    }

    fn set_fg(&mut self, x: usize, y: usize, fg: Colour) {
        
        assert_ne!(fg, TRANSPARENT);

        let code = fg as u32;
        let cell_idx = y * self.width + x;
        let idx = cell_idx / 8;
        let bit_offset = (cell_idx % 8) * 4;

        self.fg_data[idx] &= !(0b1111 << bit_offset);
        self.fg_data[idx] |= code << bit_offset; 

    }

    fn set_bg(&mut self, x: usize, y: usize, bg: Colour) {
        
        assert_ne!(bg, TRANSPARENT);

        let code = bg as u32;
        let cell_idx = y * self.width + x;
        let idx = cell_idx / 8;
        let bit_offset = (cell_idx % 8) * 4;

        self.bg_data[idx] &= !(0b1111 << bit_offset);
        self.bg_data[idx] |= code << bit_offset; 

    }

    fn fill(&mut self, cell: Cell) {

        let (char_pack, fg_pack, bg_pack) = Self::pack_fill(cell);

        self.char_data.fill(char_pack);
        self.fg_data.fill(fg_pack);
        self.bg_data.fill(bg_pack);

    }
}

#[derive(Clone)]
pub struct CellSurf {
    pub width: usize,
    pub height: usize,
    cells: Box<[Cell]>
}

impl Surface for CellSurf {
    
    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn get_char(&self, x: usize, y: usize) -> char {
        self.cells[self.get_idx(x, y)].chr
    }

    fn get_fg(&self, x: usize, y: usize) -> Colour {
        self.cells[self.get_idx(x, y)].fg
    }

    fn get_bg(&self, x: usize, y: usize) -> Colour {
        self.cells[self.get_idx(x, y)].bg
    }

    fn set_char(&mut self, x: usize, y: usize, chr: char) {
        self.cells[self.get_idx(x, y)].chr = chr;
    }

    fn set_fg(&mut self, x: usize, y: usize, fg: Colour) {
        self.cells[self.get_idx(x, y)].fg = fg;
    }

    fn set_bg(&mut self, x: usize, y: usize, bg: Colour) {
        self.cells[self.get_idx(x, y)].bg = bg;
    }

    fn fill(&mut self, cell: Cell) {
        self.cells = vec![cell; self.width * self.height].into_boxed_slice();
    }
}

impl CellSurf {

    pub fn new(width: usize, height: usize) -> Self {
        Self::filled_with(width, height, Cell::transparent())
    }

    pub fn filled_with(width: usize, height: usize, cell: Cell) -> Self {
        Self{
            width,
            height,
            cells: vec![cell; width * height].into_boxed_slice()
        }
    }

    pub fn from_file<P: Into<std::path::PathBuf>>(path: P) -> Option<Self> {
    
        let file_result = std::fs::File::open(path.into().to_str().unwrap());
    
        match file_result {
            Ok(mut file) => {
    
                let layer = &rexpaint::XpFile::read(&mut file).unwrap().layers[0];
    
                let mut surf = CellSurf::new(layer.width, layer.height);
    
                for y in 0..layer.height {
                    for x in 0..layer.width {
    
                        let cell = layer.cells[y + layer.height * x];
                        
                        surf.set(x, y, Cell::new(
                            codepage_437::CP437_WINGDINGS.decode(cell.ch as u8),
                            cell.fg.into(),
                            cell.bg.into()
                        ));
                    }
                }
    
                Some(surf)
    
            },
            Err(_) => None
        }
    }

    fn get_idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}
pub struct SubSurface<'a> {
    x: usize,
    y: usize,
    pub width: usize,
    pub height: usize,
    parent_surf: &'a mut dyn Surface
}

impl<'a> Surface for SubSurface<'a> {

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn get_char(&self, x: usize, y: usize) -> char {
        self.parent_surf.get_char(self.x + x, self.y + y)
    }

    fn get_fg(&self, x: usize, y: usize) -> Colour {
        self.parent_surf.get_fg(self.x + x, self.y + y)
    }
    
    fn get_bg(&self, x: usize, y: usize) -> Colour {
        self.parent_surf.get_bg(self.x + x, self.y + y)
    }

    fn get(&self, x: usize, y: usize) -> Cell {
        self.parent_surf.get(self.x + x, self.y + y)
    }

    fn set_char(&mut self, x: usize, y: usize, chr: char) {
        self.parent_surf.set_char(self.x + x, self.y + y, chr)
    }
    
    fn set_fg(&mut self, x: usize, y: usize, fg: Colour) {
        self.parent_surf.set_fg(self.x + x, self.y + y, fg)
    }
    
    fn set_bg(&mut self, x: usize, y: usize, bg: Colour) {
        self.parent_surf.set_bg(self.x + x, self.y + y, bg)
    }
    
    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        self.parent_surf.set(self.x + x, self.y + y, cell)
    }

    fn fill(&mut self, cell: Cell) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set(x, y, cell);
            }
        }
    }
}

impl<'a> SubSurface<'a> {

    pub fn fill_range_chr(&mut self, xs: Range<usize>, ys: Range<usize>, chr: char) {
        todo!()
    }

    pub fn fill_range_fg(&mut self, xs: Range<usize>, ys: Range<usize>, fg: Colour) {
        todo!()
    }

    pub fn fill_range_bg(&mut self, xs: Range<usize>, ys: Range<usize>, bg: Colour) {
        todo!()
    }

    pub fn fill_range_chr_and_fg(&mut self, xs: Range<usize>, ys: Range<usize>, chr: char, fg: Colour) {
        todo!()
    }

    pub fn fill_range(&mut self, xs: Range<usize>, ys: Range<usize>, cell: Cell) {
        todo!()
    }

    pub fn fill_chr(&mut self, chr: char) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_char(x, y, chr);
            }
        }
    }

    pub fn fill_fg(&mut self, fg: Colour) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_fg(x, y, fg);
            }
        }
    }

    pub fn fill_bg(&mut self, bg: Colour) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_bg(x, y, bg);
            }
        }
    }
}

pub struct Animation {
    pub surfaces: Vec<Rc<CellSurf>>,
    pub durations: Vec<u64>,
    pub total_duration: u64,
    pub timer: u64
}

impl Animation {
    
    pub fn new(surfaces: Vec<Rc<CellSurf>>, durations: Vec<u64>) -> Self {
        Self{
            surfaces,
            total_duration: durations.iter().sum(),
            durations,
            timer: 0
        }
    }

    pub fn from_surface(surface: Rc<CellSurf>) -> Self {
        Self{
            surfaces: vec![surface],
            durations: vec![1000],
            total_duration: 1000,
            timer: 0
        }
    }

    pub fn from_raw_surface(surface: CellSurf) -> Self {
        Self::from_surface(Rc::new(surface))
    }

    pub fn update(&mut self, dt: u64) {
        self.timer += dt;
        self.timer %= self.total_duration;
    }
    
    pub fn get_surf(&self) -> &CellSurf {

        let mut t = self.timer;

        for (surf, duration) in self.surfaces.iter().zip(&self.durations) {

            if *duration > t {
                return surf;
            }

            t -= duration;

        }

        // *should* not happen
        &self.surfaces[0]

    }
}

#[macro_export]
macro_rules! anim {
    ($c: expr, $($n: expr, $d: expr),+) => {
        {
            Animation::new(
                vec![$($c.get_surf_rc($n)),+],
                vec![$($d),+],
            )
        }
    }
}