use core::fmt::Display;
use std::time::Duration;

/// Format a [`Duration`] to HH:MM:SS.
pub(crate) fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    format!("{:02}h {:02}m {:02}s", hours, minutes, seconds)
}

pub(crate) fn format_thousands<T: Display>(n: T) -> String {
    let s = n.to_string();
    let negative = s.starts_with('-');

    // Split on decimal point if it exists.
    let parts: Vec<&str> = s.trim_start_matches('-').split('.').collect();
    let mut integer_part = parts[0].to_string();
    let decimal_part = if parts.len() > 1 {
        format!(".{}", parts[1])
    } else {
        String::new()
    };

    // Format the integer part with commas.
    let mut out = String::new();
    while integer_part.len() > 3 {
        let tail = integer_part.split_off(integer_part.len() - 3);
        out = format!(",{}{}", tail, out);
    }
    out = format!("{}{}", integer_part, out);

    // Add back negative sign and decimal part.
    if negative {
        out.insert(0, '-');
    }
    format!("{}{}", out, decimal_part)
}
