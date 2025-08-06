use std::{thread, time};

fn main() {
    let delay = time::Duration::from_millis(500);
    loop {
        println!("ON");
        thread::sleep(delay);
        println!("OFF");
        thread::sleep(delay);
    }
}
