struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    println!("Hello, world!");

    let mut user1: User = User {
        active: true,
        username: String::from("cascata"),
        email: String::from("jpppedreira@gmail.com"),
        sign_in_count: 1
    };
    println!("Email user1: {}", user1.email);

    user1.email = String::from("cascata@gmail.com");
    println!("Email user2: {}", user1.email);

    let email = String::from("jpppedreira@gmail.com");
    let user2 = build_user(&email, String::from("jpppeira"));
    println!("email: {}", email);

    let user3 = User {
        email: String::from("cascata@gmail.com"),
        ..user2
    };

    println!("user3 email: {}", user3.email);
}

fn build_user(email: &String, username: String) -> User {
    let email2 = email.clone();
    User {
        active: true,
        username,
        email: email2,
        sign_in_count: 1
    }
}
