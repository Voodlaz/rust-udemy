use std::io;

pub fn run() {
    println!("\n====INPUT====");
    let pi = std::f64::consts::PI;

    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");


    let parsed_float = buffer.trim().parse::<f64>();

    let f = match parsed_float {
        Ok(float) => float,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    if f > pi {
        println!("{} greater than pi({})", f, pi);
    } else {
        println!("{} less or equal as pi({})", f, pi);
    };
}