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
        .window("Schelling's model", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(8.0, 8.0);

    let mut field = Field::new(800 / 8, 600 / 8);
    field.fill(4);
    'running: loop {
        //drawing
        canvas.present();
        //checking events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some((Keycode::Escape)),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        //updating
        field.calculate_happiness();
        field.move_agent(0.70);

        for x in 0..field.field.len() {
            for y in 0..field.field[0].len() {
                match field.field[x][y] {
                    None => {
                        canvas.set_draw_color(Color::RGB(255, 255, 255));
                    }
                    Some(Agent {
                        colour: Colour::Green,
                        ..
                    }) => {
                        canvas.set_draw_color(Color::RGB(0, 153, 31));
                    }
                    Some(Agent {
                        colour: Colour::Yellow,
                        ..
                    }) => {
                        canvas.set_draw_color(Color::RGB(255, 204, 0));
                    }
                    _ => {
                        panic!("error");
                    }
                }
                let point = Point::new(x as i32, y as i32);
                canvas.draw_point(point);
            }
        }
        sleep(Duration::from_millis(500));
    }
}
