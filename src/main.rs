#[derive(Debug)]
struct User { // structs are like classes from Java and Python
    username: String,
    email: String, 
    sign_in_count: u64, 
    active: bool,
}

impl User { // impls are methods of the struct
    fn is_active(&self) -> bool { // Must pass in self reference to methods
        self.active
    }
    
    fn change_username(&mut self, new_username: String) { // Can pass in mutable self
        self.username = new_username;
    }

    fn sign_in(&mut self) {
        self.sign_in_count += 1;
        self.active = true;
    }

    fn build_user(email: String, username: String) -> User { // Associative functions do not need self reference passed in
        User {
            email,
            username, 
            active: false,
            sign_in_count: 1,
        }
    }
}

fn main() {
    let mut user1 = User { // Simple struct instance creation
        email: String::from("abc@mail.com"),
        username: String::from("andrw"), 
        active: true,
        sign_in_count: 1
    };

    User::change_username(&mut user1, String::from("andy")); // Calling impl methods
    println!(
        "name: {}; email: {}; signed in: {}; active: {}", 
        user1.username, user1.email, user1.sign_in_count, if user1.active {"yes"} else {"no"}
    );

    let mut user2 = User::build_user(
        String::from("gamerkid8@mail.com"), 
        String::from("You")
    );
    println!("user2 active: {}", User::is_active(&user2));
    User::sign_in(&mut user2);
    println!("user2 active: {}; signed in: {}", user2.active, user2.sign_in_count);

    let user3 = User {
        email: String::from("omg@mail.com"),
        username: String::from("hello"),
        ..user2 // Get the rest of the data from user2
    };

    println!("{:#?}", &user3); // Need special format for printing structs, also need Debug implementation
}