use std::ops;

pub const PI: f32 = 3.141592653589;
pub const TAU: f32 = PI * 2.0;

pub type Point<T> = (T, T);
pub type Rect<T> = (T, T, T, T);
pub type Circle<T> = (Point<T>, T);
pub type Line<T> = (Point<T>, Point<T>);

pub fn add<T>((x1, y1): Point<T>, (x2, y2): Point<T>) -> Point<T> 
where T: ops::Add<Output = T> {
    (x1 + x2, y1 + y2)
}

pub fn sub<T>((x1, y1): Point<T>, (x2, y2): Point<T>) -> Point<T> 
where T: ops::Sub<Output = T> {
    (x1 - x2, y1 - y2)
}

pub fn scale<T>((x, y): Point<T>, scale: T) -> Point<T>
where T: ops::Mul<Output = T> + Copy {
    (x * scale, y * scale)
}

pub fn dot<T>((x1, y1): Point<T>, (x2, y2): Point<T>) -> T
where T: Copy + ops::Mul<Output = T> + ops::Add<Output = T> {
    x1 * x2 + y1 * y2   
}

pub fn dist((x1, y1): Point<f32>, (x2, y2): Point<f32>) -> f32 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

pub fn slope<T>((x, y): Point<T>) -> T
where T: ops::Div<Output = T> {
    y / x
}

pub fn dist_squared<T>(p1: Point<T>, p2: Point<T>) -> T 
where T: Copy + ops::Mul<Output = T> + ops::Add<Output = T> + ops::Sub<Output = T> {
    let delta = sub(p1, p2);
    dot(delta, delta)
}

pub fn point_line_collision<T>(point: Point<T>, (line_point_1, line_point_2): Line<T>) -> bool 
where T: Copy + PartialOrd + ops::Sub<Output = T> + ops::Div<Output = T> {
    
    if slope(sub(line_point_2, line_point_1)) != slope(sub(point, line_point_1)) {
        return false;
    }

    let (min_x, max_x) = if line_point_1.0 > line_point_2.0 {
        (line_point_2.0, line_point_1.0)
    } else {
        (line_point_1.0, line_point_2.0)
    };

    min_x < point.0 && point.0 < max_x 

}

pub fn point_rect_collision<T>(
    (point_x, point_y): Point<T>,
    (rect_x, rect_y, rect_width, rect_height): Rect<T>
) -> bool 
where T: PartialOrd + ops::Add<Output = T> {
    point_x >= rect_x &&
    point_x < rect_x + rect_width &&
    point_y >= rect_y &&
    point_y < rect_y + rect_height
}

pub fn rect_collision<T>(
    (rect1_x, rect1_y, rect1_width, rect1_height): Rect<T>,
    (rect2_x, rect2_y, rect2_width, rect2_height): Rect<T>
) -> bool
where T: PartialOrd + ops::Add<Output = T> + Copy {
    rect1_x + rect1_width > rect2_x &&
    rect1_x <= rect2_x + rect2_width &&
    rect1_y + rect1_height > rect2_y &&
    rect1_y <= rect2_y + rect2_height
}

pub fn point_circle_collision<T>(
    point: Point<T>,
    (circle_centre, circle_radius): Circle<T>
) -> bool
where T: Copy + PartialOrd + ops::Mul<Output = T> + ops::Add<Output = T> + ops::Sub<Output = T> {
    dist_squared(point, circle_centre) <= circle_radius * circle_radius
}

pub fn line_circle_collision<T>(
    (point1, point2): Line<T>,
    (circle_centre, circle_radius): Circle<T>
) -> bool
where T: Copy + PartialOrd + ops::Mul<Output = T> + ops::Add<Output = T> + ops::Sub<Output = T> + ops::Div<Output = T> {

    if 
    point_circle_collision(point1, (circle_centre, circle_radius)) ||
    point_circle_collision(point2, (circle_centre, circle_radius)) {
            return true;
    }

    let line_delta = sub(point2, point1);

    let len_sq = dist_squared(point1, point2);
    let norm = dot(sub(circle_centre, point1), line_delta) / len_sq;

    let closest = add(point1, scale(line_delta, norm));

    if !point_line_collision(closest, (point1, point2)) {
        return false;
    }

    let distance_sq = dist_squared(closest, circle_centre);

    distance_sq < circle_radius * circle_radius

}

pub fn circle_collision<T>(
    (circle1_centre, circle1_radius): Circle<T>,
    (circle2_centre, circle2_radius): Circle<T>
) -> bool
where T: Copy + PartialOrd + ops::Mul<Output = T> + ops::Add<Output = T> + ops::Sub<Output = T> {
    let radius_sum = circle1_radius + circle2_radius;
    dist_squared(circle1_centre, circle2_centre) <= radius_sum * radius_sum
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Orthogonal {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3
}

impl Orthogonal {
    
    pub fn from_angle(mut angle: f32) -> Orthogonal {
        
        angle = (angle % TAU) * 4.0;

        if angle < PI {
            Orthogonal::Right

        } else if angle < 3.0 * PI {
            Orthogonal::Up

        } else if angle < 5.0 * PI {
            Orthogonal::Left
        
        } else if angle < 7.0 * PI {
            Orthogonal::Down
        
        } else {
            Orthogonal::Right
        }   
    }
}

#[derive(Clone)]
pub enum HalfAngle {
    Up = 0,
    UpRight = 1,
    Right = 2,
    DownRight = 3,
    Down = 4,
    DownLeft = 5,
    Left = 6,
    UpLeft = 7
}

impl HalfAngle {

    pub fn from_angle(mut angle: f32) -> HalfAngle {
        
        angle = (angle % TAU) * 8.0;

        if angle < PI {
            HalfAngle::Right
        
        } else if angle < 3.0 * PI {
            HalfAngle::UpRight

        } else if angle < 5.0 * PI {
            HalfAngle::Up

        } else if angle < 7.0 * PI {
            HalfAngle::UpLeft

        } else if angle < 9.0 * PI {
            HalfAngle::Left

        } else if angle < 11.0 * PI {
            HalfAngle::DownLeft
        
        } else if angle < 13.0 * PI / 8.0 {
            HalfAngle::Down

        } else if angle < 15.0 * PI / 8.0 {
            HalfAngle::DownRight
        
        } else {
            HalfAngle::Right
        }   
    }
}