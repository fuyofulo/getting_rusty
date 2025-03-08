use borsh::{ BorshDeserialize, BorshSerialize };

#[derive( BorshDeserialize, BorshSerialize, Debug, Clone)]

struct User {
    name: String,
    password: String
}

fn main() {
    
    let u = User {
        name: String::from("zaid"),
        password: String::from("password"),
    };

    let mut v: Vec<u8> = Vec::new();

    let _ = u.serialize(&mut v);

    println!("{:?}", v);

    let user = User::try_from_slice(&v).unwrap();
    println!("{}, {}", user.name, user.password);


}
