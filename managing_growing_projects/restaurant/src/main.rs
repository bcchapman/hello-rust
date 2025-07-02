// can rename to avoid conflicts
use std::fmt::Result;
use std::io::Result as IoResult;

use restaurant;

// we can combine multiple imports from the same module
// use std::cmp::Ordering;
// use std::io;
// using below
// use std::{cmp::Ordering, io};

// we can also import all using a glob operator
// use std::collections::*;

fn main() {
    println!("Hello, world!");

    function1();
    function2();

    restaurant::new_customer();
}

fn function1() -> Option<Result> {
    // --snip--
    None
}

fn function2() -> Option<IoResult<()>> {
    // --snip--
    None
}