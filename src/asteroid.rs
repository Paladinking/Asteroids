use crate::shapes::Polygon;
use crate::shapes::Point;
use sdl2::render::{Canvas, RenderTarget};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;

pub struct Asteroid {
    shape: Polygon,
    dx : f64,
    dy: f64,

    rot: f64,
    mass: f64,
    inertia: f64
}

impl Asteroid {
    pub fn new(mut poly: Polygon, pos: Point, dx: f64, dy: f64, rot: f64) -> Asteroid {
        poly.shift(pos.x, pos.y);
        let mass = poly.area();
        let inertia = mass * mass.sqrt() * 10.0;
        Asteroid{shape: poly, dx, dy, rot, mass, inertia}
    }

    pub fn collides(&self, other: &Asteroid) -> Option<(Point, Point, Point)> {
        return self.shape.get_collision(&other.shape);
    }

    pub fn solve_polygon_collision(&mut self, other: &mut Asteroid, p: Point, shift: Point, normal: Point) {
        const ELASTICITY: f64 = 1.0;

        self.shape.shift(shift.x, shift.y);
        let centre_a = self.shape.centre();
        let ra = p - centre_a;

        let centre_b = other.shape.centre();
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

        let centre_a = self.shape.centre();
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
            if p.x < 0.0 {
                self.solve_wall_collision(Point::new(-p.x, 0.0), p, Point::new(1.0, 0.0));
            } else if p.x > 800.0 {
                self.solve_wall_collision(Point::new(800.0 - p.x, 0.0), p, Point::new(-1.0, 0.0));
            } else if p.y < 0.0 {
                self.solve_wall_collision(Point::new(0.0, -p.y), p, Point::new(0.0, 1.0));
            } else if p.y > 600.0 {
                self.solve_wall_collision(Point::new(0.0, 600.0 - p.y), p, Point::new(0.0, -1.0));
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

        return Ok(());
    }

    pub fn cotains_point(&self, p: Point) -> bool {
        return self.shape.contains_point(p);
    }
}
