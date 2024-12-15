use std::{io, thread, time};

fn main() {
    let mut name = String::new();
    let mut age = String::new();

    println!("What is your name: ");
    io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("What is your age: ");
    io::stdin().read_line(&mut age).expect("Failed to read line");

    println!("Hello {} you are {} years old", name.trim(), age.trim());

    let five_seconds = time::Duration::from_millis(5000);

    thread::sleep(five_seconds);
}