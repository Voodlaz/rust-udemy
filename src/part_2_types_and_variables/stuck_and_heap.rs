use std::mem;

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn display(&self) -> () {
        println!("Point x:{}; y:{};", self.x, self.y)
    }
}

fn origin() -> Point {
    Point {x: 0.0, y: 0.0}
}


pub fn run() {
    println!("\n====2.12 STUCK AND HEAP====");
    let p1: Point = origin();
    let mut p2: Box<Point> = Box::new(origin());

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));
    println!("*p2 takes up {} bytes", mem::size_of_val(&*p2));
    println!("p2.x {}; p2.y {}", p2.x, p2.y);

    p2.x = 2.0;

    p2.display();

}