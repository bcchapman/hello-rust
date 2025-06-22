fn main() {
    // This is a simple Rust program that demonstrates basic concepts like variables, data types, and functions.
    run_section("Variables", variables);
    run_section("Data Types", data_types);
    run_section("Functions", functions);
}

fn run_section(title: &str, section_fn: fn()) {
    println!("---------------------------");
    println!("{}", title);
    println!("---------------------------");
    section_fn();
}

fn variables() {
    // let x = 5;
    let mut x;
    x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("There are {} seconds in 3 hours.", THREE_HOURS_IN_SECONDS);

    // Shadowing a variable
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The length of spaces is: {spaces}");

    // below doesn't work because `spaces` is immutable and of type str
    // let mut spaces = "   ";
    // spaces = spaces.len();

    // floating point numbers
    // f32 is a single-precision float, f64 is a double-precision float
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!(
        "The value of x is: {:.1}, and the value of y is: {:.1}",
        x, y
    );

    // Numeric operations
    // addition, subtraction, multiplication, division
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // Print the results of the numeric operations
    println!(
        "Sum: {}, Difference: {}, Product: {}, Quotient: {:.2}, Truncated: {}, Remainder: {}",
        sum, difference, product, quotient, truncated, remainder
    );
}

fn data_types() {
    // boolean types
    let t = true;
    let f: bool = false; // with explicit type annotation

    print!("Boolean values: t = {}, f = {}\n", t, f);

    // character type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    print!(
        "Character values: c = {}, z = {}, heart_eyed_cat = {}\n",
        c, z, heart_eyed_cat
    );

    // tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("Tuple values: x = {}, y = {:.1}, z = {}", x, y, z);

    // Accessing tuple elements directly
    let x = tup;
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!(
        "Direct tuple access: five_hundred = {}, six_point_four = {:.1}, one = {}",
        five_hundred, six_point_four, one
    );

    // array type
    let a = [1, 2, 3, 4, 5];
    println!(
        "Array values: a[0] = {}, a[1] = {}, a[2] = {}, a[3] = {}, a[4] = {}",
        a[0], a[1], a[2], a[3], a[4]
    );

    // You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:
    let a = [3; 5];
    println!(
        "Array with same value: a[0] = {}, a[1] = {}, a[2] = {}, a[3] = {}, a[4] = {}",
        a[0], a[1], a[2], a[3], a[4]
    );

    // Accessing array elements
    let first = a[0];
    let second = a[1];
    println!(
        "Accessed array elements: first = {}, second = {}",
        first, second
    );
}

fn functions() {
    println!("Hello, world!");

    another_function();
    another_function_with_argument(72, 27);

    // statements do not return values
    let x = 5; // This is a statement, not an expression
    println!("The value of x is: {x}");

    // this does not work because `let` is a statement, not an expression
    // and does not return a value
    // let y = let x: i32 = 5;

    // expressions return values
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    println!("The value of five() is: {}", five());
}

// rust doesn't support function overloading, so you can't have
// two functions with the same name but different parameters
fn another_function() {
    println!("\tAnother function.");
}

fn another_function_with_argument(x: i32, y: i32) {
    println!("The value of x is: {x} and the value of y is: {y}");
}

fn five() -> i32 {
    5 // no explicit return needed, the last expression is returned
    // 5; // however does not work if you use a semicolon
}
