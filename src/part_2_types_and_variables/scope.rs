pub fn run() {
    println!("\n====2.10 SCOPE====");
    scope_and_shadowing();
}

fn scope_and_shadowing() {
    let a = 123;
    println!("a = {}", a);

    {
        let b = 333;
        println!("inside b = {}", b);
        // use a from outer scope
        println!("inside a = {}", a);
        // shadow a from outer scope
        let a = 777;
        println!("inside shadow a = {}", a);
    }

    // here is a not from inner scope
    println!("outside a = {}", a);
}
