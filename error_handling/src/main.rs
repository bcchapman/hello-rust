use std::fs::File;
use std::fs;
use std::io::{self, ErrorKind, Read};

fn main() {
    println!("Hello, world!");

    //force_panic();

    // Call the function to avoid unused warning
    // invalid_array_index();

    let file_path = "hello.txt";
    //read_using_match(file_path);

    // using unwrape will provide panic content for you
    // let _ = File::open(file_path).unwrap();

    // alternatively you can use expect to customize message
    //let _ = File::open(file_path)
    //    .expect(format!("{} should be included in this project", file_path).as_str());

    // let username = match read_username_from_file(file_path) {
    let username = match read_username_from_file_simplified(file_path) {
        Ok(username) => username,
        Err(e) => panic!("Encountered error. {e:?}")
    };

    println!("Found username {} ", username);
    println!("{:?}", last_char_of_first_line(""));
    println!("{:?}", last_char_of_first_line("abcd"));
}

// an example of propogating error
// this allows caller to decide how to handle error, instead of actual implementation
// however, very verbose
#[allow(dead_code)]
fn read_username_from_file(file_path: &str) -> Result<String, io::Error> {
    let username_file_result = File::open(file_path);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// can also use ? on optional
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

#[allow(dead_code)]
fn read_username_from_file_simplified(file_path: &str) -> Result<String, io::Error> {
    // let mut username = String::new();
    // option 1 
    // let mut username_file = File::open(file_path)?;
    // username_file.read_to_string(&mut username)?;

    // option 2
    // File::open(file_path)?.read_to_string(&mut username)?;

    // option 3
    fs::read_to_string(file_path)
}

#[allow(dead_code)]
fn read_using_match(file_path: &str) {
    let greeting_file_result = File::open(file_path);

    let _ = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("File not found, creating a new one");

                match File::create(file_path) {
                    Ok(file) => file,
                    Err(e) => panic!("Problem creating the file: {e:?}"),
                }
            }
            _ => panic!("Problem opening the file: {error:?}"),
        },
    };

    // can be cleaned up with an unwrap_or_else, revisit in chapter 13
    let _ = File::open(file_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_path).unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

#[allow(dead_code)]
fn force_panic() {
    panic!("Goodbye, world");
}

#[allow(dead_code)]
fn invalid_array_index() {
    // access invalid array
    let v = vec![1, 2, 3];
    v[99];
}
