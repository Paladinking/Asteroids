use core::f64;
use std::ops::{Add, Div, Mul, Sub, Neg};

#[derive(Debug)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64
}

impl Rectangle {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Rectangle {
        Rectangle {x, y, w, h}
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        return Point {x: -self.x, y: -self.y};
    }
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

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        return rhs * self;
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

    pub fn dot(&self, other: Point) -> f64 {
        return other.x * self.x + other.y * self.y;
    }

    pub fn len_squared(&self) -> f64 {
        return self.dot(*self);
    }

    pub fn len(&self) -> f64 {
        return self.len_squared().sqrt();
    }

    pub fn dist_squared(&self, other: Point) -> f64 {
        return (*self - other).len_squared();
    }
    
    pub fn dist(&self, other: Point) -> f64 {
        return (*self - other).len();
    }
}

// pa1 -> pa2 line segment
// pb1 -> pb2 infinite line
pub fn line_intersects(pa1: Point, pa2: Point, pb1: Point, pb2: Point) -> Option<Point> {
    let denom = (pa1.x - pa2.x) * (pb1.y - pb2.y) - (pa1.y - pa2.y) * (pb1.x - pb2.x);
    if denom == 0.0 {
        return None;
    }
    let t = ((pa1.x - pb1.x) * (pb1.y - pb2.y) - (pa1.y - pb1.y) * (pb1.x - pb2.x)) / denom;
    let u = -((pa1.x - pa2.x) * (pa1.y - pb1.y) - (pa1.y - pa2.y) * (pa1.x - pb1.x)) / denom;
    if 0.0 <= t && t <= 1.0 && 0.0 <= u {
        return Some(pa1 + t * (pa2 - pa1));
    }
    return None;
}

pub fn line_segment_intersect(pa1: Point, pa2: Point, pb1: Point, pb2: Point) -> Option<Point> {
    let denom = (pa1.x - pa2.x) * (pb1.y - pb2.y) - (pa1.y - pa2.y) * (pb1.x - pb2.x);
    if denom == 0.0 {
        return None;
    }
    let t = ((pa1.x - pb1.x) * (pb1.y - pb2.y) - (pa1.y - pb1.y) * (pb1.x - pb2.x)) / denom;
    let u = -((pa1.x - pa2.x) * (pa1.y - pb1.y) - (pa1.y - pa2.y) * (pa1.x - pb1.x)) / denom;

    if 0.0 <= t && t <= 1.0 && 0.0 <= u && u <= 1.0 {
        return Some(pa1 + t * (pa2 - pa1));
    }

    return None;
}

pub struct Polygon {
    pub points: Vec<Point>,
    pub centre: Point,
    pub radius: f64,
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
    pub fn new(points: Vec<Point>) -> Polygon {
        let mut poly = Polygon{points, centre: Point::new(0.0, 0.0), radius: 0.0};
        poly.calc_centre();
        poly.calc_radius();
        return poly;
    }

    pub fn calc_centre(&mut self) {
        let mut centre = Point::new(0.0, 0.0);
        for p in self.points.iter() {
            centre = centre + *p;
        }
        self.centre = centre / self.points.len() as f64;
    }

    pub fn calc_radius(&mut self) {
        let mut radius: f64 = 0.0;
        for p in self.points.iter() {
            let len_sqrd = (p.x-self.centre.x) * (p.x-self.centre.x) + (p.y-self.centre.y) * (p.y-self.centre.y);
            if radius < len_sqrd {
                radius = len_sqrd;
            }
        }
        self.radius = radius.sqrt();
    }

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

    pub fn bounds(&self) -> Rectangle {
        assert!(self.points.len() > 0);

        let mut it = self.points.iter();

        let mut min = *it.next().unwrap();
        let mut max = min;

        for p in it {
            if p.x < min.x {
                min.x = p.x;
            } else if p.x > max.x {
                max.x = p.x;
            }
            if p.y < min.y {
                min.y = p.y;
            } else if p.y > max.y {
                max.y = p.y;
            }
        }

        return Rectangle::new(min.x, min.y, max.x - min.x, max.y - min.y);
    }

    // Point and normal
    fn closest_point(&self, p: Point) -> (Point, Point) {
        let mut closest = Point::new(0.0, 0.0);
        let mut normal = Point::new(0.0, 0.0);
        let mut dist = f64::MAX;

        for (p1, p2) in self.lines() {
            let a = p - p1;
            let b = p2 - p1;
            let proj = (a.dot(b) / b.dot(b)) * b;
            let close = p1 + proj;
            let d = p.dist_squared(close);
            if d < dist {
                dist = d;
                let norm = Point::new(b.y, -b.x);
                let p = norm + close;
                let d = - norm.dot(close);
                if (p.dot(norm) + d).signum() != (self.centre.dot(norm) + d).signum() {
                    normal = norm;
                } else {
                    normal = Point::new(-norm.x, -norm.y);
                }

                closest = close;
            }
        }

        return (closest, normal);
    }

    // point and normal
    // p2 should be iside, p1 outside
    fn get_intersect(&self, p1: Point, p2: Point) -> Option<(Point, Point)> {
        for (p3, p4) in self.lines() {
            if let Some(p) = line_segment_intersect(p1, p2, p3, p4) {
                //println!("{:?}, {:?}, {:?}, {:?}", p1, p2, p3, p4);
                let delta = p4 - p3;
                let rotated = Point::new(-delta.y, delta.x);
                let out = p1 - p;
                
                let normal = (rotated.dot(out) / rotated.dot(rotated)) * rotated;
                return Some((p, normal / normal.len()));
            } 
        }
        return None;
    }

    fn check_collision(&self, other: &Polygon) -> Option<(Point, Point, Point)> {
        for p in &other.points {
            if self.contains_point(*p) {
                let centre = other.centre;
                if let Some((col, normal)) = self.get_intersect(*p, centre) {
                    return Some((col, col -*p, normal));
                } else {
                    return None;
                }
            }
        }
        return None;
    } 

    // Returns tuple: (Point of collision, How to move self to not intersect, normal)
    pub fn get_collision(&self, other: &Polygon) -> Option<(Point, Point, Point)> {
        if let Some(res) = self.check_collision(other) {
            return Some(res);
        }
        if let Some((col, offset, normal)) = other.check_collision(self) {
            return Some((col, -offset, normal));
        }
        return None;
    }

    pub fn rotate(&mut self, rad: f64) {
        for p in self.points.iter_mut() {
            *p = p.rotated(rad, self.centre);
        }
        self.calc_centre();
    }

    // pub fn centre(&self) -> Point {
    //     assert!(self.points.len() > 0);

    //     let len = self.points.len() as f64;

    //     return self.points.iter().fold(Point {x: 0.0, y: 0.0}, |p, a| p + *a) / len;
    // }

    pub fn shift(&mut self, dx: f64, dy: f64) {
        let delta = Point::new(dx, dy);
        for p in self.points.iter_mut() {
            *p = *p + delta;
        }
        self.calc_centre();
    }

    pub fn area(&self) -> f64 {
        let mut sum = 0.0;
        for (p1, p2) in self.lines() {
            sum += p1.x * p2.y - p1.y * p2.x;
        }
        return 0.5 * sum.abs();
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
