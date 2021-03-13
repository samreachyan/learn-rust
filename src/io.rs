use std::io;

fn main() {
    let mut buf = String::new();
    println!("Enter your name : ");
    io::stdin().read_line(&mut buf).ok().expect("Error!");
    println!("Hello {}", buf);
}