use rand::prelude::RngCore;
use rand::thread_rng;
use std::mem::transmute;
#[derive(PartialEq, Clone, Copy)]
pub enum Agent {
    One,
    Two,
}

pub struct Field {
    pub field: Vec<Vec<Option<Agent>>>,
    emptys: Vec<(usize, usize)>,
}

impl Field {
    pub fn new(x: usize, y: usize) -> Self {
        let mut field: Vec<Vec<Option<Agent>>> = Vec::new();

        // Init 2d array
        for _ in 0..x {
            let mut temp_vec: Vec<Option<Agent>> = Vec::new();
            for _ in 0..y {
                temp_vec.push(None);
            }
            field.push(temp_vec);
        }

        Field {
            field,
            emptys: Vec::new(),
        }
    }

    pub fn fill(&mut self, density: u32) {
        let mut rng = thread_rng();

        let max_x = self.field.len();
        let max_y = self.field[0].len();

        let mut emptys = Vec::new();

        for x in 0..max_x {
            for y in 0..max_y {
                if rng.next_u32() % density == 0 {
                    self.field[x][y] = None;
                    emptys.push((x, y));
                } else {
                    self.field[x][y] = Some(unsafe { transmute(rng.next_u32() as u8 % 2) });
                }
            }
        }

        self.emptys = emptys;
    }

    fn get_happiness(&self, x: usize, y: usize) -> f32 {
        let max_x = self.field.len();
        let max_y = self.field[0].len();

        let mut one = 0;
        let mut two = 0;

        for xx in -1..1 {
            for yy in -1..1 {
                let x_index = ((x as i32 + xx) as usize) % max_x;
                let y_index = ((y as i32 + yy) as usize) % max_y;

                if self.field[x_index][y_index].is_none() {
                    continue;
                }
                if self.field[x_index][y_index].unwrap() == Agent::One {
                    one += 1;
                } else if self.field[x_index][y_index].unwrap() == Agent::Two {
                    two += 1;
                }
            }
        }

        match self.field[x][y].unwrap() {
            Agent::One => one as f32 / (one + two) as f32,
            Agent::Two => two as f32 / (one + two) as f32,
        }
    }

    pub fn move_agent(&mut self, wanted_happiness: f32) {
        let max_x = self.field.len();
        let max_y = self.field[0].len();

        let mut rng = thread_rng();

        // Looking for suitable agent to move
        let agent = loop {
            let agent = (
                rng.next_u32() as usize % max_x,
                rng.next_u32() as usize % max_y,
            );
            if self.field[agent.0][agent.1].is_none()
                || (self.get_happiness(agent.0, agent.1) > wanted_happiness)
            {
                continue;
            }
            break agent;
        };

        // moving
        let empty_index = rng.next_u32() as usize % self.emptys.len();
        let empty = self.emptys[empty_index];

        self.field[empty.0][empty.1] = self.field[agent.0][agent.1];
        self.field[agent.0][agent.1] = None;
        self.emptys[empty_index] = (agent.0, agent.1);
    }
}
