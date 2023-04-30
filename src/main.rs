mod sm;
use crate::sm::sm::{Agent, Colour, Field};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::video;
use std::{thread::sleep, time::Duration};

fn main() {
    //sdl initialisation
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Schelling's model", 1000, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(8.0, 8.0);

    let mut field = Field::new(800 / 8, 600 / 8);
    field.fill(4);

    let mut speed: u64 = 500;
    let mut wanted_happiness: f32 = 0.50;
    let mut draw_to_screen: bool = true;

    'running: loop {
        //checking events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    if speed < 1000 {
                        speed = speed + 100;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    if speed > 0 {
                        speed -= 100;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    draw_to_screen = !draw_to_screen;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => {
                    if wanted_happiness > 0.10 {
                        wanted_happiness -= 0.10;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    ..
                } => {
                    if wanted_happiness < 1.0 {
                        wanted_happiness += 0.10;
                    }
                }
                _ => {}
            }
        }
        //updating
        field.calculate_happiness();
        field.move_agent(wanted_happiness);

        if !draw_to_screen {
            continue;
        }
        //cleaning screen
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        //drawing field
        let mut green_points: Vec<Point> = Vec::new();
        let mut yellow_points: Vec<Point> = Vec::new();

        for x in 0..field.field.len() {
            for y in 0..field.field[0].len() {
                let point = Point::new(x as i32, y as i32);
                match field.field[x][y] {
                    None => {}
                    Some(Agent {
                        colour: Colour::Green,
                        ..
                    }) => {
                        green_points.push(point);
                    }
                    Some(Agent {
                        colour: Colour::Yellow,
                        ..
                    }) => {
                        yellow_points.push(point);
                    }
                    _ => {
                        panic!("error");
                    }
                }
            }
        }

        canvas.set_draw_color(Color::RGB(0, 153, 31));
        canvas.draw_points(green_points.as_slice());

        canvas.set_draw_color(Color::RGB(255, 204, 0));
        canvas.draw_points(yellow_points.as_slice());

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        // drawing speed
        for i in 0..(1100 - speed) / 100 {
            canvas.draw_point(Point::new(i as i32 + 101, 1));
        }

        //drawing wanted_happiness
        for i in 0..(wanted_happiness * 10 as f32 + 1.0) as i32 {
            canvas.draw_point(Point::new(i + 101, 4));
        }

        canvas.present();

        //sleeping
        sleep(Duration::from_millis(1));
        sleep(Duration::from_millis(speed));
    }
}
