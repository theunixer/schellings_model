mod sm;
use crate::sm::sm::{Agent, Field, Group};
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::{thread::sleep, time::Duration};

fn main() {
    //define colours
    let dark_gray: Color = Color::RGB(35, 35, 35);
    let white: Color = Color::RGB(250, 250, 250);
    let green: Color = Color::RGB(0, 153, 31);
    let yellow: Color = Color::RGB(255, 204, 0);
    let black: Color = Color::RGB(0, 0, 0);
    //define colours of elements
    let mut background_colour = dark_gray;
    let mut ui_colour = white;
    let mut group1_colour = green;
    let mut group2_colour = yellow;

    //sdl initialisation
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Schelling's model", 1000, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(8.0, 8.0);

    let mut field = Field::new(
        (canvas.window().size().0 / 8) as usize - 12,
        canvas.window().size().1 as usize / 8,
    );
    field.fill(4);

    let mut speed: u64 = 500;
    let mut wanted_happiness: f32 = 0.50;
    let mut draw_to_screen: bool = true;
    let mut dark_theme: bool = true;

    'running: loop {
        //checking events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::Window {
                    win_event: WindowEvent::Resized(x, y),
                    ..
                } => {
                    field = Field::new((x / 8) as usize - 12, y as usize / 8);
                    field.fill(4);
                }
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
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    dark_theme = !dark_theme;
                    if dark_theme {
                        background_colour = dark_gray;
                        ui_colour = white;
                    } else {
                        background_colour = white;
                        ui_colour = black;
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
        canvas.set_draw_color(background_colour);
        canvas.clear();

        //drawing field
        let mut points1: Vec<Point> = Vec::new();
        let mut points2: Vec<Point> = Vec::new();

        for x in 0..field.field.len() {
            for y in 0..field.field[0].len() {
                let point = Point::new(x as i32, y as i32);
                match field.field[x][y] {
                    None => {}
                    Some(Agent {
                        colour: Group::One, ..
                    }) => {
                        points1.push(point);
                    }
                    Some(Agent {
                        colour: Group::Two, ..
                    }) => {
                        points2.push(point);
                    }
                    _ => {
                        panic!("error");
                    }
                }
            }
        }

        canvas.set_draw_color(group1_colour);
        canvas.draw_points(points1.as_slice());

        canvas.set_draw_color(group2_colour);
        canvas.draw_points(points2.as_slice());

        canvas.set_draw_color(ui_colour);

        let toolbar_beggining = field.field.len() + 1;
        // drawing speed
        for i in 0..(1100 - speed) / 100 {
            canvas.draw_point(Point::new(i as i32 + toolbar_beggining as i32, 1));
        }

        //drawing wanted_happiness
        for i in 0..(wanted_happiness * 10 as f32 + 1.0) as i32 {
            canvas.draw_point(Point::new(i + toolbar_beggining as i32, 4));
        }

        canvas.present();

        //sleeping
        sleep(Duration::from_millis(1));
        sleep(Duration::from_millis(speed));
    }
}
