#[allow(unused_variables)]
fn main() {
    run_section("references", references);
}

fn run_section(title: &str, section_fn: fn()) {
    println!("---------------------------");
    println!("{}", title);
    println!("---------------------------");
    section_fn();
}

fn references() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    let mut s1: String = s1; // Reassigning to a mutable variable
    change_string(&mut s1);
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    // mutable borrow rules only allow one mutable reference at a time
    let r1 = &mut s1;
    // Uncommenting the following lines will cause a compile-time error due to mutable borrow rules
    //let r2 = &mut s1;

    // using scope we can create new references, just can't be simultaneously active
    {
        let r1 = &mut s1;
        println!("Mutable reference r1: {}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s1;
    println!("Mutable reference r2: {}", r2);

    // cannot have mutable and immutable references at the same time
    let r1 = &s1; // no problem
    let r2 = &s1; // no problem
    // below line would cause a compile-time error, only if used
    //let r3 = &mut s1; // BIG PROBLEM
    //println!("{}, {}, and {}", r1, r2, r3);

    let r1 = &s1; // no problem
    let r2 = &s1; // no problem

    println!("{}, {}", r1, r2);
    // No longer a BIG PROBLEM because  r1 and r2 are not used after this point
    let r3 = &mut s1; 

    // below line would cause a compile-time error, because dangling references are not allowed
    // let reference_to_nothing = dangle();

    // Instead, we can use a function that returns a String instead of a reference
    // which ensures that the String is valid for the lifetime of the variable
    let removed_reference = no_dangle();  
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str(", world!");
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}