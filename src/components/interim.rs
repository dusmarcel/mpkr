use leptos::prelude::*;

use crate::utils::format_euro;
use crate::popover;

// Vorläufiger Rechtsschutz

#[component]
pub fn Interim(
    v: Memo<Option<u32>>,
    v1: Memo<Option<bool>>,
    set_v1: SignalSetter<Option<bool>>,
    v2: Memo<Option<bool>>,
    set_v2: SignalSetter<Option<bool>>,
    n3100v: Memo<Option<bool>>,
    set_n3100v: SignalSetter<Option<bool>>,
    n3101v: Memo<Option<bool>>,
    set_n3101v: SignalSetter<Option<bool>>,
    verfgeb13_v1: Memo<f64>,
    verfgeb49_v1: Memo<f64>,
    n3104v: Memo<Option<bool>>,
    set_n3104v: SignalSetter<Option<bool>>,
    tgeb13_v1: Memo<f64>,
    tgeb49_v1: Memo<f64>,
    v1p: Memo<Option<bool>>,
    set_v1p: SignalSetter<Option<bool>>,
    pauschale13_v1: Memo<f64>,
    pauschale49_v1: Memo<f64>,
    v1a: Memo<Option<bool>>,
    set_v1a: SignalSetter<Option<bool>>,
    v1sa: Memo<Option<f64>>,
    set_v1sa: SignalSetter<Option<f64>>,
    n5210: Memo<Option<bool>>,
    set_n5210: SignalSetter<Option<bool>>,
    n5211: Memo<Option<bool>>,
    set_n5211: SignalSetter<Option<bool>>,
    gkg_v1: Memo<f64>,
    n3200v: Memo<Option<bool>>,
    set_n3200v: SignalSetter<Option<bool>>,
    n3201v: Memo<Option<bool>>,
    set_n3201v: SignalSetter<Option<bool>>,
    verfgeb13_v2: Memo<f64>,
    verfgeb49_v2: Memo<f64>,
    n3202v: Memo<Option<bool>>,
    set_n3202v: SignalSetter<Option<bool>>,
    tgeb13_v2: Memo<f64>,
    tgeb49_v2: Memo<f64>,
    v2p: Memo<Option<bool>>,
    set_v2p: SignalSetter<Option<bool>>,
    pauschale13_v2: Memo<f64>,
    pauschale49_v2: Memo<f64>,
    v2a: Memo<Option<bool>>,
    set_v2a: SignalSetter<Option<bool>>,
    v2sa: Memo<Option<f64>>,
    set_v2sa: SignalSetter<Option<f64>>,
    n5240: Memo<Option<bool>>,
    set_n5240: SignalSetter<Option<bool>>,
    n5241: Memo<Option<bool>>,
    set_n5241: SignalSetter<Option<bool>>,
    gkg_v2: Memo<f64>,
    summe_rvg13_v: Memo<f64>,
    summe_rvg49_v: Memo<f64>,
    summe_gkg_v: Memo<f64>
) -> impl IntoView {
    let change_v1 = move |ev| set_v1.set(Some(event_target_checked(&ev)));
    let change_v2 = move |ev| set_v2.set(Some(event_target_checked(&ev)));
    let change_n3100v = move |ev| set_n3100v.set(Some(event_target_checked(&ev)));
    let change_n3101v = move |ev| set_n3101v.set(Some(event_target_checked(&ev)));
    let change_n3104v = move |ev| set_n3104v.set(Some(event_target_checked(&ev)));
    let change_v1_pauschale = move |ev| set_v1p.set(Some(event_target_checked(&ev)));
    let change_v1_auslagen = move |ev| set_v1a.set(Some(event_target_checked(&ev)));
    let change_v1_sonstige_auslagen = move |ev| {
        set_v1sa.set(Some(event_target_value(&ev).parse::<f64>().unwrap_or(0.0)));
        if event_target_value(&ev).parse::<f64>().unwrap_or(0.0) != 0.0 {
            set_v1a.set(Some(true));
        } else {
            set_v1a.set(Some(false));
        }
    };
    let change_n5210 = move |ev| set_n5210.set(Some(event_target_checked(&ev)));
    let change_n5211 = move |ev| set_n5211.set(Some(event_target_checked(&ev)));
    let change_n3200v = move |ev| set_n3200v.set(Some(event_target_checked(&ev)));
    let change_n3201v = move |ev| set_n3201v.set(Some(event_target_checked(&ev)));
    let change_n3202v = move |ev| set_n3202v.set(Some(event_target_checked(&ev)));
    let change_v2_pauschale = move |ev| set_v2p.set(Some(event_target_checked(&ev)));
    let change_v2_auslagen = move |ev| set_v2a.set(Some(event_target_checked(&ev)));
    let change_v2_sonstige_auslagen = move |ev| {
        set_v2sa.set(Some(event_target_value(&ev).parse::<f64>().unwrap_or(0.0)));
        if event_target_value(&ev).parse::<f64>().unwrap_or(0.0) != 0.0 {
            set_v2a.set(Some(true));
        } else {
            set_v2a.set(Some(false));
        }
    };
    let change_n5240 = move |ev| set_n5240.set(Some(event_target_checked(&ev)));
    let change_n5241 = move |ev| set_n5241.set(Some(event_target_checked(&ev)));

    view! {
        <div class=move || if v.get().unwrap_or(0) != 0 { // Container einblenden, wenn nicht "nur Hauptsacheverfahren" ausgewählt ist 
                "container visible max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300"
            } else {
                "hidden"
            }
        >
            <h2 class="pt-4 text-2xl font-medium">
                "Vorläufiger Rechtsschutz"
            </h2>
            <p>
                <input
                    type="checkbox"
                    id="v1"
                    on:change=change_v1
                    prop:checked=move || v1.get().unwrap_or(true)
                />
                <span class="mx-1">1. Instanz</span>
                <input
                    type="checkbox"
                    id="v2"
                    on:change=change_v2
                    prop:checked=move || v2.get().unwrap_or(false)
                />
                <span class="mx-1">2. Instanz</span>
            </p>
            <p class=move || if v1.get().unwrap_or(true) { "visible" } else { "hidden" }>
                <h3 class="text-xl font-medium">
                    "1. Instanz"
                </h3>
                <h4 class="text-l font-bold">
                    "Rechtsanwaltsvergütungsgesetz"
                </h4>
                <table class="table-auto">
                    <thead>
                        <tr class="text-left">
                            <th>
                            </th>
                            <th>
                                "Gebührentatbestand und Nummer"
                            </th>
                            <th class="px-1">
                                "Gebührensatz"
                            </th>
                            <th class="px-1">
                                "Wertgebühr (§ 13 RVG)"
                            </th>
                            <th class="px-1">
                                "Wertgebühr (§ 49 RVG)"
                            </th>
                            <th class="pl-1">
                                "Differenz"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3100v"
                                    on:change=change_n3100v
                                    prop:checked=move || n3100v.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3100v">"Verfahrensgebühr, Nr. 3100"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,3"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb13_v1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb49_v1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb13_v1.get() - verfgeb49_v1.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3101"
                                    on:change=change_n3101v
                                    prop:checked=move || n3101v.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3101">"Ermäßigte Verfahrensgebühr, Nr. 3101"</label>
                                <button popovertarget="ermaessigung3101v" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung3101v" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Verfahrensgebühr Nr. 3100"</h4>
                                    <p>{ popover::ERMAESSIGUNG3101 }</p>
                                </div>
                            </td>
                            <td class="px-1 text-right">
                                "0,8"
                            </td>
                            <td></td>
                            <td></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3104v"
                                    on:change=change_n3104v
                                    prop:checked=move || n3104v.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3104v">"Terminsgebühr, Nr. 3104"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,2"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb13_v1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb49_v1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb13_v1.get() - tgeb49_v1.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="v1_pauschale"
                                    on:change=change_v1_pauschale
                                    prop:checked=move || v1p.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="v1_pauschale">"Auslagenpauschale, Nr. 7002"</label>
                            </td>
                            <td></td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale13_v1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_v1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_v1.get() - pauschale13_v1.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="v1_auslagen"
                                    on:change=change_v1_auslagen
                                    prop:checked=move || v1a.get().unwrap_or(false)
                                />
                            </td>
                            <td colspan="2" class="px-1">
                                <label for="v1_auslagen">"Sonstige Auslagen"</label>
                                <button popovertarget="v1auslagen" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                                <div id="v1auslagen" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Sonstige Auslagen"</h4>
                                    <p>"Zum Beispiel:"
                                        <ul>
                                            <li>"7000 Pauschale für die Herstellung und Überlassung von Dokumenten:"
                                                <ul>
                                                    <li>"für Kopien und Ausdrucke"</li>        
                                                    <li>"für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR"</li>
                                                    <li>"für jede weitere Seite 0,15 EUR"</li>
                                                    <li>"für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR"</li>
                                                    <li>"für jede weitere Seite in Farbe 0,30 EUR"</li>
                                                </ul>
                                            </li>
                                            <li>"7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR."</li>
                                            <li>"7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe."</li>
                                            <li>"7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise"
                                                <ol>
                                                    <li>"von nicht mehr als 4 Stunden 30,00 EUR"</li>
                                                    <li>"von mehr als 4 bis 8 Stunden 50,00 EUR"</li>
                                                    <li>"von mehr als 8 Stunden 80,00 EUR"</li>
                                                </ol>
                                                "Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden."</li>
                                            <li>"7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe."</li>
                                            "Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet."
                                        </ul>
                                    </p>
                                </div>
                            </td>
                            <td class="px-1 text-right">
                                <input
                                    type="text"
                                    class="px-1 border-2 border-stone-400 rounded-lg text-right"
                                    value=move || v1sa.get().unwrap_or(0.0)
                                    on:change=change_v1_sonstige_auslagen
                                    prop:value=move || if v1a.get().unwrap_or(false) { format_euro(v1sa.get().unwrap_or(0.0)) } else { "0,00".to_string() }
                                />
                            </td>
                            <td colspan="2"></td>
                        </tr>
                    </tbody>
                </table>
                <h4 class="text-l font-bold">
                    "Gerichtskostengesetz"
                </h4>
                <table class="table-auto">
                    <thead>
                        <tr class="text-left">
                            <th>
                            </th>
                            <th>
                                "Gebührentatbestand und Nummer"
                            </th>
                            <th class="px-1">
                                "Gebührensatz"
                            </th>
                            <th class="px-1">
                                "Wertgebühr"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5210"
                                    on:change=change_n5210
                                    prop:checked=move || n5210.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1 max-w-64">
                                <label for="n5210">"Verfahren im Allgemeinen, Nr. 5210"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,5"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(gkg_v1.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5211"
                                    on:change=change_n5211
                                    prop:checked=move || n5211.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1 max-w-64">
                                <label for="n5211">"Ermäßigte Gebühr, Nr. 5211"</label>
                                <button popovertarget="ermaessigung5211" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung5211" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Gebühr Nr. 5210"</h4>
                                    <p>{ popover::ERMAESSIGUNG5211 }</p>
                                </div>  
                            </td>
                            <td class="px-1 text-right">
                                "0,5"
                            </td>
                            <td></td>
                        </tr>
                    </tbody>
                </table>
            </p>
            <p class=move || if v2.get().unwrap_or(false) { "visible" } else { "hidden" }>
                <h3 class="text-xl font-medium">
                    "2. Instanz"
                </h3>
                <h4 class="text-l font-medium">
                    "Rechtsanwaltsvergütungsgesetz"
                </h4>
                <table class="table-auto">
                    <thead>
                        <tr class="text-left">
                            <th>
                            </th>
                            <th>
                                "Gebührentatbestand und Nummer"
                            </th>
                            <th class="px-1">
                                "Gebührensatz"
                            </th>
                            <th class="px-1">
                                "Wertgebühr (§ 13 RVG)"
                            </th>
                            <th class="px-1">
                                "Wertgebühr (§ 49 RVG)"
                            </th>
                            <th class="pl-1">
                                "Differenz"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3200v"
                                    on:change=change_n3200v
                                    prop:checked=move || n3200v.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3200">"Verfahrensgebühr, Nr. 3200"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,6"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb13_v2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb49_v2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb13_v2.get() - verfgeb49_v2.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3201v"
                                    on:change=change_n3201v
                                    prop:checked=move || n3201v.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3201v">"Ermäßigte Verfahrensgebühr, Nr. 3201"</label>
                                <button popovertarget="ermaessigung3201v" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung3201v" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Verfahrensgebühr Nr. 3200"</h4>
                                    <p>{ popover::ERMAESSIGUNG3201 }</p>
                                </div>
                            </td>
                            <td class="px-1 text-right">
                                "1,1"
                            </td>
                            <td></td>
                            <td></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3202v"
                                    on:change=change_n3202v
                                    prop:checked=move || n3202v.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3202v">"Terminsgebühr, Nr. 3202"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,2"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb13_v2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb49_v2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb13_v2.get() - tgeb49_v2.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="v2_pauschale"
                                    on:change=change_v2_pauschale
                                    prop:checked=move || v2p.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="v2_pauschale">"Auslagenpauschale, Nr. 7002"</label>
                            </td>
                            <td></td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale13_v2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_v2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_v2.get() - pauschale13_v2.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="v2_auslagen"
                                    on:change=change_v2_auslagen
                                    prop:checked=move || v2a.get().unwrap_or(false)
                                />
                            </td>
                            <td colspan="2" class="px-1">
                                <label for="v2_auslagen">"Sonstige Auslagen"</label>
                                <button popovertarget="v2auslagen" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                                <div id="v2auslagen" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Sonstige Auslagen"</h4>
                                    <p>"Zum Beispiel:"
                                        <ul>
                                            <li>"7000 Pauschale für die Herstellung und Überlassung von Dokumenten:"
                                                <ul>
                                                    <li>"für Kopien und Ausdrucke"</li>        
                                                    <li>"für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR"</li>
                                                    <li>"für jede weitere Seite 0,15 EUR"</li>
                                                    <li>"für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR"</li>
                                                    <li>"für jede weitere Seite in Farbe 0,30 EUR"</li>
                                                </ul>
                                            </li>
                                            <li>"7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR."</li>
                                            <li>"7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe."</li>
                                            <li>"7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise"
                                                <ol>
                                                    <li>"von nicht mehr als 4 Stunden 30,00 EUR"</li>
                                                    <li>"von mehr als 4 bis 8 Stunden 50,00 EUR"</li>
                                                    <li>"von mehr als 8 Stunden 80,00 EUR"</li>
                                                </ol>
                                                "Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden."</li>
                                            <li>"7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe."</li>
                                            "Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet."
                                        </ul>
                                    </p>
                                </div>
                            </td>
                            <td class="px-1 text-right">
                                <input
                                    type="text"
                                    class="px-1 border-2 border-stone-400 rounded-lg text-right"
                                    value=move || v2sa.get().unwrap_or(0.0)
                                    on:change=change_v2_sonstige_auslagen
                                    prop:value=move || if v2a.get().unwrap_or(false) { format_euro(v2sa.get().unwrap_or(0.0)) } else { "0,00".to_string() }
                                />
                            </td>
                            <td colspan="2"></td>
                        </tr>
                    </tbody>
                </table>
                <h4 class="text-l font-medium">
                    "Gerichtskostengesetz"
                </h4>
                <table class="table-auto">
                    <thead>
                        <tr class="text-left">
                            <th>
                            </th>
                            <th>
                                "Gebührentatbestand und Nummer"
                            </th>
                            <th class="px-1">
                                "Gebührensatz"
                            </th>
                            <th class="px-1">
                                "Wertgebühr"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5240"
                                    on:change=change_n5240
                                    prop:checked=move || n5240.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1 max-w-128">
                                <label for="n5240">"Verfahren im Allgemeinen, Nr. 5240"</label>
                            </td>
                            <td class="px-1 text-right">
                                "2,0"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(gkg_v2.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5241"
                                    on:change=change_n5241
                                    prop:checked=move || n5241.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1 max-w-128">
                                <label for="n5241">"Ermäßigte Gebühr, Nr. 5241"</label>
                                <button popovertarget="ermaessigung5241" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung5241" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Gebühr Nr. 5240"</h4>
                                    <p>{ popover::ERMAESSIGUNG5241 }</p>
                                </div>  
                            </td>
                            <td class="px-1 text-right">
                                "0,5"
                            </td>
                            <td></td>
                        </tr>
                    </tbody>
                </table>
            </p>
            <h3 class="text-xl font-medium">
                "Summen"
            </h3>
            <p class="grid grid-cols-3 font-semibold">
                <div>
                    "Rechtsanwaltsvergütungsgesetz (§ 13 RVG)"
                </div>
                <div class="text-right">
                   { move || format_euro(summe_rvg13_v.get()) }
                </div>
                <div></div>
                <div>
                    "Rechtsanwaltsvergütungsgesetz (§ 49 RVG)"
                </div>
                <div class="text-right">
                   { move || format_euro(summe_rvg49_v.get()) }
                </div>
                <div></div>
                <div>
                    "Gerichtskostengesetz"
                </div>
                <div class="text-right">
                   { move || format_euro(summe_gkg_v.get()) }
                </div>
                <div></div>        
            </p>
        </div>
    }
}