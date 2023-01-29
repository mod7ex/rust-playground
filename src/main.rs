#![allow(unused)]

#[derive(Default, Debug)]
struct User {
    name: String,
    age: u8,
}

impl User {
    fn new() -> User {
        User{ ..Default::default() }
    }

}

fn main() {
    let me = User::new();

    println!("{:?}", &me);
}