fn main() {
    println!("Hello, world!");

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    if let IpAddr::V4(addr1, addr2, addr3, addr4) = &home {
        println!("Home string value: {}.{}.{}.{}", addr1, addr2, addr3, addr4);
    }

    if let IpAddr::V6(addr) = &loopback {
        println!("Loopback string value: {}", addr);
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // Options
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("some_char: {:?}", some_char);
    println!("absent_number: {:?}", absent_number);

    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let ala_quarter = Coin::Quarter(UsState::Alabama);
    let ak_quarter = Coin::Quarter(UsState::Alaska);

    // print each coin value
    println!("Penny value: {}", value_in_cents(&penny));
    println!("Nickel value: {}", value_in_cents(&nickel));
    println!("Dime value: {}", value_in_cents(&dime));
    println!("Quarter value: {}", value_in_cents(&ala_quarter));

    if let Some(description) = describe_state_quarter(&ala_quarter) {
        println!("{}", description);
    }
    if let Some(description) = describe_state_quarter(&ak_quarter) {
        println!("{}", description);
    }
    
    // try to describe a non-quarter coin
    if let Some(description) = describe_state_quarter(&dime) {
        println!("{}", description);
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five: {:?}", five);
    println!("six: {:?}", six);
    println!("none: {:?}", none);

    // match arms are evaluated from top to bottom and must be exhaustive
    let dice_roll = 3;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {
        println!("Added a fancy hat!")
    }
    fn remove_fancy_hat() {
        println!("Removed the fancy hat!")
    }

    let config_max = Some(3u8);
    // requires boiler plate code to destructure
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // alternatively use if let to check for one pattern while ignoring all others
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("Call method executed.");
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska, // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn describe_state_quarter(coin: &Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

// adds one to the value inside an Option<i32>, returning None if the value is None
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // all cases must be covered, if we comment below it wont compile
        None => None,
        Some(i) => Some(i + 1),
    }
}
