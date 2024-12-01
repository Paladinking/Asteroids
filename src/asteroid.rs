use crate::shapes::line_intersects;
use crate::shapes::Polygon;
use crate::shapes::Point;
use rand;
use sdl2::render::{Canvas, RenderTarget};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use crate::{WINDOW_WIDTH, WINDOW_HEIGHT};

pub struct Asteroid {
    pub shape: Polygon,
    dx : f64,
    dy: f64,

    rot: f64,
    mass: f64,
    inertia: f64
}

const WINDOW_MARGIN: f64 = 250.0;
const MIN_AREA: f64 = 500.0;


impl Asteroid {
    pub fn new(mut poly: Polygon, pos: Point, dx: f64, dy: f64, rot: f64) -> Asteroid {
        poly.shift(pos.x, pos.y);
        let mass = poly.area();
        let inertia = mass * mass.sqrt() * 10.0;
        Asteroid{shape: poly, dx, dy, rot, mass, inertia}
    }

    pub fn small(&self) -> bool {
        return self.shape.area() < MIN_AREA;
    }

    pub fn split(&mut self, p1: Point, p2: Point) -> Option<Asteroid> {
        let mut it = self.shape.lines().enumerate();

        loop {
            let (ix, (pa1, pa2)) = it.next()?;
            if let Some(p) = line_intersects(pa1, pa2, p1, p2) {
                loop {
                    let (ix2, (pb1, pb2)) = it.next()?;
                    if let Some(q) = line_intersects(pb1, pb2, p1, p2) {
                        let mut v = Vec::new();
                        let mut v2 = Vec::new();
                        v2.push(p);
                        for p in &self.shape.points[0..ix] {
                            v.push(*p);
                        }
                        for p in &self.shape.points[ix..(ix2)] {
                            v2.push(*p);
                        }
                        v2.push(q);
                        v.push(p);
                        v.push(q);
                        for p in &self.shape.points[(ix2)..] {
                            v.push(*p);
                        }
                        assert!(v.len() > 2);
                        assert!(v2.len() > 2);
                        self.shape.points = v;
                        let poly = Polygon::new(v2);
                        let mut diff = self.shape.centre - poly.centre;
                        diff = diff / diff.len();
                        let v = Point::new(self.dx, self.dy) - 50.0 * diff;
                        self.dx += 50.0 * diff.x;
                        self.dy += 50.0 * diff.y;
                        self.shape.calc_centre();
                        self.shape.calc_radius();
                        return Some(Asteroid::new(poly, Point::new(0.0, 0.0), v.x, v.y, self.rot));
                    }
                }
            }
        }
    }

