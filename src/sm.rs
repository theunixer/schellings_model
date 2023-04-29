pub mod sm {
    use rand::prelude::RngCore;
    use rand::thread_rng;
    use std::mem::transmute;
    #[derive(PartialEq, Clone, Copy)]
    pub enum Colour {
        Yellow,
        Green,
    }

    #[derive(PartialEq, Clone, Copy)]
    pub struct Agent {
        pub colour: Colour,
        pub happiness: f32,
    }

    pub struct Field {
        pub field: Vec<Vec<Option<Agent>>>,
    }

    impl Field {
        pub fn new(x: usize, y: usize) -> Self {
            let mut f: Vec<Vec<Option<Agent>>> = Vec::new();

            for _ in 0..x {
                let mut temp_vec: Vec<Option<Agent>> = Vec::new();
                for _ in 0..y {
                    temp_vec.push(None);
                }
                f.push(temp_vec);
            }

            Field { field: f }
        }

        pub fn fill(&mut self, density: u32) {
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

        pub fn calculate_happiness(&mut self) {
            let max_x = self.field.len();
            let max_y = self.field[0].len();

            for x in 0..max_x {
                for y in 0..max_y {
                    Self::get_and_set_happiness(self, x, y);
                }
            }
        }

        fn get_and_set_happiness(field: &mut Field, x: usize, y: usize) {
            let max_x = field.field.len();
            let max_y = field.field[0].len();

            field.field[x][y] = {
                let mut yellow = 0;
                let mut green = 0;

                for xx in -1..1 {
                    for yy in -1..1 {
                        let x_index = ((x as i32 + xx) as usize) % max_x;
                        let y_index = ((y as i32 + yy) as usize) % max_y;

                        if field.field[x_index][y_index] == None {
                            continue;
                        }
                        if field.field[x_index][y_index].unwrap().colour == Colour::Yellow {
                            yellow += 1;
                        } else if field.field[x_index][y_index].unwrap().colour == Colour::Green {
                            green += 1;
                        }
                    }
                }

                if field.field[x][y] == None {
                    None
                } else if field.field[x][y].unwrap().colour == Colour::Yellow {
                    Some(Agent {
                        colour: Colour::Yellow,
                        happiness: yellow as f32 / (yellow + green) as f32,
                    })
                } else if field.field[x][y].unwrap().colour == Colour::Green {
                    Some(Agent {
                        colour: Colour::Green,
                        happiness: green as f32 / (yellow + green) as f32,
                    })
                } else {
                    None
                }
            };
        }

        pub fn move_agent(&mut self, wanted_happiness: f32) {
            let max_x = self.field.len();
            let max_y = self.field[0].len();

            let mut agent: (usize, usize);
            let mut empty: (usize, usize);

            let mut rng = thread_rng();

            loop {
                agent = (
                    rng.next_u32() as usize % max_x,
                    rng.next_u32() as usize % max_y,
                );
                if self.field[agent.0][agent.1] == None {
                    continue;
                }
                if self.field[agent.0][agent.1].unwrap().happiness > wanted_happiness {
                    continue;
                }
                break;
            }

            loop {
                empty = (
                    rng.next_u32() as usize % max_x,
                    rng.next_u32() as usize % max_y,
                );
                if self.field[empty.0][empty.1] != None {
                    continue;
                }
                break;
            }

            self.field[empty.0][empty.1] = self.field[agent.0][agent.1].clone();
            self.field[agent.0][agent.1] = None;
        }
    }
}
