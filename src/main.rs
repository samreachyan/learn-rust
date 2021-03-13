fn main() {
    let langs: [&str; 4]= ["Rust", "C++", "Java", "Python"];
    println!("{:?}", langs);

    let arr: [i32; 3] = [23, 856, 9302];
    for element in arr.iter() {
        println!("{}", element);
    }
}