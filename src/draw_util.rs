use super::cell::*;
use super::math::*;

const RGB_TO_COLOUR: &[u8] = include_bytes!("rgbtocolour.txt");

pub fn rgb_to_cell(r: u8, g: u8, b: u8) -> Cell {

    let offset = ((((r as u64) << 16) + ((g as u64) << 8) + b as u64) * 2) as usize;

    Cell::new(
        char::from_u32(RGB_TO_COLOUR[offset] as u32).unwrap(),
        Colour::from_num(RGB_TO_COLOUR[offset + 1] >> 4),
        Colour::from_num(RGB_TO_COLOUR[offset + 1] & 0xF)
    )
}

pub fn pixel_line((mut start, mut end): Line<i32>) -> Vec<Point<i32>> {

    if start.0 > end.0 {
        std::mem::swap(&mut start, &mut end);
    }

    let dir = if start.1 > end.1 {
        -1
    }
    else {
        1
    };

    let x_diff = end.0 - start.0;
    let y_diff = (start.1 - end.1).abs();
    let x_main_axis = x_diff > y_diff;

    let mut points = Vec::with_capacity(x_diff.max(y_diff) as usize);
    let (mut x, mut y) = start;

    while (x, y) != end {

        let flat_point = if x_main_axis {
            (x + 1, y)
        }
        else {
            (x, y + dir)
        };

        let diagonal_point = (x + 1, y + dir);

        todo!("finish pixel line");

    }

    points

}

pub fn pixel_line_filtered(line: Line<i32>, x_cutoff: usize, y_cutoff: usize) -> Vec<Point<usize>> {
    pixel_line(line).iter().map(
        |(x, y)| (*x as usize, *y as usize)
    ).filter(
        |(x, y)| *x < x_cutoff && *y < y_cutoff
    ).collect()
}

pub fn pixel_circle((centre, radius): Circle<i32>) -> Vec<Point<i32>> {
    todo!("finish pixel circle");
}

pub fn pixel_circle_filled(circle: Circle<i32>) -> Vec<Point<i32>> {
    
    let mut final_pixels = Vec::new();
    let centre_x = circle.0.0;

    for (x, y) in pixel_circle(circle) {

        if x <= centre_x {

            for fill_x in x..(2 * centre_x - x + 1) {
                final_pixels.push((fill_x, y));
            }
        }
    }

    final_pixels

}