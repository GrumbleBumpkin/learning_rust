fn main() {
    let mut user1 = User {
        username: String::from("doodTHEdood"),
        email: String::from("dood@example.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("user1's email is {}", user1.email);
    user1.email = String::from("differentdood@example.com");
    println!("user1's email is {}", user1.email);

    let mut another_user = struct_factory(
        String::from("doodette"),
        String::from("doodette@example.com"),
    );
    another_user.email = String::from("doodette2@example.com");
    println!("another_users email is {}", user1.email);

    let user2 = User {
        username: String::from("ScrumBubbler4"),
        email: String::from("also_dood@example.com"),
        ..user1
    };
    println!("{}", user2.username);

    let black = Color(0, 0, 0);
    println!("{}", black.0);

    let origin = Point(0, 0, 0);
    println!("{}", origin.0);
}

struct Point(i32, i32, i32);
struct Color(i32, i32, i32);

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn struct_factory(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}
