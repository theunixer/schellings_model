use crate::Field;
use std::time::SystemTime;

#[test]
fn ten_thousand_iterations() {
    let mut field = Field::new(100, 100);
    field.fill(4);

    let started = SystemTime::now();

    for _ in 0..10000 {
        field.move_agent(0.5);
    }

    let ended = started.elapsed().unwrap().as_millis();
    println!("It took {} mils for 10000 to be completed.", ended);
}
