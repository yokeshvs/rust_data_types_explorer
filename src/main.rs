#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_must_use)]
#![allow(unused_variables)]

mod match_;
mod loops;
mod sh;
mod control_flow;
use std::mem;
mod lock;

fn main() {
    let a:i8 = 100;
    println!("a = {}", a);

    // a = 120; ---> cannot do this, as a is not defined as mutable.

    let mut b:u8 = 20;
    println!("b = {}", b);

    b = 99;
    println!("b = {}", b);

    //b = 1000; // this wont work. as we have defined b as 8 bit variable
    b = 255; // this is the max for this variable type
    println!("b = {}", b);

    let mut c:i8 = 127;
    // c = 128; this wont work as its signed variable. -128 to 127
    println!("c = {}", c);

    // u8 -> unsigned number -> 0 to 255
    // i8 -> signed number -> -127 to 128

    let d:i64 = 1231231231112312;
    println!("d = {} of size {} bytes", d, mem::size_of_val(&d));

    let mut e = 123;
    println!("e = {} of size {} bytes", e, mem::size_of_val(&e));

    let mut f = "abc";
    println!("f = {} of size {} bytes", f, mem::size_of_val(&f));

    f = "abcaqaeqweq";
    println!("f = {} of size {} bytes", f, mem::size_of_val(&f));

    let g:usize = 12;
    println!("g = {} of size {} bytes of {}-bit OS", g, mem::size_of_val(&g), 8*mem::size_of_val(&g));

    let h:&str = "x";
    println!("h = {} of size {} bytes", h, mem::size_of_val(&h));

    let i:char = 'a';
    println!("i = {} of size {} bytes", i, mem::size_of_val(&i));

    operators();

    scopes();

    sh::stack_and_heap();

    control_flow::if_statement(50);

    control_flow::if_statement(10);

    control_flow::if_statement(25);

    loops::loops(10);

    println!("matching {} = {}", 100, match_::get(100));
    println!("matching {} = {}", 3, match_::get(3));
    println!("matching {} = {}", 15, match_::get(15));
    println!("matching {} = {}", 90, match_::get(90));

    lock::unlock();
}

fn operators() {
    println!("1+2*5 = {}", 1+2*5);

    println!("3 cubed = {}", i64::pow(3,3));

    println!("2.5 cubed = {}", f64::powf(2.5, std::f64::consts::PI));

    println!("2.5 cubed = {}", f32::powf(2.5, std::f32::consts::PI));

}

fn scopes() {
    let a = 123;

    let a = 8888;

    {
        println!("Inside without local variable, a = {}", a);
    }

    {
        let a = 1234;
        println!("Inside with local variable, a = {}", a);
    }

    println!("Outside a = {}", a);
}