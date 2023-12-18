// STRUCTURES

struct User {
    username: String,
    _email: String,
    _sign_in_count: u64,
    _active: bool,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // Method
        self.width * self.height
    }

    // Method
    fn can_hold(&self, other: &Rectangle) -> bool { 
        self.width > other.width && self.height > other.height
    }
    
    // Associated Function
    fn square(size: u32) -> Rectangle { 
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        username: String::from("sachin"),
        _email: String::from("gmail.com"),
        _sign_in_count: 1,
        _active: true,
    };

    let _name = user1.username;
    user1.username = String::from("rahul");

    let user2 = build_user(String::from("rachin@gmail.com"), String::from("rachin"));

    let user3 = User {
        username: String::from("sachin"),
        _email: String::from("gmail.com"),
        ..user1
    };

    println!("{} {} {}", user1.username, user2.username, user3.username);




    // Tuple Structs
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);




    // Area of Rectangle
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 69,
    };

    println!("rect is {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));

    let _square = Rectangle::square(3);
}

fn build_user(_email: String, username: String) -> User {
    User {
        _email,
        username,
        _active: true,
        _sign_in_count: 1,
    }
}
