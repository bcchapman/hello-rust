
// returns true if successful, false if waitlist is full
pub fn add_to_waitlist() -> bool {
    true
}

fn seat_at_table() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // This test checks if the `add_to_waitlist` function works correctly.
    fn test_add_to_waitlist() {
        let result = add_to_waitlist();
        assert!(result, "Expected add_to_waitlist to return true");
    }
}
