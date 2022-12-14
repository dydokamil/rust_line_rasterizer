// https://stackoverflow.com/questions/5186939/algorithm-for-drawing-a-4-connected-line
use std::cmp::Ordering;

mod tests;

pub type Point = (isize, isize);

pub struct LineRasterizer {
    error: isize,
    step: isize,
    dx: isize,
    dy: isize,
    x_cursor: isize,
    y_cursor: isize,
    sgn_x: isize,
    sgn_y: isize,
}

impl LineRasterizer {
    pub fn new(start: Point, end: Point) -> Self {
        let dx = (end.0 - start.0).abs();
        let dy = (end.1 - start.1).abs();

        Self {
            error: 0,
            step: 0,
            dx,
            dy,
            x_cursor: start.0,
            y_cursor: start.1,
            sgn_x: if start.0 < end.0 { 1 } else { -1 },
            sgn_y: if start.1 < end.1 { 1 } else { -1 },
        }
    }
}

impl Iterator for LineRasterizer {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        let e1 = self.error + self.dy;
        let e2 = self.error - self.dx;

        match e1.abs().cmp(&e2.abs()) {
            Ordering::Less => {
                self.x_cursor += self.sgn_x;
                self.error = e1;
                self.step += 1;
            }
            Ordering::Greater => {
                self.y_cursor += self.sgn_y;
                self.error = e2;
                self.step += 1;
            }
            Ordering::Equal => {
                self.x_cursor += self.sgn_x;
                self.y_cursor += self.sgn_y;
                self.error += self.dy - self.dx;
                self.step += 2;
            }
        }

        if self.step >= self.dx + self.dy {
            return None;
        }

        Some((self.x_cursor, self.y_cursor))
    }
}
