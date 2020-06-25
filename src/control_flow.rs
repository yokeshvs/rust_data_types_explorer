pub fn if_statement(temp:i32) {
    if temp > 30 {
        println!("Really hot!")
    } else if temp > 20 {
        println!("Normal")
    } else {
        println!("Really cold!")
    }
}