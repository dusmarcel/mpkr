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
                "Stand der anzuwendenden Normen"
            </h2>
            <p>
                "Der Rechner verwendet standardmäßig die Fassungen von Rechtsanwaltsvergütungsgesetz (RVG),
                Gerichtskostengesetz (GKG) und Streitwertkatalog aus 2025. Wenn du dir unsicher bist, belasse es dabei.
                Für die richtige Fassung des RVG kommt es auf den Zeitpunkt der Bauftragung an. Für das GKG ist der
                Zeitpunkt, zu dem das Rechtsmittel anhängig gemacht wurde, maßgeblich. Liegt dieser Zeitpunt im Juni 2025 oder 
                später, ist die Version von 2025 anzuwenden. Liegt der Zeitpunkt  zwischen dem 01.01.2021 und dem 31.05.2025,
                ist die Fassung von 2021 zu verwenden. Gebühren für noch ältere Angelegenheiten können mit diesem Rechner nicht
                ermittelt werden. Zudem kann es praktisch auch durchaus vorkommen, dass auf unterschiedliche Instanzen
                unterschiedliche Normfassungen anzuwenden sind. Aus Praktikabilitätsgründen kann hier aber immer nur eine 
                Normfassung ausgewählt werden, die dann auf alle Instanzen angewendet wird."
            </p>
            <p class="flex flex-row">
                <div class="mr-2">
                    <select class="p-1 border-2 border-stone-400 rounded-lg" aria-label="Auswahl der Fassung des RVG" id="rg_select" on:change=change_rg>
                        <option value="0" selected=move || rg.get().unwrap_or(0) == 0>"RVG 2025"</option>
                        <option value="1" selected=move || rg.get().unwrap_or(0) == 1>"RVG 2021"</option>
                    </select>
                </div>
                <div class="mr-2">
                    <select class="p-1 border-2 border-stone-400 rounded-lg" aria-label="Auswahl der Fassung des GKG" id="gg_select" on:change=change_gg>
                        <option value="0" selected=move || gg.get().unwrap_or(0) == 0>"GKG 2025"</option>
                        <option value="1" selected=move || gg.get().unwrap_or(0) == 1>"GKG 2021"</option>
                    </select>
                </div>
                <div>
                    <select class="p-1 border-2 border-stone-400 rounded-lg" aria-label="Auswahl der Version des Streitwertkatalogs" id="sk_select" on:change=change_sk>
                        <option value="0" selected=move || sk.get().unwrap_or(0) == 0>"Streitwertkatalog 2025"</option>
                        <option value="1" selected=move || sk.get().unwrap_or(0) == 1>"Streitwertkatalog 2013"</option>
                    </select>
                </div>
            </p>
        </div>
    }
}