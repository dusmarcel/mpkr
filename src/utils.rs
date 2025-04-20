use num_format::{Locale, ToFormattedString};

pub fn format_euro(value: f64) -> String {
    let rounded = format!("{:.2}", value);
    let parts: Vec<&str> = rounded.split('.').collect();
    let int_part = parts[0].parse().unwrap_or(0);
    let decimal_part = parts.get(1).unwrap_or(&"00");
    let formatted_int = int_part.to_formatted_string(&Locale::de);
    format!("{},{}", formatted_int, decimal_part)
}