    pub fn get_randomized(approx_radius: f64, pos: Point, vel: Point) -> Asteroid {
        let num_points = (rand::random::<f64>() * 6.0) as i64 + 5;
        let mut points: Vec<Point> = vec![];
        let centre = Point::new(0.0, 0.0);
        let safe_margin: f64 = 2.0;
        for i in 0..num_points {
            points.push(Point::new(
                0.0,
                approx_radius * (safe_margin + rand::random::<f64>()) / (safe_margin + 1.0)).rotated(
                     2.0 * 3.1415 * i as f64 / num_points as f64, centre
                    )
                ) 
        }
        let mut i = 0;
        'make_convex: loop {
            let first_point = points[i % points.len()];
            let second_point = points[(i+1) % points.len()];
            let third_point = points[(i+2) % points.len()];
            let is_to_left = ((third_point.x - first_point.x) * (first_point.y - second_point.y) - (first_point.y - third_point.y) * (second_point.x - first_point.x)).is_sign_negative();
            if is_to_left {
                points.remove((i+1) % points.len());
                // print!(" got removed!!!\n");
                i = 0;
            } else {
                i += 1;
                // print!(" is kept...\n");
            }
            if points.len() < 4 || i >= points.len() {
                break 'make_convex;
            }
        }
        let dx = vel.x;
        let dy = vel.y;
        return Asteroid::new(Polygon::new(points), pos, dx, dy, 0.0);
    }

    pub fn collides(&self, other: &Asteroid) -> Option<(Point, Point, Point)> {
        return self.shape.get_collision(&other.shape);
    }

    pub fn solve_polygon_collision(&mut self, other: &mut Asteroid, p: Point, shift: Point, normal: Point) {
        const ELASTICITY: f64 = 0.5;

        self.shape.shift(shift.x, shift.y);
        let centre_a = self.shape.centre;
        let ra = p - centre_a;

        let centre_b = other.shape.centre;
        let rb = p - centre_b;

        let vrel = Point::new(self.dx - self.rot * ra.y -  (other.dx - other.rot * rb.y),
                              self.dy + self.rot * ra.x - (other.dy + other.rot * rb.x)).dot(normal);

        let ra_x_n = ra.x * normal.y - ra.y * normal.x;
        let rb_x_n = rb.x * normal.y - rb.y * normal.x;
        let cross_thing_a = Point::new(ra.y * -ra_x_n, ra.x * ra_x_n);
        let cross_thing_b = Point::new(rb.y * -rb_x_n, rb.x * rb_x_n);

        let norm_part = normal.dot(cross_thing_a / self.inertia + cross_thing_b / other.inertia);

        let j = -(ELASTICITY + 1.0) * vrel / (1.0 / self.mass + 1.0 / other.mass + norm_part);
        let vel_a = Point::new(self.dx, self.dy) + j * normal / self.mass;
        let vel_b = Point::new(other.dx, other.dy) - j * normal / other.mass;
        self.rot = self.rot + j * ra_x_n / self.inertia;
        self.dx = vel_a.x;
        self.dy = vel_a.y;

        other.rot = other.rot - j * rb_x_n / other.inertia;
        other.dx = vel_b.x;
        other.dy = vel_b.y;
    }

    fn solve_wall_collision(&mut self, offset: Point, mut p : Point, normal: Point) {
        const ELASTICITY: f64 = 0.9;

        self.shape.shift(offset.x, offset.y);
        p = p + offset;

        let centre_a = self.shape.centre;
        let ra = p - centre_a;

        let vrel = Point::new(self.dx - self.rot * ra.y, self.dy + self.rot * ra.x).dot(normal);

        let r_x_n = ra.x * normal.y - ra.y * normal.x;
        let cross_thing = Point::new(ra.y * -r_x_n, ra.x * r_x_n);

        let j = -(ELASTICITY + 1.0) * vrel / (1.0 / self.mass + normal.dot(cross_thing) / self.inertia);

        let vel =  Point::new(self.dx, self.dy) + j * normal / self.mass;
        self.rot = self.rot + j * r_x_n / self.inertia;
        self.dx = vel.x;
        self.dy = vel.y;
    }

    pub fn tick(&mut self, mut delta: f64) {
        delta = delta ;
        self.shape.shift(self.dx * delta, self.dy * delta);

        self.shape.rotate(self.rot * delta);

        for i in 0..self.shape.points.len() {
            let p = self.shape.points[i];
            if p.x < -WINDOW_MARGIN {
                self.solve_wall_collision(Point::new(-WINDOW_MARGIN-p.x, 0.0), p, Point::new(1.0, 0.0));
            } else if p.x > WINDOW_WIDTH + WINDOW_MARGIN {
                self.solve_wall_collision(Point::new(WINDOW_WIDTH + WINDOW_MARGIN - p.x, 0.0), p, Point::new(-1.0, 0.0));
            } else if p.y < -WINDOW_MARGIN {
                self.solve_wall_collision(Point::new(0.0, -WINDOW_MARGIN-p.y), p, Point::new(0.0, 1.0));
            } else if p.y > WINDOW_HEIGHT + WINDOW_MARGIN {
                self.solve_wall_collision(Point::new(0.0, WINDOW_HEIGHT + WINDOW_MARGIN - p.y), p, Point::new(0.0, -1.0));
            }
        }
    }

    pub fn render<T: RenderTarget>(&self, canvas : &mut Canvas<T>) -> Result<(), String>{
        if self.shape.corners() == 0 {
            return Ok(());
        }

        let vx = self.shape.points.iter().map(|p| p.x as i16).collect::<Vec<_>>();
        let vy = self.shape.points.iter().map(|p| p.y as i16).collect::<Vec<_>>();

        canvas.aa_polygon(&vx, &vy, Color::RGB(0xff, 0xff, 0xff))?;


        // let mut i = 0;
        // 'draw_lines: loop {
        //     let first_point = self.shape.points[i % self.shape.points.len()];
        //     let second_point = self.shape.points[(i+1) % self.shape.points.len()];
        //     let third_point = self.shape.points[(i+2) % self.shape.points.len()];
        //     let is_to_right = ((third_point.x - first_point.x) * (first_point.y - second_point.y) - (first_point.y - third_point.y) * (second_point.x - first_point.x)).is_sign_positive();
        //     canvas.aa_line(first_point.x as i16, first_point.y as i16, third_point.x as i16, third_point.y as i16, if is_to_right {Color::RGB(0xff, 0x00, 0x00)} else {Color::RGB(0x00, 0xff, 0x00)})?;
        //     i += 1;
        //     if i > self.shape.points.len()-1 {
        //         break 'draw_lines;
        //     }
        // }
        return Ok(());
    }

    // pub fn contains_point(&self, p: Point) -> bool {
    //     return self.shape.contains_point(p);
    // }
}
