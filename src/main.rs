use rand::prelude::RngCore;
use rand::thread_rng;
use std::mem::transmute;

#[derive(PartialEq, Clone, Copy)]
enum Colour {
    Yellow,
    Green,
}

#[derive(PartialEq, Clone, Copy)]
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

        for xx in 0..x {
            let mut temp_vec: Vec<Option<Agent>> = Vec::new();

            for yy in 0..y {
                temp_vec.push(None);
            }

            f.push(temp_vec);
        }
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

    fn calculate_happiness(&mut self) {
        let max_x = self.field.len();
        let max_y = self.field[0].len();
        for x in 0..max_x {
            for y in 0..max_y {
                let mut yellow = 0;
                let mut green = 0;
                for xx in -1..1 {
                    for yy in -1..1 {
                        let x_index = x + xx as usize % max_x;
                        let y_index = y + yy as usize % max_y;
                        if self.field[x_index][y_index] == None {
                            continue;
                        }
                        if self.field[x_index][y_index].unwrap().colour == Colour::Yellow {
                            yellow += 1;
                        } else {
                            green += 1;
                        }
                    }
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
