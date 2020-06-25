use std::mem;

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point{ x: 0.0, y: 0.0 }
}

pub fn stack_and_heap() {
    let p1 = origin();
    let p2 = Box::new(origin());
    println!("size of p1 {} bytes", mem::size_of_val(&p1));
    println!("size of p2 {} bytes", mem::size_of_val(&p2));

    let p3 = *p2;
    println!("size of p3 {} bytes", mem::size_of_val(&p3));
    println!("p3.x = {}", p3.x);
}