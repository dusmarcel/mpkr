use leptos::prelude::*;

#[component]
pub fn Status(
    rg: Memo<Option<u32>>,
    set_rg: SignalSetter<Option<u32>>,
    gg: Memo<Option<u32>>,
    set_gg: SignalSetter<Option<u32>>,
    sk: Memo<Option<u32>>,
    set_sk: SignalSetter<Option<u32>>
) -> impl IntoView {
    let change_rg = move |ev: leptos::ev::Event| set_rg.set(Some(event_target_value(&ev).parse::<u32>().unwrap_or(0)));
    let change_gg = move |ev: leptos::ev::Event| set_gg.set(Some(event_target_value(&ev).parse::<u32>().unwrap_or(0)));
    let change_sk = move |ev: leptos::ev::Event| set_sk.set(Some(event_target_value(&ev).parse::<u32>().unwrap_or(0)));

    view! {
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Stand der anzuwendenen Normen"
            </h2>
            <p>
                <label for="rg_select">
                    "Welche Version des Rechtsanwaltsvergütungsgesetzes (RVG) soll verwendet werden? Maßgeblich ist der Zeitpunkt der Bauftragung. Wurde der Auftrag ab dem 01.06.2025 erteilt, ist die Fassung von 2025 zu verwenden. Bei Aufträgen, die zwischen dem 01.01.2021 und dem 31.05.2025 erteilt wurden, ist die Fassung von 2021 zu verwenden. Gebühren für Aufträge, die davor erteilt wurden, können mit diesem Rechner nicht berechnet werden."
                </label>
            </p>
            <p>
                <select class="p-1 border-2 border-stone-400 rounded-lg" aria-label="Auswahl der Fassung des RVG" id="rg_select" on:change=change_rg>
                    <option value="0" selected=move || rg.get().unwrap_or(0) == 0>"RVG 2025"</option>
                    <option value="1" selected=move || rg.get().unwrap_or(0) == 1>"RVG 2021"</option>
                </select>
            </p>
            <p>
                <label for="gg_select">
                    "Welche Version des Gerichtskostengesetzes (GKG) soll verwendet werden? Wenn ein Rechtsmittel ab dem 01.06.2025 anhängig gemacht wurde, ist die Version von 2025 zu verenden. Wurde das Rechtsmittel zwischen dem 01.01.2021 uns dem 31.05.2025 anhängig gemacht, ist die Version von 2021 zu verwenden. Gebühren für Rechtsmittel, die davor anhängig gemacht wurden, können mit diesem Rechner nicht berechnet werden."
                </label>
            </p>
            <p>
                <select class="p-1 border-2 border-stone-400 rounded-lg" aria-label="Auswahl der Fassung des GKG" id="gg_select" on:change=change_gg>
                    <option value="0" selected=move || gg.get().unwrap_or(0) == 0>"GKG 2025"</option>
                    <option value="1" selected=move || gg.get().unwrap_or(0) == 1>"GKG 2021"</option>
                </select>
            </p>
            <p>
                <label for="sk_select">
                    "Welche Version des Streitwertkatalogs soll verwendet werden? Wenn du unsicher bist, belasse es bei der aktuellen Version von 2025."
                </label>
            </p>
            <p>
                <select class="p-1 border-2 border-stone-400 rounded-lg" aria-label="Auswahl der Version des Streitwertkatalogs" id="sk_select" on:change=change_sk>
                    <option value="0" selected=move || sk.get().unwrap_or(0) == 0>"Streitwertkatalog 2025"</option>
                    <option value="1" selected=move || sk.get().unwrap_or(0) == 1>"Streitwertkatalog 2013"</option>
                </select>
            </p>
        </div>
    }
}