#[derive(Debug)]

struct User {
    name: String,
    status: Status,
}

#[derive(Debug)]
enum Status {
    Active,
    Inactive,
    Online,
}

fn main() {
    let user = User {
        name: "Maria".to_string(),
        status: Status::Active,
    };
    println!("The user data is: {:?}", user);
}
