use std;
use std::io::stdin;

enum Status {
    LOCKED,
    UNLOCKED,
    FAILED
}

pub fn unlock() {
    let code = "1234";
    let mut status = Status::LOCKED;
    let mut input = String::new();
    loop {
        match status {
            Status::LOCKED => {
                stdin().read_line(&mut input);
                println!("{}", &input);
                if &input.trim_end().to_string() == code {
                    status = Status::UNLOCKED;
                } else {
                    status = Status::FAILED;
                }
            },
            Status::UNLOCKED => {
                println!("Unlocked");
                return;
            },
            Status::FAILED => {
                println!("Invalid Password");
                status = Status::LOCKED;
                input = String::new();
                continue
            }
        }
    }
}