/// Calculate running pace in min/km.
pub fn calculate_pace(distance: f64, duration: f64) -> f64 {
    duration / (distance / 1000.0)
}

/// Format a pace in decimal minutes as mm:ss (e.g. 5.5 -> "5:30").
pub fn format_pace(pace: f64) -> String {
    let total_seconds: u64 = (pace * 60.0).round() as u64;
    format!("{}:{:02}", total_seconds / 60, total_seconds % 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- calculate_pace ---

    #[test]
    fn calculates_pace_per_kilometre() {
        // 5 km in 25 minutes is 5 min/km
        assert_eq!(calculate_pace(5000.0, 25.0), 5.0);
    }

    #[test]
    fn calculates_pace_for_partial_kilometre() {
        // 500 m in 3 minutes is 6 min/km
        assert_eq!(calculate_pace(500.0, 3.0), 6.0);
    }

    // --- format_pace ---

    #[test]
    fn formats_pace_as_minutes_and_seconds() {
        assert_eq!(format_pace(5.5), "5:30");
        assert_eq!(format_pace(5.0), "5:00");
        assert_eq!(format_pace(5.0 + 5.0 / 60.0), "5:05");
    }

    #[test]
    fn formats_pace_rounding_up_to_next_minute() {
        // 5:59.7 rounds to 6:00, not 5:60
        assert_eq!(format_pace(5.0 + 59.7 / 60.0), "6:00");
    }
}
