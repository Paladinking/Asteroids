use std::ops::{Add, Sub};
use sdl2::render::{Canvas, RenderTarget};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        return Point {x: self.x + rhs.x, y: self.y + rhs.y};
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Self::Output {
        return Point {x: self.x - rhs.x, y: self.y - rhs.y};
    }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {x, y}
    }
}

pub struct Polygon {
    pub points: Vec<Point>
}

impl Polygon {
    pub fn render<T: RenderTarget>(&self, canvas : &mut Canvas<T>) -> Result<(), String>{
        if self.points.len() == 0 {
            return Ok(());
        }

        let middle = self.points.iter().fold(Point {x: 0.0, y: 0.0}, |p, a| p + *a);

        let vx = self.points.iter().map(|p| p.x as i16).collect::<Vec<_>>();
        let vy = self.points.iter().map(|p| p.y as i16).collect::<Vec<_>>();

        canvas.aa_polygon(&vx, &vy, Color::RGB(0, 0, 0))?;

        return Ok(());
    }

    pub fn contains_point(&self, p : Point) -> bool {
        if self.points.len() <= 2 {
            return false;
        }

        let mut p2: Point = *self.points.last().unwrap();
        let mut iter = self.points.iter();
        let mut c = false;

        let mut p1 = iter.next().unwrap();
        loop {

            if ((p1.y <= p.y && p.y < p2.y) || (p2.y <= p.y && p.y < p1.y)) &&
                (p.x < (p2.x - p1.x) * (p.y - p1.y) / (p2.y - p1.y) + p1.x) {
                    c = !c;
            }

            let next = iter.next();
            if let Some(p) = next {
                p2 = *p1;
                p1 = p;
            } else {
                break;
            }
        }

        return c;
    }
}
