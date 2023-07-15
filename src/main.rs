use schellings_model::simulation::Simulation;
use schellings_model::{Agent, Field};
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::rwops::RWops;
use sdl2::url::open_url;
use std::sync::{Arc, Mutex};
use std::thread;
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
    //define current colours of elements
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
    canvas.set_scale(scale, scale).unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let texture_creator = canvas.texture_creator();
    let rwops = include_bytes!("NimbusSanL-Regu.ttf");
    let font = ttf_context
        .load_font_from_rwops(RWops::from_bytes(rwops).unwrap(), 13)
        .unwrap();

    let opened_toolbar_size = 24;
    let mut toolbar_size = opened_toolbar_size;

    let mut width = canvas.window().size().0;
    let mut height = canvas.window().size().1;

    // Creating and starting a simulation
    let mut field = Field::new(
        (width / scale as u32) as usize - toolbar_size,
        height as usize / scale as usize,
    );
    field.fill(4);
    let simulation = Mutex::new(Simulation {
        field,
        speed: 500,
        wanted_happiness: 0.50,
        running: true,
    });
    let simulation_ref = Arc::new(simulation);
    let simulation_ref1 = simulation_ref.clone();
    let simulation_thread = thread::spawn(move || loop {
        let mut simulation = simulation_ref1.lock().unwrap();
        if !simulation.running {
            break;
        }
        let wanted_happiness = simulation.wanted_happiness;
        simulation.field.move_agent(wanted_happiness);
        let speed = simulation.speed;
        drop(simulation);
        sleep(Duration::from_millis(speed));
    });

    let mut draw_to_screen: bool = true;
    let mut dark_theme: bool = true;
    let mut ui_changed: &str = "Press Q for help.";
    let mut toolbar_visible: bool = true;

    'running: loop {
        //checking events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    simulation_ref.lock().unwrap().running = false;
                    simulation_thread.join().unwrap();
                    break 'running;
                }
                Event::Window {
                    win_event: WindowEvent::Resized(x, y),
                    ..
                } => {
                    width = x as u32;
                    height = y as u32;

                    field = Field::new(
                        (width / scale as u32) as usize - toolbar_size,
                        height as usize / scale as usize,
                    );
                    field.fill(4);

                    ui_changed = "Window resized.";
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    let speed = &mut simulation_ref.lock().unwrap().speed;
                    if *speed < 1000 {
                        *speed += 100;
                        ui_changed = "Slowed down the simulation.";
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    let speed = &mut simulation_ref.lock().unwrap().speed;
                    if *speed > 0 {
                        *speed -= 100;
                        ui_changed = "Speeded up the simulation.";
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    draw_to_screen = !draw_to_screen;
                    if draw_to_screen {
                        ui_changed = "Rendering enabled.";
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => {
                    let wanted_happiness = &mut simulation_ref.lock().unwrap().wanted_happiness;
                    if *wanted_happiness > 0.10 {
                        *wanted_happiness -= 0.10;
                        ui_changed = "Require less alike neighbours.";
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    ..
                } => {
                    let wanted_happiness = &mut simulation_ref.lock().unwrap().wanted_happiness;
                    if *wanted_happiness < 1.0 {
                        *wanted_happiness += 0.10;
                        ui_changed = "Require more alike neighbours.";
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    dark_theme = !dark_theme;
                    ui_changed = "Theme changed.";

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
                    keycode: Some(Keycode::T),
                    ..
                } => {
                    toolbar_visible = !toolbar_visible;
                    if toolbar_visible {
                        toolbar_size = opened_toolbar_size;
                        ui_changed = "Toolbar opened.";
                    } else {
                        toolbar_size = 0;
                    }
                    field = Field::new(
                        (width / scale as u32) as usize - toolbar_size,
                        height as usize / scale as usize,
                    );
                    field.fill(4);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    let _ = open_url("https://github.com/GreatC0der/schellings_model/blob/master/readme.md#key-bindings");
                }
                Event::KeyDown { .. } => ui_changed = "Press Q for help.",
                _ => {}
            }
        }

        if !draw_to_screen {
            continue;
        }

        let Ok(simulation) = simulation_ref.try_lock() else { continue };

        canvas.clear();
        //getting the data from the app

        //cleaning screen
        canvas.set_draw_color(background_colour);

        canvas.set_draw_color(ui_colour);
        let toolbar_beggining = simulation.field.field.len() + 1;
        // drawing speed
        for i in 0..(1100 - simulation.speed) / 100 {
            let _ = canvas.draw_point(Point::new(i as i32 + toolbar_beggining as i32, 1));
        }
        //drawing wanted_happiness
        for i in 0..(simulation.wanted_happiness * 10.0 + 1.0) as i32 {
            let _ = canvas.draw_point(Point::new(i + toolbar_beggining as i32, 4));
        }
        //Last event's description
        let _ = canvas.set_scale(1.0, 1.0);
        let surface = font.render(ui_changed).blended(ui_colour).unwrap();
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

        //drawing field
        let mut points1: Vec<Point> = Vec::new();
        let mut points2: Vec<Point> = Vec::new();
        let mut empty_points: Vec<Point> = Vec::new();

        for x in 0..simulation.field.field.len() {
            for y in 0..simulation.field.field[0].len() {
                let point = Point::new(x as i32, y as i32);
                match simulation.field.field[x][y] {
                    None => {
                        empty_points.push(point);
                    }
                    Some(Agent::One) => {
                        points1.push(point);
                    }
                    Some(Agent::Two) => {
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
    }
}
