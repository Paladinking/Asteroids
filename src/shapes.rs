use core::f64;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        return Point {x: self.x / rhs, y : self.y / rhs};
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        return Point {x: self.x * rhs, y: self.y * rhs};
    }
    
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

    pub fn rotated(&self, rad: f64, centre: Point) -> Point {
        let p = Point::new(self.x - centre.x, self.y - centre.y);

        let (sa, ca) = rad.sin_cos();

        let x = p.x * ca - p.y * sa;
        let y = p.y * ca + p.x * sa;

        return Point::new(x + centre.x, y + centre.y);
    }
}

pub struct Polygon {
    pub points: Vec<Point>
}

pub struct Lines<'a> {
    p: Point,
    it: std::slice::Iter<'a, Point>
}

impl <'a> Iterator for Lines<'a> {
    type Item = (Point, Point);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(p) = self.it.next() {
            let p1 = self.p;
            self.p = *p;
            return Some((p1, self.p));
        }
        return None;
    }
}

impl Polygon {
    pub fn corners(&self) -> usize {
        return self.points.len();
    }

    pub fn lines(&self) -> Lines {
        assert!(self.points.len() >= 3);
        return Lines {
            p: *self.points.last().unwrap(),
            it: self.points.iter()
        }
    }

    pub fn rotate(&mut self, rad: f64) {
        let centre = self.centre();
        for p in self.points.iter_mut() {
            *p = p.rotated(rad, centre);
        }
    }

    pub fn centre(&self) -> Point {
        assert!(self.points.len() > 0);

        let len = self.points.len() as f64;

        return self.points.iter().fold(Point {x: 0.0, y: 0.0}, |p, a| p + *a) / len;
    }

    pub fn shift(&mut self, dx: f64, dy: f64) {
        let delta = Point::new(dx, dy);
        for p in self.points.iter_mut() {
            *p = *p + delta;
        }
    }

    pub fn contains_point(&self, p : Point) -> bool {
        if self.points.len() <= 2 {
            return false;
        }

        let mut c = false;
        for (p2, p1) in self.lines() {
            if ((p1.y <= p.y && p.y < p2.y) || (p2.y <= p.y && p.y < p1.y)) &&
                (p.x < (p2.x - p1.x) * (p.y - p1.y) / (p2.y - p1.y) + p1.x) {
                    c = !c;
            }
        }

        return c;
    }
}
