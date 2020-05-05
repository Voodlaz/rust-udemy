pub fn run() {
    println!("\n====2.9 OPERATORS====");
    let mut a = 3;
    a += 3;
    println!("remainder of {} / {} = {}", a, 3, (a % 3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    // bitwise
    let c = 1 | 2;
    println!("c = {}", c); // | OR & AND ^ XOR ! NOR
}