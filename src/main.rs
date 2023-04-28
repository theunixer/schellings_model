mod sm;
use crate::sm::sm::{Agent, Colour, Field};

fn main() {
    let mut field = Field::new(10, 20);
    field.fill(5);

    for x in 0..10 {
        for y in 0..20 {
            match field.field[x][y] {
                None => {
                    print!(" ");
                }
                Some(Agent {
                    colour: Colour::Green,
                    ..
                }) => {
                    print!("G");
                }
                Some(Agent {
                    colour: Colour::Yellow,
                    ..
                }) => {
                    print!("Y");
                }
                _ => {
                    panic!("error");
                }
            }
        }
        print!("\n");
    }
}
