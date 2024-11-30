use crate::shapes::Polygon;
use crate::shapes::Point;
use sdl2::render::{Canvas, RenderTarget};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;

pub struct Asteroid {
    shape: Polygon,
    dx : f64,
    dy: f64,

    rot: f64
}

impl Asteroid {
    pub fn new(poly: Polygon) -> Asteroid {
        Asteroid{shape: poly, dx: 10.0, dy: 7.7, rot: 1.0}
    }

    pub fn tick(&mut self, delta: f64) {
        self.shape.shift(self.dx * delta, self.dy * delta);

        self.shape.rotate(self.rot * delta);
    }

    pub fn render<T: RenderTarget>(&self, canvas : &mut Canvas<T>) -> Result<(), String>{
        if self.shape.corners() == 0 {
            return Ok(());
        }

        let vx = self.shape.points.iter().map(|p| p.x as i16).collect::<Vec<_>>();
        let vy = self.shape.points.iter().map(|p| p.y as i16).collect::<Vec<_>>();

        canvas.aa_polygon(&vx, &vy, Color::RGB(0, 0, 0))?;

        return Ok(());
    }

    pub fn cotains_point(&self, p: Point) -> bool {
        return self.shape.contains_point(p);
    }
}
