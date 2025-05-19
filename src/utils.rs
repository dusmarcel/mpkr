use leptos::prelude::*;
use leptos_use::{use_intl_number_format, CurrencyDisplay, NumberStyle, UseIntlNumberFormatOptions};

pub fn format_euro(value: f64) -> String {
    use_intl_number_format(
        UseIntlNumberFormatOptions::default()
            .locale("de-DE")
            .style(NumberStyle::Currency)
            .currency("EUR")
            .currency_display(CurrencyDisplay::Code)
    )
    .format::<f64>(value)
    .get()
}