use leptos::prelude::*;

#[component]
pub fn Status() -> impl IntoView {
    view! {
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Stand der anzuwendenen Normen"
            </h2>
            <p>
                "Die hier abgebildeten Gebührensätze für Rechtsanwält*innen finden Anwendung, wenn die*der Anwält*in ab dem 01.06.2025 beauftragt wurde. Für Aufträge, die im Zeitraum zwischen 01.01.2021 und 31.05.2025 erteilt worden sind, habe ich "<a class="text-blue-600 hover:underline hover:text-violet-600" href="https://mpkr21.aufentha.lt">"hier auch einen Rechner"</a>" erstellt."
            </p>
            <p>
                "Der Rechner kann derzeit nur den Streitwertkatalog von 2013 anwenden und ist insofern leider nicht mehr gänzlich auf dem aktuellen Stand. Einen echten Unterschied dürfte das nur im Hinblick auf Streitigkeiten, die unbefristete Aufenthaltsrechte betreffen, ausmachen. Ein Update ist in Arbeit und wird hoffentlich in Kürze erscheinen."
            </p>
        </div>
    }
}