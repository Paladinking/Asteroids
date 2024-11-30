extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, RenderTarget};
use std::iter::Sum;
use std::ops::Add;
use std::time::Duration;

#[derive(Copy, Clone)]
struct Point {
    pub x: f64,
    pub y: f64
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        return Point {x: self.x + rhs.x, y: self.y + rhs.y};
    }
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point {x, y}
    }
}

struct Polygon {
    points: Vec<Point>
}

impl Polygon {
    fn render<T: RenderTarget>(&self, canvas : &mut Canvas<T>) -> Result<(), String>{
        if self.points.len() == 0 {
            return Ok(());
        }

        let middle = self.points.iter().fold(Point {x: 0.0, y: 0.0}, |p, a| p + *a);

        let vx = self.points.iter().map(|p| p.x as i16).collect::<Vec<_>>();
        let vy = self.points.iter().map(|p| p.y as i16).collect::<Vec<_>>();

        canvas.aa_polygon(&vx, &vy, Color::RGB(0, 0, 0))?;

        return Ok(());
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let poly = Polygon {points : vec![Point::new(150.0, 0.0), Point::new(100.0, 100.0), Point::new(150.0, 200.0)]};

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {

        canvas.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        canvas.clear();
        poly.render(&mut canvas).unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
