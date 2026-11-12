/// Validate that a duration is positive, is not zero and has less than 60 seconds.
fn validate_duration(hours: f64, minutes: f64, seconds: f64) -> Result<(), String> {
    if minutes < 0.0 || seconds < 0.0 || hours < 0.0 {
        return Err("Hours, minutes and seconds cannot be negative".to_string());
    }

    if seconds >= 60.0 {
        return Err("Seconds must be less than 60".to_string());
    }

    if hours == 0.0 && minutes == 0.0 && seconds == 0.0 {
        return Err("Duration must be greater than 0.0".to_string());
    }

    Ok(())
}

/// Parse a part of the duration input string to f64.
fn parse_part(value: &str, name: &str) -> Result<f64, String> {
    value
        .parse()
        .map_err(|e| format!("Invalid {name} '{value}': {e}"))
}

/// Parse duration input to duration in minutes.
pub fn parse_duration(input: &str) -> Result<f64, String> {
    let parts: Vec<&str> = input.split(":").collect();

    match parts.as_slice() {
        [hours, minutes, seconds] => {
            let hours: f64 = parse_part(hours, "hours")?;
            let minutes: f64 = parse_part(minutes, "minutes")?;
            let seconds: f64 = parse_part(seconds, "seconds")?;

            if minutes >= 60.0 {
                return Err("Minutes must be less than 60".to_string());
            }

            validate_duration(hours, minutes, seconds)?;

            Ok(hours * 60.0 + minutes + seconds / 60.0)
        }
        [minutes, seconds] => {
            let minutes: f64 = parse_part(minutes, "minutes")?;
            let seconds: f64 = parse_part(seconds, "seconds")?;

            validate_duration(0.0, minutes, seconds)?;

            Ok(minutes + seconds / 60.0)
        }
        [minutes] => {
            let minutes: f64 = parse_part(minutes, "minutes")?;

            validate_duration(0.0, minutes, 0.0)?;

            Ok(minutes)
        }
        _ => Err("Invalid duration input".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // Floats like 18 + 5/60 aren't exact, so compare with a small tolerance
    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-9,
            "expected {expected}, got {actual}"
        );
    }

    // --- parse_duration: valid input ---

    #[rstest]
    // plain minutes
    #[case::whole_minutes("18", 18.0)]
    #[case::decimal_minutes("18.5", 18.5)]
    // mm:ss
    #[case::minutes_and_seconds("18:30", 18.5)]
    #[case::seconds_with_leading_zero("18:05", 18.0 + 5.0 / 60.0)]
    #[case::zero_seconds("18:00", 18.0)]
    #[case::zero_minutes("0:45", 0.75)]
    #[case::more_than_sixty_minutes("75:30", 75.5)]
    // h:mm:ss
    #[case::hours_minutes_and_seconds("1:05:30", 65.5)]
    #[case::whole_hours("2:00:00", 120.0)]
    #[case::zero_hours("0:18:30", 18.5)] // same as 18:30
    #[case::zero_hours_and_minutes("0:00:45", 0.75)]
    #[case::last_minute_and_second_of_hour("1:59:59", 60.0 + 59.0 + 59.0 / 60.0)]
    #[case::hours_with_leading_zero("02:30:30", 120.0 + 30.0 + 30.0 / 60.0)]
    fn parses_valid_duration(#[case] input: &str, #[case] expected: f64) {
        assert_close(parse_duration(input).unwrap(), expected);
    }

    // --- parse_duration: invalid input ---

    #[rstest]
    #[case::empty("")]
    #[case::non_numeric("abc")]
    // mm:ss
    #[case::seconds_of_sixty("18:60")]
    #[case::seconds_over_sixty("18:75")]
    #[case::missing_seconds("18:")]
    #[case::missing_minutes(":30")]
    #[case::non_numeric_seconds("18:3x")]
    #[case::non_numeric_minutes("ab:30")]
    #[case::negative_minutes("-5")]
    #[case::negative_minutes_with_seconds("-5:30")]
    #[case::negative_seconds("18:-30")]
    // A zero duration would give a pace of 0:00 min/km
    #[case::zero_minutes("0")]
    #[case::zero_minutes_and_seconds("0:00")]
    // h:mm:ss - unlike mm:ss, minutes must stay below 60 once hours are present
    #[case::minutes_of_sixty_with_hours("1:60:00")]
    #[case::minutes_over_sixty_with_hours("1:75:00")]
    #[case::seconds_of_sixty_with_hours("1:05:60")]
    #[case::missing_hours(":05:30")]
    #[case::missing_minutes_with_hours("1::30")]
    #[case::missing_seconds_with_hours("1:05:")]
    #[case::only_separators("::")]
    #[case::non_numeric_hours("a:05:30")]
    #[case::non_numeric_minutes_with_hours("1:0b:30")]
    #[case::non_numeric_seconds_with_hours("1:05:3c")]
    #[case::negative_hours("-1:05:30")]
    #[case::negative_minutes_with_hours("1:-5:30")]
    #[case::negative_seconds_with_hours("1:05:-30")]
    #[case::zero_hours_minutes_and_seconds("0:00:00")]
    #[case::more_than_three_parts("1:05:30:00")]
    fn rejects_invalid_duration(#[case] input: &str) {
        let result = parse_duration(input);
        assert!(
            result.is_err(),
            "expected '{input}' to be rejected, got {result:?}"
        );
    }

    #[rstest]
    #[case::hours("x:05:30", "Invalid hours 'x'")]
    #[case::minutes("1:y:30", "Invalid minutes 'y'")]
    #[case::seconds("1:05:z", "Invalid seconds 'z'")]
    fn errors_name_the_part_that_failed_to_parse(#[case] input: &str, #[case] expected: &str) {
        let error = parse_duration(input).unwrap_err();
        assert!(error.contains(expected), "got: {error}");
    }

    // --- validate_duration ---

    #[rstest]
    #[case::last_second_of_minute(0.0, 18.0, 59.0)]
    // Zero minutes and seconds is fine as long as there are hours
    #[case::whole_hours(1.0, 0.0, 0.0)]
    fn validate_accepts(#[case] hours: f64, #[case] minutes: f64, #[case] seconds: f64) {
        assert!(validate_duration(hours, minutes, seconds).is_ok());
    }

    #[rstest]
    #[case::seconds_of_sixty(0.0, 18.0, 60.0, "Seconds must be less than 60")]
    #[case::negative_hours(-1.0, 5.0, 30.0, "Hours, minutes and seconds cannot be negative")]
    #[case::negative_minutes(0.0, -1.0, 0.0, "Hours, minutes and seconds cannot be negative")]
    #[case::negative_seconds(0.0, 1.0, -1.0, "Hours, minutes and seconds cannot be negative")]
    #[case::zero(0.0, 0.0, 0.0, "Duration must be greater than 0.0")]
    fn validate_reports_why_it_rejects(
        #[case] hours: f64,
        #[case] minutes: f64,
        #[case] seconds: f64,
        #[case] expected: &str,
    ) {
        assert_eq!(
            validate_duration(hours, minutes, seconds),
            Err(expected.to_string())
        );
    }
}
