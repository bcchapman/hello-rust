#![allow(unused)]
mod front_of_house;
mod back_of_house;
mod customer;

pub fn new_customer() {
    customer::eat_at_restaurant();
}

fn deliver_order() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_customer() {
        new_customer();
        // Additional assertions can be added here to verify the behavior of eat_at_restaurant
    }
}
