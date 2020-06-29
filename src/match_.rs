pub fn get(val: i64) -> i64 {
    return match val {
        1..=10 => 100,
        2..=20 => 200,
        _ => 1000
    };
}