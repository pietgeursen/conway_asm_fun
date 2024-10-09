use std::time::Duration;

use conway_asm_fun::*;

fn main() {
    let mut conways = Conway::new();

    loop{
        conways.print();
        conways.next();
        std::thread::sleep(Duration::from_secs(1));
    }
}
