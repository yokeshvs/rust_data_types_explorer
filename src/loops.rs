pub fn loops(mut x:u32) {
    while x < 100 {
        x *= 2;
        if x == 160 {continue}
        println!("x = {}", x);
    }
    let mut y = 10;
    loop {
        y *= 2;
        println!("y = {}", y);
        if y > 100 {break}
    }

    for a in 1..11 {
        println!("a = {}", a)
    }

    for (pos, b) in (1..11).enumerate() {
        println!("index = {} and val {}", pos, b)
    }
}