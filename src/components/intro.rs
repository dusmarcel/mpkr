use leptos::prelude::*;

#[component]
pub fn Intro() -> impl IntoView {
    view! {
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h1 class="pt-4 text-3xl font-medium">
                "Migrationsrechtlicher Prozesskostenrechner"
            </h1>
            <p>
                "Erstellt von "<a class="text-violet-800 hover:underline hover:text-violet-900" href="https://aufentha.lt">"Marcel Keienborg"</a>". Bitte beachte unbedingt auch die
                Hinweise unten auf dieser Seite."
            </p>
        </div>
    }
}