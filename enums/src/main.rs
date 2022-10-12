enum IpAddrKind {
    V4,
    V6,
}

// struct IpAddr{
//     kind: IpAddrKind,
//     address: String
// }

enum IpAddr {
    V4(String),
    // V4(u8, u8, u8, u8) Also legal in enums; different types inside the same enum.
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:#?}", self);
    }
}

fn main() {
    println!("Hello, world!");
    // let home = IpAddr{
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };
    //
    // let loopback = IpAddr{
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1")
    // };

    let m = Message::Write(String::from("hello"));

    m.call();
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V4(String::from("::1"));
}

fn route(ip_kind: IpAddrKind) {}
