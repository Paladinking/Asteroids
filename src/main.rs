extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use std::time::{Duration, Instant};

mod shapes;
use shapes::{Point, Polygon};


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut time = Instant::now();
    let speed = 1000.0;
    // movement_dirs are [up, down, left, right]
    let mut movement_dirs = vec![0.0, 0.0, 0.0, 0.0];

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut poly = Polygon {points : vec![Point::new(150.0, 0.0), Point::new(100.0, 100.0), Point::new(150.0, 200.0)]};

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let delta = time.elapsed().as_secs_f64();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { x, y, .. } => {
                    println!("{}", poly.contains_point(Point::new(x as f64, y as f64)));
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    movement_dirs[0] = 1.0;
                },
                Event::KeyUp { keycode: Some(Keycode::W), .. } => {
                    movement_dirs[0] = 0.0;
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    movement_dirs[1] = 1.0;
                },
                Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                    movement_dirs[1] = 0.0;
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    movement_dirs[2] = 1.0;
                },
                Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                    movement_dirs[2] = 0.0;
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    movement_dirs[3] = 1.0;
                },
                Event::KeyUp { keycode: Some(Keycode::D), .. } => {
                    movement_dirs[3] = 0.0;
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        for p in poly.points.iter_mut() {
            p.y += (movement_dirs[1] - movement_dirs[0]) * speed * delta;
            p.x += (movement_dirs[3] - movement_dirs[2]) * speed * delta;
        }
 
        // Draw stuff
        canvas.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        canvas.clear();
        poly.render(&mut canvas).unwrap();

        canvas.present();

        time = Instant::now();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
