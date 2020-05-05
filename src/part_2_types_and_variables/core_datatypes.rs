use std::mem;

pub fn primitive() {
    println!("\n====2.8 CORE DATATYPES====");
    let a: u8 = 255; // u8 holds values 0-255
    let b: i32 = 255; // i8 holds values - 128 - 127
    // e.t.c.
    // 16-bit	i16	  u16
    // 32-bit	i32	  u32
    // 64-bit	i64	  u64
    // 128-bit	i128  u128
    // arch	    isize usize

    let c: isize = 125;
    let size_of_c = mem::size_of_val(&c);
    let os_bit = size_of_c * 8;


    let f:f64 = 2.5;

    println!("a = {}, size = {} bytes", a, mem::size_of_val(&a));
    println!("b = {}, size = {} bytes", b, mem::size_of_val(&b));
    println!("c = {}, size = {} bytes in {}-bit os", c, size_of_c, os_bit);
    println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));
    println!("\n");


    let c:char = 'c';
    let k:char = 'ж';
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    println!("k = {}, size = {} bytes", k, mem::size_of_val(&k));
    println!("\n");

    let bl = false;
    println!("bl = {}, size = {} bytes", bl, mem::size_of_val(&bl));
    println!("\n");
}
