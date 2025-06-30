fn main() {
    // create a User
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    // print user1 details
    println!(
        "User Details:\nActive: {}\nUsername: {}\nEmail: {}\nSign In Count: {}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );

    // create another user
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // using the struct update syntax to copy fields from user1
    };
    println!("---------------------------");

    // print user2 details
    println!(
        "User Details:\nActive: {}\nUsername: {}\nEmail: {}\nSign In Count: {}",
        user2.active, user2.username, user2.email, user2.sign_in_count
    );

    // user1 is no longer accessible here because we used `..user1` in user2 creation
    // If we had given user2 new String values for both email and username, and thus only
    // used the active and sign_in_count values from user1, then user1 would still be valid after creating user2.
    // Both active and sign_in_count are types that implement the Copy trait
    // println!("{}", user1.username); // This line would cause a compile error if uncommented, as user1 is no longer valid.

    // Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields;
    // rather, they just have the types of the fields
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!(
        "Color: ({}, {}, {})\nPoint: ({}, {}, {})",
        black.0, black.1, black.2, origin.0, origin.1, origin.2
    );

    // Unit-like structs are useful when you need to implement a trait on some type but don’t need any data
    // associated with that type. They are also useful in generics.
    struct AlwaysEqual;

    #[allow(unused)]
    let subject = AlwaysEqual;

    example();
}

//Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // because of the field init shorthand, we can use just `username` instead of `username: username`
        email,    // same for email
        sign_in_count: 1,
    }
}

fn example() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1),
    );

    println!(
        "The area of the rectangle using a tuple is {} square pixels.",
        rect_area((width1, height1)),
    );

    println!("rect is {:#?}", rect);
    dbg!(&rect);

    println!(
        "The area of the rectangle using a rect is {} square pixels.",
        rect_struct_area(&rect),
    );

    println!(
        "The area of the rectangle using an impl is {} square pixels.",
        rect.area()
    );

    // rust knows when you don't include the parentheses that you want to call the method
    // and not the field, so it will call the method instead of accessing the field directly
    // without the parentheses, it accesses the field directly
    println!(
        "The rectangle has a width of {}. It is {}",
        if rect.width() {
            "greater than 0"
        } else {
            "0 or less"
        },
        rect.width
    );

    // multiple parameters to function
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Does rect1 hold rect2? {}", rect1.can_hold(&rect2));

    println!("Does rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(10);
    println!("The area of the square is {} square pixels.", square.area());

    println!("Does rect1 hold square? {}", rect1.can_hold(&square));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// we don't need to use separate impl blocks, but it is valid to do so
// best practice is to keep all methods related to the struct in the same impl block
impl Rectangle {
    // don't need use `self` here because we are not using any instance data
    // this is a static method, it does not require an instance of the struct to be called
    // it is similar to a constructor in other languages
    fn square(size: u32) -> Self {
        Self {
            // Self is a shorthand for Rectangle here
            width: size,
            height: size,
        }
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn rect_area(rectangle: (u32, u32)) -> u32 {
    rectangle.0 * rectangle.1
}

fn rect_struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
