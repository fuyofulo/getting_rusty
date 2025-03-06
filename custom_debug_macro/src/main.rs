use std::fmt::{Debug, Formatter};

struct User {
    username: String
}

impl Debug for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "username is {}", self.username)
    }
}

fn main() {
    let u = User {
        username: String::from("harkirat")
    };

    println!("{:?}", u);
}
