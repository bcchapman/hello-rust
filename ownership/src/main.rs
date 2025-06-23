fn main() {
    run_section("Ownership", ownership);
    run_section("References", references);
}

fn run_section(title: &str, section_fn: fn()) {
    println!("---------------------------");
    println!("{}", title);
    println!("---------------------------");
    section_fn();
}

fn ownership() {
      {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
        println!("s is: {}", s);
    }
    // s is not valid here, it went out of scope and was dropped

    let mut s = String::from("hello");
    println!("s is: {}", s);
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("s is now: {}", s);

    let s1 = String::from("hello");
    let s2 = s1;

    // s1 is no longer valid here, as ownership has been moved to s2
    println!("{s2}, world!");

    #[allow(unused_assignments)]
    let mut s = String::from("hello");
    // existing memory is dropped, and new memory in heap is allocated
    s = String::from("ahoy");

    println!("{s}, world!");

    // use clone to deep copy
    // s1 is still valid here, as clone creates a deep copy
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // ownership rules don't apply to primitive types with fixed size
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");

    // Ownership and Functions
    // When a variable is passed to a function, ownership can be transferred.
    let s = String::from("hello"); // s comes into scope

    // s's value moves into the function
    //and so is no longer valid here
    takes_ownership(s);

    // This line will not compile)
    // println!("This line will not compile s is out of scope {s}!");

    let x = 5; // x comes into scope
    // because i32 implements the Copy trait,
    // x does NOT move into the function,
    makes_copy(x);

    println!("{}", x); // so it's okay to use x afterward

    // Returning Values and Scope
    // gives_ownership moves its return value into s1
    let s1 = gives_ownership();
    println!("s1 is: {s1}");

    let s2 = String::from("hello"); // s2 comes into scope

    // s2 is moved into takes_and_gives_back,
    // which also moves its return value into s3
    let s3 = takes_and_gives_back(s2);
    println!("s3 is: {s3}");

    // How to use a value in function by returning it back in tuple.
    // This is not ideal and can be replaced with references.
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

// gives_ownership will move its
// return value into the function
// that calls it
fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string comes into scope

    // some_string is returned and moves out to the calling function
    some_string
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn references() {

}