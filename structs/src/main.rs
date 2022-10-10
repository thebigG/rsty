struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//named tuples
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("someotheremail@example.com");

    //Please note that this is a move. user1.username is no longer valid
    //In order for user1.username to remain valid one would have use String::from("")
    //to make a copy of that username, instead of moving it like we are doing here.
    let user2 = User {
        email: String::from("anotheremail2@example.com"),
        ..user1
    };

    //Same as
    // let user2 = User{
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("anotheremail2@example.com"),
    //     sign_in_count: user1.sign_in_count
    // };

    // user1.username = String::from("valid"); Would validate user1.username
    let name = user2.username;
    println!("Username from user1:{name}");

    let origin = Point(0, 0, 23);
    let black = Color(0, 0, 0);

    let origin_z = origin.2;
    println!("origin_z-->{origin_z}");
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }

    //Same as
    // User {
    //     email: email,
    //     username: username,
    //     active: true,
    //     sign_in_count: 1
    // }
}
