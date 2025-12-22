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
    let mut s = n.to_string();

    let negative = s.starts_with('-');
    if negative {
        s.remove(0);
    }

    let mut out = String::new();
    while s.len() > 3 {
        let tail = s.split_off(s.len() - 3);
        out = format!(",{}{}", tail, out);
    }

    out = format!("{}{}", s, out);

    if negative {
        out.insert(0, '-');
    }

    out
}
