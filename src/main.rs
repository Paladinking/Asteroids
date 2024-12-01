extern crate sdl2;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::render::{Canvas, RenderTarget};
use std::time::{Duration, Instant};

mod shapes;
use shapes::{Point, Polygon};
mod player;
use player::Player;
mod asteroid;
use asteroid::Asteroid;

const WINDOW_WIDTH: f64 = 1600.0;
const WINDOW_HEIGHT: f64 = 900.0;
const TIME_BETWEEN_METEOROIDS: f64 = 30.0;


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut time = Instant::now();

    let window = video_subsystem.window("Meteoroids", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0x00, 0x00, 0x00));
    canvas.clear();
    canvas.present();

    let mut reset_game = false;
    let mut asteroids = vec![];
    let mut spawn_time = TIME_BETWEEN_METEOROIDS;
    let mut meteoroid_spawner = Point::new(WINDOW_WIDTH / 2.0, -WINDOW_WIDTH * 1.0);
    let screen_centre = Point::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0);
    let start_meteoroids = 5;
    for _i in 0..start_meteoroids {
        let vel = Point::new(screen_centre.x - meteoroid_spawner.x, screen_centre.y - meteoroid_spawner.y) / ((2.0 + rand::random::<f64>()) * 5.0);
        // print!("start pos: {}x, {}y, start vel: {}x, {}y\n", meteoroid_spawner.x, meteoroid_spawner.y, vel.x, vel.y);
        let new_asteroid = Asteroid::get_randomized(75.0, meteoroid_spawner, vel);
        asteroids.push(new_asteroid);
        meteoroid_spawner = meteoroid_spawner.rotated(2.0 * 3.1415 / start_meteoroids as f64, screen_centre);
    }

    let mut player_alive = true;
    let mut time_alive = 0.0;
    let mut player = Player::new(screen_centre.clone());

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let delta = time.elapsed().as_secs_f64();

        if reset_game {
            reset_game = false;
            asteroids = vec![];
            spawn_time = TIME_BETWEEN_METEOROIDS;
            meteoroid_spawner = Point::new(WINDOW_WIDTH / 2.0, -WINDOW_WIDTH * 1.0);
            for _i in 0..start_meteoroids {
                let vel = Point::new(screen_centre.x - meteoroid_spawner.x, screen_centre.y - meteoroid_spawner.y) / ((2.0 + rand::random::<f64>()) * 5.0);
                // print!("start pos: {}x, {}y, start vel: {}x, {}y\n", meteoroid_spawner.x, meteoroid_spawner.y, vel.x, vel.y);
                let new_asteroid = Asteroid::get_randomized(75.0, meteoroid_spawner, vel);
                asteroids.push(new_asteroid);
                meteoroid_spawner = meteoroid_spawner.rotated(2.0 * 3.1415 / start_meteoroids as f64, screen_centre);
            }
            player_alive = true;
            time_alive = 0.0;
            player = Player::new(screen_centre.clone());
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    reset_game = true;
                },
                Event::MouseButtonDown { x, y, .. } => {
                    if player_alive {
                        player.fire_if_ready(Point{ x: x as f64, y: y as f64 }, &mut asteroids);
                    }
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

        if player_alive {
            player.tick(delta);
            time_alive += delta;
        }
        
        for a in &mut asteroids {
            a.tick(delta);
        }
        for i in 0..asteroids.len() {
            for j in 0..i {
                if asteroids[i].shape.centre.dist(asteroids[j].shape.centre) < asteroids[i].shape.radius + asteroids[j].shape.radius {
                    let col = asteroids[i].collides(&asteroids[j]);
                    if let Some((col, shift, norm)) = col {
                        //println!("{:?}, {:?}, {:?}", col, shift, norm);
                        let (p1, p2) = asteroids.split_at_mut(j + 1);
                        let a1 = &mut p1[j];
                        let a2 = &mut p2[i - (j + 1)];

                        a1.solve_polygon_collision(a2, col, shift, norm);
                        asteroids[i].render(&mut canvas).unwrap();
                        asteroids[j].render(&mut canvas).unwrap();
                        canvas.set_draw_color(Color::RGB(0xff, 0, 0));

                        fn draw_point<T: RenderTarget> (canvas: &mut Canvas<T>, p: Point, color: Color) {
                            canvas.set_draw_color(color);
                            canvas.draw_rect(sdl2::rect::Rect::new(p.x as i32 - 2, p.y as i32 - 2, 4, 4)).unwrap();
                        }

                        //draw_point(&mut canvas, col, Color::BLUE);
                        //draw_point(&mut canvas, col + shift, Color::RED);
                        canvas.set_draw_color(Color::RED);
                        //draw_point(&mut canvas, Point::new(152.6614114901822, 197.64510669436532), Color::YELLOW);
                        //draw_point(&mut canvas, Point::new(135.06919566423753, 83.07936587590183), Color::YELLOW);
                        //draw_point(&mut canvas, Point::new(142.13933756665278, 129.12223237753375), Color::YELLOW);
                        //draw_point(&mut canvas, Point::new(148.820142620565, 172.62962154544064), Color::YELLOW);

                        //canvas.draw_rect(sdl2::rect::Rect::new(col.x as i32 - 2, col.y as i32 - 2, 4, 4)).unwrap();
                        canvas.draw_line(sdl2::rect::Point::new(col.x as i32, col.y as i32), 
                                        sdl2::rect::Point::new((col.x + norm.x * 5.0) as i32, (col.y + norm.y * 5.0) as i32)).unwrap();
                        canvas.set_draw_color(Color::RGB(0xff, 0, 0));
                        //canvas.draw_line(sdl2::rect::Point::new(col.x as i32, col.y as i32), 
                        //                 sdl2::rect::Point::new((col.x + shift.x) as i32, (col.y + shift.y) as i32)).unwrap();


                        canvas.present();
                    }
                }
            }
            if player_alive {
                let player_col = asteroids[i].shape.get_collision(&player.shape);
                if let Some((_player_col, _shift, _norm)) = player_col {
                    player_alive = false;
                }
            }
        }

        spawn_time -= delta;
        if spawn_time < 0.0 {
            spawn_time = TIME_BETWEEN_METEOROIDS;
            meteoroid_spawner = meteoroid_spawner.rotated(2.0 * 3.1415 * rand::random::<f64>() as f64, screen_centre);
            let vel = Point::new(screen_centre.x - meteoroid_spawner.x, screen_centre.y - meteoroid_spawner.y) / ((2.0 + rand::random::<f64>()) * 5.0);
            asteroids.push(Asteroid::get_randomized(75.0 + 25.0 * rand::random::<f64>(), meteoroid_spawner, vel));
        }

        // Draw stuff
        canvas.set_draw_color(Color::RGB(0x00, 0x00, 0x00));
        canvas.clear();
        if player_alive {
            player.render(&mut canvas).unwrap();
        }
        for a in &asteroids {
            a.render(&mut canvas).unwrap();
        }

        canvas.string(20, 20, &(time_alive as i32).to_string(), Color::RGB(0xff, 0xff, 0xff)).unwrap();

        canvas.present();

        time = Instant::now();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
