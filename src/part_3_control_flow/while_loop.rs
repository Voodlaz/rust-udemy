pub fn run() {
    println!("\n====3.14 LOOPS / 3.15 FOR====");
    let mut x = 0;
    let loop_result = loop {
        x += 100;
        if x == 1000 {
            break x;  // break returns value here
        }
    };
    assert_eq!(loop_result, 1000);
    println!("loop result is {}", loop_result);

    let mut i = 0;

    while i < 3 {
        println!("hello");
        i += 1;
    }

    let mut z = vec![1, 2, 3];

    while let Some(y) = z.pop() {
        println!("y = {}", y);
    };

    for x in 1..10 {
        if x == 2 {
            continue;
        }
        if x == 4 {
            break;
        }
        println!("{}", x)
    }


    // enumerate
    for (index, value) in (30..40).enumerate() {
        println!("index = {}; value = {}", index, value);
        if index > 4 {
            break;
        }
    }

    // last
    let mut last = 0;
    for x in 1..6 {
        if x > 3 {
            break;
        }
        last = x;
    }
    assert_eq!(last, 3);

    println!("last = {}", last)
        
}