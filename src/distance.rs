/// Parse distance input in kilometres.
pub fn parse_distance(distance: &str) -> Result<f64, String> {
    let distance: f64 = distance
        .parse()
        .map_err(|e| format!("Unable to parse distance '{distance}': {e}"))?;

    if distance.is_nan() {
        return Err("Distance must be a number".to_string());
    }

    if distance.is_infinite() {
        return Err("Distance cannot be infinite".to_string());
    }

    if distance <= 0.0 {
        return Err("Distance must be greater than 0.0".to_string());
    }

    Ok(distance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // --- parse_distance: valid input ---

    #[rstest]
    #[case::whole_kilometres("5", 5.0)]
    #[case::decimal_kilometres("3.16", 3.16)]
    #[case::half_marathon("21.0975", 21.0975)]
    #[case::leading_zero("0.5", 0.5)]
    #[case::trailing_dot("10.", 10.0)]
    #[case::smallest_positive("0.001", 0.001)]
    fn parses_valid_distance(#[case] input: &str, #[case] expected: f64) {
        assert_eq!(parse_distance(input).unwrap(), expected);
    }

    // --- parse_distance: invalid input ---

    #[rstest]
    #[case::empty("")]
    #[case::non_numeric("abc")]
    #[case::with_unit("5km")]
    #[case::comma_as_decimal_separator("3,16")]
    #[case::zero("0")]
    #[case::zero_with_decimals("0.0")]
    #[case::negative("-5")]
    #[case::negative_zero("-0")]
    // Rust parses these as valid f64 values, but they are not real distances
    #[case::not_a_number("NaN")]
    #[case::infinite("inf")]
    #[case::negative_infinite("-inf")]
    fn rejects_invalid_distance(#[case] input: &str) {
        let result = parse_distance(input);
        assert!(
            result.is_err(),
            "expected '{input}' to be rejected, got {result:?}"
        );
    }

    #[test]
    fn error_names_the_input_that_failed_to_parse() {
        let error = parse_distance("abc").unwrap_err();
        assert!(error.contains("'abc'"), "got: {error}");
    }
}
