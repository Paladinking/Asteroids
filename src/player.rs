use sdl2::render::{Canvas, RenderTarget};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use crate::shapes::{Point, Polygon};

const ACCELERATION: f64 = 500.0;
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
        Laser { pos_start, pos_end, shape: Polygon{ points: vec![Point::new(-10.0, 10.0), Point::new(-10.0, -10.0), Point::new(10.0, -10.0), Point::new(10.0, 10.0)] } }
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
    shape: Polygon,
    pos: Point,
    vel: Point,
    acc: f64,
    mov_dir: Vec<f64>, // [up, down, left, right]
    laser: Laser,
    firing: f64,
}

impl Player {
    pub fn new(shape: Polygon, pos: Point) -> Player {
        Player {shape, pos,
            vel: Point{ x: 0.0, y: 0.0 },
            acc: ACCELERATION,
            mov_dir: vec![0.0, 0.0, 0.0, 0.0],
            laser: Laser::new(Point{x: 0.0, y: 0.0}, Point{x: 0.0, y: 0.0}),
            firing: 0.0}
    }

    pub fn render<T: RenderTarget>(&mut self, canvas: &mut Canvas<T>) -> Result<(), String> {
        if self.shape.corners() == 0 {
            return Ok(());
        }

        let rot = self.vel.y.atan2(self.vel.x);
        let centre = Point::new(0.0, 0.0);

        let vx = self.shape.points.iter().map(|p| (self.pos.x + p.rotated(rot, centre).x) as i16).collect::<Vec<_>>();
        let vy = self.shape.points.iter().map(|p| (self.pos.y + p.rotated(rot, centre).y) as i16).collect::<Vec<_>>();

        canvas.aa_polygon(&vx, &vy, Color::RGB(255, 0, 0))?;

        self.laser.render(canvas, self.firing > 0.0)?;

        return Ok(());
    }

    // pub fn contains_point(&self, p : Point) -> bool {
    //     return self.shape.contains_point(p);
    // }

    pub fn tick(&mut self, delta: f64) {
        self.vel.y += (self.mov_dir[1] - self.mov_dir[0]) * self.acc * delta;
        self.vel.x += (self.mov_dir[3] - self.mov_dir[2]) * self.acc * delta;
        // Clamp velocity
        // if self.vel.x * self.vel.x + self.vel.y * self.vel.y > MAX_VEL * MAX_VEL {
        //     let div = (self.vel.x * self.vel.x + self.vel.y * self.vel.y).sqrt() / MAX_VEL;
        //     self.vel.x /= div;
        //     self.vel.y /= div;
        // }

        self.pos = self.pos + Point::new(self.vel.x * delta, self.vel.y * delta);

        if self.firing > 0.0 {
            self.firing = (self.firing - delta).clamp(0.0, FIRING_TIME);
        } else {
            self.laser.pos_start = self.pos;
        }
    }

    pub fn set_mov_dir(&mut self, dir: usize, val: bool) {
        self.mov_dir[dir] = if val {1.0} else {0.0};
    }

    pub fn fire(&mut self, target: Point) {
        self.laser.pos_start = self.pos;
        let dir = (target - self.pos) / ((target - self.pos).x * (target - self.pos).x + (target - self.pos).y * (target - self.pos).y).sqrt();
        self.laser.pos_end = self.pos + dir * LASER_LENGTH;
        self.firing = FIRING_TIME;
    }
}
