use leptos::prelude::*;
use leptos_use::{use_intl_number_format, NumberStyle, UseIntlNumberFormatOptions};

pub fn format_euro(value: f64) -> String {
    use_intl_number_format(
        UseIntlNumberFormatOptions::default()
            .locale("de-DE")
            .style(NumberStyle::Currency)
            .currency("EUR")
    )
    .format::<f64>(value)
    .get()
}