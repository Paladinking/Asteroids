extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use std::time::{Duration, Instant};

mod shapes;
use shapes::{Point, Polygon};
mod player;
use player::Player;
mod asteroid;
use asteroid::Asteroid;


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut time = Instant::now();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0x00, 0x00, 0x00));
    canvas.clear();
    canvas.present();

    let p1 = Polygon {points : vec![Point::new(150.0, 0.0), Point::new(100.0, 100.0),
                                          Point::new(150.0, 200.0), Point::new(175.0, 150.0)]};
    let p2 = Polygon {points: vec![Point::new(0.0, 0.0), Point::new(50.0, 50.0), Point::new(33.0, 50.0), Point::new(2.0, 0.0)]};

    let mut asteroids = vec![Asteroid::new(p1, Point::new(100.0, 100.0), 77.0, -123.0, 2.0),
                             Asteroid::new(p2, Point::new(50.0, 50.0), -33.0, 20.0, 1.0)];
    let player_poly = Polygon {points : vec![Point::new(25.0, -45.0), Point::new(-25.0, 0.0), Point::new(25.0, 45.0)]};

    let mut player = Player::new(player_poly, Point::new(50.0, 50.0));

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
                    player.fire(Point{ x: x as f64, y: y as f64 });
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    player.set_mov_dir(0, true);
                },
                Event::KeyUp { keycode: Some(Keycode::W), .. } => {
                    player.set_mov_dir(0, false);
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    player.set_mov_dir(1, true);
                },
                Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                    player.set_mov_dir(1, false);
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    player.set_mov_dir(2, true);
                },
                Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                    player.set_mov_dir(2, false);
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    player.set_mov_dir(3, true);
                },
                Event::KeyUp { keycode: Some(Keycode::D), .. } => {
                    player.set_mov_dir(3, false);
                },
                _ => {}
            }
        }
        player.tick(delta);
        for a in &mut asteroids {
            a.tick(delta);
        }
 
        // Draw stuff
        canvas.set_draw_color(Color::RGB(0x00, 0x00, 0x00));
        canvas.clear();
        player.render(&mut canvas).unwrap();
        for a in &asteroids {
            a.render(&mut canvas).unwrap();
        }

        canvas.present();

        time = Instant::now();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
