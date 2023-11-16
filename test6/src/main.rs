struct User {
    name: String,
    job: String,
    require_value: bool,
    value: i64
}

fn main() {
    println!("Hello, world!");

    let mut test1 = User {
        name: String::from("TEST1"),
        job: String::from("TEST2"),
        require_value: true,
        value: 32
    };

    test1.name = String::from("TEST Value");
    println!("User Data > {}", test1.name);

    let mut user1 = create_user(String::from("TEST1"), String::from("TEST2"), test1);
    user1.name = String::from("TEST2");
    println!("user1 > {}", user1.name);
    println!("user2 > {} struct update syntax {}", user1.name, user1.value);

    
}

fn create_user(name: String, job: String, user1: User) -> User {
    User {
        name,
        job,
        ..user1
    }
}
