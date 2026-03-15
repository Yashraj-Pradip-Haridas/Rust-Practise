mod option_enum;

// Enums allows us to enumerate a list of variants

enum ipType {
    v3(u8, u8, u8), // This can also be done
    v4(String),
    v6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// Instead of writing separate structs for move and all other we can enclose all within the enum

impl Message {
    fn some_function() {
        print!("Lets get Rusty !!");
    }
}

struct ipAddress {
    kind: ipType,
    address: String,
}
fn main() {
    // println!("Hello, world!");
    // let four = ipType::v4;
    // let six = ipType::v6;

    // let localhost = ipAddress{
    //     kind:ipType::v4,
    //     address:String::from("127.0.0.1")
    // }
    let fictionalIp = ipType::v3(123, 255, 234);
    let localhost = ipType::v4(String::from("127.0.0.1"));
}

fn route(ip_kind: ipType) {}
