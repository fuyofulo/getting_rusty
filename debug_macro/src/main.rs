#[derive(Debug)]
struct User {
    username: String,
    age: i32,
    country: String
}

fn main () {
    
    let u = User {
        username: String::from("Zaid"),
        age: 21,
        country: String::from("India")
    };

    println!("{:?}", u);

}

