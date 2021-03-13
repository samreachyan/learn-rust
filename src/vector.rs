fn main() {
    let mut numbers = vec![423, 943, 192];
    numbers.push(13);
    numbers.push(52);
    println!("{:?}", numbers); // [423, 943, 192, 13, 52]
    numbers.pop();
    println!("{:?}", numbers); // [423, 943, 192, 13]\

    println!("{:?}", personal_info("Long", 24)); // ("Long", 24)
}

fn personal_info(name: &str, age: i32) -> (&str, i32) {
    return (name, age);
}