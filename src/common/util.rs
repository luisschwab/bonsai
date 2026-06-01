use core::fmt::Display;
use std::time::Duration;

use bitcoin::Amount;

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

pub(crate) fn format_bytes(bytes: usize) -> String {
    if bytes < 1_000 {
        format!("{} BYTES", bytes)
    } else if bytes < 1_000_000 {
        format!("{:.2} KB", bytes as f64 / 1_000.0)
    } else {
        format!("{:.2} MB", bytes as f64 / 1_000_000.0)
    }
}

pub(crate) fn format_btc(amount: Amount) -> String {
    format!(
        "{} BTC",
        format_thousands(format!("{:.2}", amount.to_btc()))
    )
}

pub(crate) fn split_hash_64(hex: impl AsRef<str>) -> String {
    let hex = hex.as_ref();
    if hex.len() == 64 {
        format!("{}\n{}", &hex[..32], &hex[32..])
    } else {
        hex.to_string()
    }
}

pub(crate) fn parse_formatted_u32(value: &str) -> Option<u32> {
    value.replace(",", "").parse::<u32>().ok()
}

#[cfg(test)]
mod tests {
    use bitcoin::Amount;

    use super::format_btc;
    use super::format_bytes;
    use super::parse_formatted_u32;
    use super::split_hash_64;

    #[test]
    fn formats_byte_counts() {
        assert_eq!(format_bytes(999), "999 BYTES");
        assert_eq!(format_bytes(1_500), "1.50 KB");
        assert_eq!(format_bytes(2_500_000), "2.50 MB");
    }

    #[test]
    fn formats_btc_with_thousands_separator() {
        assert_eq!(
            format_btc(Amount::from_sat(123_456_789_000)),
            "1,234.57 BTC"
        );
    }

    #[test]
    fn splits_64_char_hashes_only() {
        let hash = "0".repeat(64);
        assert_eq!(
            split_hash_64(&hash),
            format!("{}\n{}", "0".repeat(32), "0".repeat(32))
        );
        assert_eq!(split_hash_64("abc"), "abc");
    }

    #[test]
    fn parses_comma_formatted_u32() {
        assert_eq!(parse_formatted_u32("123,456"), Some(123_456));
        assert_eq!(parse_formatted_u32("abc"), None);
    }
}
