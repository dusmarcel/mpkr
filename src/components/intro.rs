use leptos::prelude::*;

#[component]
pub fn Intro() -> impl IntoView {
    view! {
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h1 class="pt-4 text-3xl font-medium">
                "Migrationsrechtlicher Prozesskostenrechner (Stand 2025)"
            </h1>
            <p>
                "Erstellt von "<a class="text-blue-600 hover:underline hover:text-violet-600" href="https://aufentha.lt">"Marcel Keienborg"</a>". Bitte beachte unbedingt auch die
                Hinweise unten auf dieser Seite."
            </p>
            <p>
                "Die hier abgebildeten Gebührensätze für Rechtsanwält*innen finden Anwendung, wenn die*der Anwält*in ab dem 01.06.2025 beauftragt wurde. Für Aufträge, die im Zeitraum zwischen 01.01.2021 und 31.05.2025 erteilt worden sind, habe ich "<a class="text-blue-600 hover:underline hover:text-violet-600" href="https://mpkr21.aufentha.lt">"hier auch einen Rechner"</a>" erstellt."
            </p>
        </div>
    }
}