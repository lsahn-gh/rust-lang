struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user = build_user(String::from("someone@fff.com"),
                          String::from("username123"));
    let user2 = User {
        email: String::from("another1@fffef.com"),
        username: String::from("username2"),
        ..user
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
