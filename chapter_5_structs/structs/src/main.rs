struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
// A single stuct can have multiple implementation blocks
// There are methods and associate functions
// Methods gets passed with &self whereas associate function does not have one.
impl User {
    // These are methods since they are inside structs
    fn display_user(&self) {
        println!(
            "The username is {}, with the email {}",
            self.username, self.email
        );
    }
}

fn main() {
    println!("Hello, world!");
    let mut user1 = User {
        email: String::from("yashraj.haridas22@pccoepune.org"),
        username: String::from("yashraj"),
        sign_in_count: 0,
        active: true,
    };
    let name = user1.username;
    user1.username = String::from("Yashraj Haridas");

    let user2 = build_user(
        String::from("yashraj.haridas236@gmail.com"),
        String::from("Yashraj"),
    );

    // We can create new instances of struct using existing instances
    let user3 = User {
        email: String::from("xyz@gmail.com"),
        username: String::from("xyz"),
        ..user2
    };
    user3.display_user();

    // Other notation is example for rectangel struct
    // let rect3 = Rectangle::square(10);

    // we can also create structs without name fields and these are called tuple structs
    // Tuple structs are useful when you want your entire tuple to have a name and be of different type than other tuples
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // unit like structs -  structs without any fields
}

fn build_user(email: String, username: String) -> User {
    User {
        username, // can be done only when same
        email,    // also called field init short hand syntax
        sign_in_count: 1,
        active: false,
    }
}
