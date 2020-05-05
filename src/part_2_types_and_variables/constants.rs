const MEANING_OF_LIFE: u8 = 42;
static mut IS_CHUCHUA: bool = true;

pub fn run() {
    println!("\n====2.11 CONSTANTS====");
    consts()
}


fn consts() {
    println!("Meaining of life is {}", MEANING_OF_LIFE);
    unsafe {
        println!("Is chuchua? {}", IS_CHUCHUA);
        IS_CHUCHUA = false;
        println!("Turn chuchua off");
        println!("Is chuchua? {}", IS_CHUCHUA);
    };
}