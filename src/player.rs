use sdl2::render::{Canvas, RenderTarget};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use crate::asteroid::Asteroid;
use crate::shapes::{Point, Polygon};
use crate::{WINDOW_WIDTH, WINDOW_HEIGHT};

const ACCELERATION: f64 = 2000.0;
// const MAX_VEL: f64 = 250.0;
const FIRING_TIME: f64 = 0.25;
const LASER_LENGTH: f64 = 1000.0;

struct Laser {
    pos_start: Point,
    pos_end: Point,
    shape: Polygon,
}

impl Laser {
    fn new(pos_start : Point, pos_end : Point) -> Laser {
        Laser { pos_start, pos_end, shape: Polygon::new(vec![Point::new(-10.0, 10.0), Point::new(-10.0, -10.0), Point::new(10.0, -10.0), Point::new(10.0, 10.0)]) }
    }

    fn render<T: RenderTarget>(&mut self, canvas: &mut Canvas<T>, firing: bool) -> Result<(), String> {
        if firing && (self.pos_start.x - self.pos_end.x).abs() + (self.pos_start.y - self.pos_end.y).abs() > 0.0 {
            let rot = (self.pos_end.y - self.pos_start.y).atan2(self.pos_end.x - self.pos_start.x);
            let centre = Point::new(0.0, 0.0);
            self.shape.points[0] = Point::new(-10.0, 10.0).rotated(rot, centre);
            self.shape.points[1] = Point::new(-10.0, -10.0).rotated(rot, centre);
            self.shape.points[2] = Point::new(10.0, -10.0).rotated(rot, centre);
            self.shape.points[3] = Point::new(10.0, 10.0).rotated(rot, centre);
        }
        let vx = self.shape.points.iter().map(|p| (self.pos_start.x + p.x) as i16).collect::<Vec<_>>();
        let vy = self.shape.points.iter().map(|p| (self.pos_start.y + p.y) as i16).collect::<Vec<_>>();
        canvas.aa_polygon(&vx, &vy, Color::RGB(0xf0, 0xf0, 0xf0))?;
        if firing {
            canvas.aa_line(
                self.pos_start.x as i16,
                self.pos_start.y as i16,
                self.pos_end.x as i16,
                self.pos_end.y as i16,
                Color::RGB(0xff, 0x00, 0x00)
            )?;
        }

        return Ok(());
    }
}

pub struct Player {
    pub shape: Polygon,
    pos: Point,
    vel: Point,
    acc: f64,
    mov_dir: Vec<f64>, // [up, down, left, right]
    laser: Laser,
    firing: f64,
}

impl Player {
    pub fn new(pos: Point) -> Player {
        let mut shape = Polygon::new(vec![Point::new(-25.0, 45.0), Point::new(25.0, 0.0), Point::new(-25.0, -45.0)]);
        shape.shift(pos.x, pos.y);
        Player {shape, pos,
            vel: Point{ x: 0.0, y: 0.0 },
            acc: ACCELERATION,
            mov_dir: vec![0.0, 0.0, 0.0, 0.0],
            laser: Laser::new(Point{x: 0.0, y: 0.0}, Point{x: 0.0, y: 0.0}),
            firing: 0.0,
        }
    }

    pub fn render<T: RenderTarget>(&mut self, canvas: &mut Canvas<T>) -> Result<(), String> {
        if self.shape.corners() == 0 {
            return Ok(());
        }

        let vx = self.shape.points.iter().map(|p | p.x as i16).collect::<Vec<_>>();
        let vy = self.shape.points.iter().map(|p | p.y as i16).collect::<Vec<_>>();

        canvas.aa_polygon(&vx, &vy, Color::RGB(0xff, 0x00, 0x00))?;

        self.laser.render(canvas, self.firing > 0.0)?;

        return Ok(());
    }

    // pub fn contains_point(&self, p : Point) -> bool {
    //     return self.shape.contains_point(p);
    // }

    pub fn tick(&mut self, delta: f64) {
        let ddx = (self.mov_dir[1] - self.mov_dir[0]) * self.acc * delta;
        let ddy = (self.mov_dir[3] - self.mov_dir[2]) * self.acc * delta;
        self.vel.y += ddx;
        self.vel.x += ddy;

        let rot = self.vel.y.atan2(self.vel.x);

        let dx = self.vel.x * delta;
        let dy = self.vel.y * delta;
        self.pos = self.pos + Point::new(dx, dy);
        if self.pos.x < 0.0 {
            self.pos.x = 0.0;
            if self.vel.x < 0.0 {
                self.vel.x = 0.0;
            }
        } else if self.pos.x > WINDOW_WIDTH {
            self.pos.x = WINDOW_WIDTH;
            if self.vel.x > 0.0 {
                self.vel.x = 0.0;
            }
        }
        if self.pos.y < 0.0 {
            self.pos.y = 0.0;
            if self.vel.y < 0.0 {
                self.vel.y = 0.0;
            }
        } else if self.pos.y > WINDOW_HEIGHT {
            self.pos.y = WINDOW_HEIGHT;
            if self.vel.y > 0.0 {
                self.vel.y = 0.0;
            }
        }
        let mut shape = Polygon::new(vec![Point::new(-25.0, 45.0), Point::new(25.0, 0.0), Point::new(-25.0, -45.0)]);
        shape.shift(self.pos.x, self.pos.y);
        shape.rotate(rot);
        self.shape = shape;

        if self.firing > 0.0 {
            self.firing = (self.firing - delta).clamp(0.0, FIRING_TIME);
        } else {
            self.laser.pos_start = self.pos;
        }
    }

    pub fn set_mov_dir(&mut self, dir: usize, val: bool) {
        self.mov_dir[dir] = if val {1.0} else {0.0};
    }

    pub fn fire_if_ready(&mut self, target: Point, asteroids: &mut Vec<Asteroid>) {
        if self.firing <= 0.0 {
            self.fire(target, asteroids);
        }
    }

    fn fire(&mut self, target: Point, asteroids: &mut Vec<Asteroid>) {
        self.laser.pos_start = self.pos;
        let dir = (target - self.pos) / ((target - self.pos).x * (target - self.pos).x + (target - self.pos).y * (target - self.pos).y).sqrt();
        self.laser.pos_end = target + dir * LASER_LENGTH;
        self.firing = FIRING_TIME;
        let mut new = Vec::new();
        asteroids.retain_mut(|a| {
            if let Some(a2) = a.split(self.laser.pos_start, self.laser.pos_end) {
                if !a2.small() {
                    new.push(a2);
                }
                if a.small() {
                    return false;
                }
            }
            return true;
        });
        asteroids.append(&mut new);
    }
}
