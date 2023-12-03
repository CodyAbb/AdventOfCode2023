#[cfg(test)]
mod tests {
    use crate::generate_line_number;

    #[test]
    fn returns_two_digit_total() {
        let response: u32 = generate_line_number("1abc2");
        assert_eq!(response, 12);
    }

    #[test]
    fn returns_one_digit_total() {
        let response: u32 = generate_line_number("a7bc");
        assert_eq!(response, 77);
    }

    #[test]
    fn handles_no_digit_line() {
        let response: u32 = generate_line_number("abc");
        assert_eq!(response, 0);
    }
}
