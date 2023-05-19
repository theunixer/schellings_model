mod sm;
use crate::sm::sm::{Agent, Field, Group};
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::rwops::RWops;
use std::{thread::sleep, time::Duration};

fn main() {
    //define colours
    let dark_gray: Color = Color::RGB(35, 35, 35);
    let white: Color = Color::RGB(250, 250, 250);
    let green: Color = Color::RGB(0, 153, 31);
    let yellow: Color = Color::RGB(255, 204, 0);
    let red: Color = Color::RGB(226, 61, 61);
    let blue: Color = Color::RGB(49, 76, 146);
    let black: Color = Color::RGB(0, 0, 0);
    //define colours of elements
    let mut background_colour = dark_gray;
    let mut ui_colour = white;
    let mut group1_colour = red;
    let mut group2_colour = blue;

    //sdl initialisation
    let scale: f32 = 8.0;

    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Schelling's model", 1000, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let _ = canvas.set_scale(scale, scale);
    let mut event_pump = sdl_context.event_pump().unwrap();

    let texture_creator = canvas.texture_creator();
    let rwops = include_bytes!("NimbusSanL-Regu.ttf");
    let font = ttf_context
        .load_font_from_rwops(RWops::from_bytes(rwops).unwrap(), 13)
        .unwrap();

    let toolbar_size = 24;
    let mut field = Field::new(
        (canvas.window().size().0 / scale as u32) as usize - toolbar_size,
        canvas.window().size().1 as usize / scale as usize,
    );
    field.fill(4);

    let mut speed: u64 = 500;
    let mut wanted_happiness: f32 = 0.50;
    let mut draw_to_screen: bool = true;
    let mut dark_theme: bool = true;
    let mut ui_changed: (bool, &str) = (true, "Press Q for help.");

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
                    field = Field::new(
                        (x / scale as i32) as usize - toolbar_size,
                        y as usize / scale as usize,
                    );
                    field.fill(4);

                    ui_changed = (true, "Window resized.");
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    if speed < 1000 {
                        speed = speed + 100;
                        ui_changed = (true, "Slowed down the simulation.");
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    if speed > 0 {
                        speed -= 100;
                        ui_changed = (true, "Speeded up the simulation.");
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    draw_to_screen = !draw_to_screen;
                    if draw_to_screen {
                        ui_changed = (true, "Rendering enabled.");
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => {
                    if wanted_happiness > 0.10 {
                        wanted_happiness -= 0.10;
                        ui_changed = (true, "Require less alike neighbours.");
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    ..
                } => {
                    if wanted_happiness < 1.0 {
                        wanted_happiness += 0.10;
                        ui_changed = (true, "Require more alike neighbours.");
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    dark_theme = !dark_theme;
                    ui_changed = (true, "Theme changed.");

                    if dark_theme {
                        background_colour = dark_gray;
                        ui_colour = white;
                        group1_colour = red;
                        group2_colour = blue;
                    } else {
                        background_colour = white;
                        ui_colour = black;
                        group1_colour = green;
                        group2_colour = yellow;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    let _ =
                        open::that("https://notabug.org/GreatC0der/schellings_model/src/master/readme.md#key-bindings");
                }
                Event::KeyDown { .. } => {
                    ui_changed = (true, "Press Q for help.");
                }
                _ => {}
            }
        }
        //updating
        field.move_agent(wanted_happiness);

        if !draw_to_screen {
            continue;
        }

        if ui_changed.0 {
            //cleaning screen
            canvas.set_draw_color(background_colour);
            canvas.clear();

            canvas.set_draw_color(ui_colour);
            let toolbar_beggining = field.field.len() + 1;
            // drawing speed
            for i in 0..(1100 - speed) / 100 {
                let _ = canvas.draw_point(Point::new(i as i32 + toolbar_beggining as i32, 1));
            }
            //drawing wanted_happiness
            for i in 0..(wanted_happiness * 10 as f32 + 1.0) as i32 {
                let _ = canvas.draw_point(Point::new(i + toolbar_beggining as i32, 4));
            }
            //Last event's description
            let _ = canvas.set_scale(1.0, 1.0);
            let surface = font.render(ui_changed.1).blended(ui_colour).unwrap();
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();

            let TextureQuery { width, height, .. } = texture.query();
            let text_rect = Rect::new(
                (toolbar_beggining * scale as usize) as i32,
                6 * scale as i32,
                width,
                height,
            );
            let _ = canvas.copy(&texture, None, text_rect);
            let _ = canvas.set_scale(scale, scale);
        }

        //drawing field
        let mut points1: Vec<Point> = Vec::new();
        let mut points2: Vec<Point> = Vec::new();
        let mut empty_points: Vec<Point> = Vec::new();

        for x in 0..field.field.len() {
            for y in 0..field.field[0].len() {
                let point = Point::new(x as i32, y as i32);
                match field.field[x][y] {
                    None => {
                        empty_points.push(point);
                    }
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
                }
            }
        }

        canvas.set_draw_color(group1_colour);
        let _ = canvas.draw_points(points1.as_slice());

        canvas.set_draw_color(group2_colour);
        let _ = canvas.draw_points(points2.as_slice());

        canvas.set_draw_color(background_colour);
        let _ = canvas.draw_points(empty_points.as_slice());

        canvas.present();

        //sleeping
        sleep(Duration::from_millis(1));
        sleep(Duration::from_millis(speed));

        ui_changed = (false, "");
    }
}
