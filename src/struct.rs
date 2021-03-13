struct Blog(&'static str, i32);

struct Person{
    name: &'static str,
    age: i32,
}
fn main() {
    let code = Blog("code", 2021);
    let Blog(name, date) = code;
    println!("{} - {} ", name, date);

    let person = Person{
        name: "Samreach",
        age: 24
    };

    println!("{} - {}", person.name, person.age);
}