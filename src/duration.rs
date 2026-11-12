/// Validate that a duration is positive and has fewer than 60 seconds.
fn validate_duration(minutes: f64, seconds: f64) -> Result<(), String> {
    if minutes < 0.0 || seconds < 0.0 {
        return Err("Minutes and seconds cannot be negative".to_string());
    }

    if seconds >= 60.0 {
        return Err("Seconds must be less than 60".to_string());
    }

    if minutes == 0.0 && seconds == 0.0 {
        return Err("Duration must be greater than 0.0".to_string());
    }

    Ok(())
}

/// Parse duration input.
pub fn parse_duration(input: &str) -> Result<f64, String> {
    let split_input: Vec<&str> = input.split(":").collect();

    if split_input.iter().count() == 3 {
        let hours = split_input[0];
        let minutes = split_input[1];
        let seconds = split_input[2];

        let hours: f64 = split_input[0]
            .parse()
            .map_err(|e| format!("Invalid hours '{hours}: {e}'"))?;
        let minutes: f64 = split_input[1]
            .parse()
            .map_err(|e| format!("Invalid hours '{minutes}: {e}'"))?;
        let seconds: f64 = split_input[2]
            .parse()
            .map_err(|e| format!("Invalid hours '{seconds}: {e}'"))?;
    } else {
        match input.split_once(":") {
            Some((minutes, seconds)) => {
                let minutes: f64 = minutes
                    .parse()
                    .map_err(|e| format!("Invalid minutes '{minutes}': {e}"))?;

                let seconds: f64 = seconds
                    .parse()
                    .map_err(|e| format!("Invalid seconds '{seconds}': {e}"))?;

                validate_duration(minutes, seconds)?;

                Ok(minutes + seconds / 60.0)
            }
            None => {
                let minutes: f64 = input
                    .parse()
                    .map_err(|e| format!("Unable to parse input '{input}': {e}"))?;

                validate_duration(minutes, 0.0)?;

                Ok(minutes)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Floats like 18 + 5/60 aren't exact, so compare with a small tolerance
    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-9,
            "expected {expected}, got {actual}"
        );
    }

    // --- Valid input: mm:ss ---

    #[test]
    fn parses_minutes_and_seconds() {
        assert_close(parse_duration("18:30").unwrap(), 18.5);
    }

    #[test]
    fn parses_seconds_with_leading_zero() {
        assert_close(parse_duration("18:05").unwrap(), 18.0 + 5.0 / 60.0);
    }

    #[test]
    fn parses_zero_seconds() {
        assert_close(parse_duration("18:00").unwrap(), 18.0);
    }

    #[test]
    fn parses_zero_minutes() {
        assert_close(parse_duration("0:45").unwrap(), 0.75);
    }

    #[test]
    fn parses_more_than_sixty_minutes() {
        assert_close(parse_duration("75:30").unwrap(), 75.5);
    }

    // --- Valid input: plain minutes (old format keeps working) ---

    #[test]
    fn parses_whole_minutes() {
        assert_close(parse_duration("18").unwrap(), 18.0);
    }

    #[test]
    fn parses_decimal_minutes() {
        assert_close(parse_duration("18.5").unwrap(), 18.5);
    }

    // --- Invalid input ---

    #[test]
    fn rejects_seconds_of_sixty_or_more() {
        assert!(parse_duration("18:60").is_err());
        assert!(parse_duration("18:75").is_err());
    }

    #[test]
    fn rejects_missing_seconds() {
        assert!(parse_duration("18:").is_err());
    }

    #[test]
    fn rejects_missing_minutes() {
        assert!(parse_duration(":30").is_err());
    }

    #[test]
    fn rejects_non_numeric_input() {
        assert!(parse_duration("abc").is_err());
        assert!(parse_duration("18:3x").is_err());
        assert!(parse_duration("ab:30").is_err());
    }

    #[test]
    fn rejects_empty_input() {
        assert!(parse_duration("").is_err());
    }

    #[test]
    fn rejects_negative_duration() {
        assert!(parse_duration("-5").is_err());
        assert!(parse_duration("-5:30").is_err());
        assert!(parse_duration("18:-30").is_err());
    }

    #[test]
    fn rejects_zero_duration() {
        // A zero duration would give a pace of 0:00 min/km
        assert!(parse_duration("0").is_err());
        assert!(parse_duration("0:00").is_err());
    }

    #[test]
    fn rejects_hours_format() {
        // h:mm:ss isn't supported (yet) - change this test if you add it
        assert!(parse_duration("1:05:30").is_err());
    }

    // --- validate_duration ---

    #[test]
    fn validate_accepts_last_second_of_minute() {
        assert!(validate_duration(18.0, 59.0).is_ok());
    }

    #[test]
    fn validate_reports_why_it_rejects() {
        assert_eq!(
            validate_duration(18.0, 60.0),
            Err("Seconds must be less than 60".to_string())
        );
        assert_eq!(
            validate_duration(-1.0, 0.0),
            Err("Minutes and seconds cannot be negative".to_string())
        );
        assert_eq!(
            validate_duration(0.0, 0.0),
            Err("Duration must be greater than 0.0".to_string())
        );
    }
}
