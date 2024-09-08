use super::definitions::*;

const RGB_TO_COLOUR: &'static [u8] = include_bytes!("rgbtocolour.txt");

pub fn rgb_to_cell(r: u8, g: u8, b: u8) -> Cell {

    let offset = ((((r as u64) << 16) + ((g as u64) << 8) + b as u64) * 2) as usize;

    Cell::new(
        char::from_u32(RGB_TO_COLOUR[offset] as u32).unwrap(),
        Colour::from_num(RGB_TO_COLOUR[offset + 1] >> 4),
        Colour::from_num(RGB_TO_COLOUR[offset + 1] & 0xF)
    )
}

// pub fn pixel_line((start, end): Line<i32>) -> Vec<Point<i32>> {
//     bterm::line2d_bresenham(
//         bterm::Point::from_tuple(start),
//         bterm::Point::from_tuple(end)
//     ).iter().map(
//         |point| (point.x, point.y)
//     ).collect()
// }

// pub fn pixel_line_filtered(line: Line<i32>, x_cutoff: usize, y_cutoff: usize) -> Vec<Point<usize>> {
//     pixel_line(line).iter().map(
//         |(x, y)| (*x as usize, *y as usize)
//     ).filter(
//         |(x, y)| *x < x_cutoff && *y < y_cutoff
//     ).collect()
// }

// pub fn pixel_circle((centre, radius): Circle<i32>) -> Vec<Point<i32>> {
//     bterm::BresenhamCircle::new(
//         bterm::Point::from_tuple(centre),
//         radius
//     ).map(
//         |point| (point.x, point.y)
//     ).collect()
// }

// pub fn pixel_circle_filled(circle: Circle<i32>) -> Vec<Point<i32>> {
    
//     let mut final_pixels = Vec::new();
//     let centre_x = circle.0.0;

//     for (x, y) in pixel_circle(circle) {

//         if x <= centre_x {

//             for fill_x in x..(2 * centre_x - x + 1) {
//                 final_pixels.push((fill_x, y));
//             }
//         }
//     }

//     final_pixels

// }