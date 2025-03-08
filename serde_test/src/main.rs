use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]

struct User {
    name: String,
    password: String
}

fn main() {
    
    let u = User {
        name: String::from("Zaid"),
        password: String::from("password")
    };

    let serialized_string = match serde_json::to_string(&u) {
        Ok(s) => {
            println!("serialized the struct to string: {:?}", s);
            s
        },
        Err(e) => {
            println!("Error serializing: {:?}", e);
            return;
        }
    };

    let u2 = serde_json::from_str::<User>(&serialized_string);

    match u2
    {
        Ok(user) => {
            println!("deserialized the string to struct: {:?}", user);
            user
        },
        Err(e) => {
            println!("Error deserializing: {:?}", e);
            return;
        }
    };
}
