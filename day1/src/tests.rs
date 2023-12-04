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

    #[test]
    fn handles_string_characters() {
        let response: u32 = generate_line_number("onetwo");
        assert_eq!(response, 12);
    }

    #[test]
    fn handles_multiple_string_characters() {
        let response: u32 = generate_line_number("onetwoseven");
        assert_eq!(response, 17);
    }

    #[test]
    fn handles_same_characters_multiple_times() {
        let response: u32 = generate_line_number("oneone");
        assert_eq!(response, 17);
    }

    #[test]
    fn handles_multiple_string_and_digit_characters() {
        let response: u32 = generate_line_number("acsd9twoseven");
        assert_eq!(response, 97);
    }
}
