#[cfg(test)]
mod tests {
    use crate::generate_line_number;

    #[test]
    fn returns_expected_total() {
        let response = generate_line_number("1abc2");
        assert_eq!(response, 10);
    }
}
