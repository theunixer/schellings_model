use rand::prelude::RngCore;
use rand::thread_rng;
use std::mem::transmute;

enum Colour {
    Yellow,
    Green,
}

struct Agent {
    colour: Colour,
    happiness: f32,
}

struct Field {
    field: Vec<Vec<Option<Agent>>>,
}

impl Field {
    fn new(x: usize, y: usize) -> Self {
        let mut f: Vec<Vec<Option<Agent>>> = Vec::new();

        /*
        TODO: Implement reserve here
        f.reserve(x);
        f[0].reserve(y);
        */
        Field { field: f }
    }

    fn fill(&mut self, density: u32) {
        let mut rng = thread_rng();

        let max_x = self.field.len();
        let max_y = self.field[0].len();

        for x in 0..max_x {
            for y in 0..max_y {
                if rng.next_u32() % density == 0 {
                    self.field[x][y] = None;
                } else {
                    self.field[x][y] = Some(Agent {
                        colour: unsafe { transmute(rng.next_u32() as u8 % 2) },
                        happiness: 0.0,
                    });
                }
            }
        }
    }
}

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
