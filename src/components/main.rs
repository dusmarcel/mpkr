use leptos::prelude::*;

use crate::utils::format_euro;
use crate::fees::pauschale;
use crate::popover;

// Hauptsacheverfahren

#[component]
pub fn Main(
    v: Memo<Option<u32>>,
    a: Memo<Option<bool>>,
    h1: Memo<Option<bool>>,
    set_h1: SignalSetter<Option<bool>>,
    h2: Memo<Option<bool>>,
    set_h2: SignalSetter<Option<bool>>,
    h3: Memo<Option<bool>>,
    set_h3: SignalSetter<Option<bool>>,
    n3100: Memo<Option<bool>>,
    set_n3100: SignalSetter<Option<bool>>,
    n3101: Memo<Option<bool>>,
    set_n3101: SignalSetter<Option<bool>>,
    verfgeb13_h1: Memo<f64>,
    verfgeb49_h1: Memo<f64>,
    anr: Memo<Option<bool>>,
    set_anr: SignalSetter<Option<bool>>,
    anrechnung13: Memo<f64>,
    anrechnung49: Memo<f64>,
    n3104: Memo<Option<bool>>,
    set_n3104: SignalSetter<Option<bool>>,
    tgeb13_h1: Memo<f64>,
    tgeb49_h1: Memo<f64>,
    h1p: Memo<Option<bool>>,
    set_h1p: SignalSetter<Option<bool>>,
    pauschale13_h1: Memo<f64>,
    pauschale49_h1: Memo<f64>,
    h1a: Memo<Option<bool>>,
    set_h1a: SignalSetter<Option<bool>>,
    h1sa: Memo<Option<f64>>,
    set_h1sa: SignalSetter<Option<f64>>,
    n5110: Memo<Option<bool>>,
    set_n5110: SignalSetter<Option<bool>>,
    n5111: Memo<Option<bool>>,
    set_n5111: SignalSetter<Option<bool>>,
    gkg_h1: Memo<f64>,
    n3200: Memo<Option<bool>>,
    set_n3200: SignalSetter<Option<bool>>,
    n3201: Memo<Option<bool>>,
    set_n3201: SignalSetter<Option<bool>>,
    verfgeb13_h2: Memo<f64>,
    verfgeb49_h2: Memo<f64>
) -> impl IntoView {
    let change_h1 = move |ev| set_h1.set(Some(event_target_checked(&ev)));
    let change_h2 = move |ev| set_h2.set(Some(event_target_checked(&ev)));
    let change_h3 = move |ev| set_h3.set(Some(event_target_checked(&ev)));
    let change_n3100 = move |ev| set_n3100.set(Some(event_target_checked(&ev)));
    let change_n3101 = move |ev| set_n3101.set(Some(event_target_checked(&ev)));
    let change_anrechnung = move |ev| set_anr.set(Some(event_target_checked(&ev)));
    let change_n3104 = move |ev| set_n3104.set(Some(event_target_checked(&ev)));
    let change_h1_pauschale = move |ev| set_h1p.set(Some(event_target_checked(&ev)));
    let change_h1_auslagen = move |ev| set_h1a.set(Some(event_target_checked(&ev)));
    let change_h1_sonstige_auslagen = move |ev| {
        set_h1sa.set(Some(event_target_value(&ev).parse::<f64>().unwrap_or(0.0)));
        if event_target_value(&ev).parse::<f64>().unwrap_or(0.0) != 0.0 {
            set_h1a.set(Some(true));
        } else {
            set_h1a.set(Some(false));
        }
    };
    let change_n5110 = move |ev| set_n5110.set(Some(event_target_checked(&ev)));
    let change_n5111 = move |ev| set_n5111.set(Some(event_target_checked(&ev)));
    let change_n3200 = move |ev| set_n3200.set(Some(event_target_checked(&ev)));
    let change_n3201 = move |ev| set_n3201.set(Some(event_target_checked(&ev)));

    view! {
        <div class=move || if v.get().unwrap_or(0) != 1 { // Container einblenden, wenn nicht "nur vorläufiger Rechtsschutz" ausgewählt ist 
                "container visible max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300"
            } else {
                "hidden"
            }
        >
            <h2 class="pt-4 text-2xl font-medium">
                "Hauptsacheverfahren"
            </h2>
            <p>
                <input
                    type="checkbox"
                    id="h1"
                    on:change=change_h1
                    prop:checked=move || h1.get().unwrap_or(true)
                />
                <label for="h1" class="mx-1">1. Instanz</label>
                <input
                    type="checkbox"
                    id="h2"
                    on:change=change_h2
                    prop:checked=move || h2.get().unwrap_or(false)
                />
                <label for="h2" class="mx-1">2. Instanz</label>
                <input
                    type="checkbox"
                    id="h3"
                    on:change=change_h3
                    prop:checked=move || h3.get().unwrap_or(false)
                />
                <label for="h3" class="mx-1">3. Instanz</label>
            </p>
            <p class=move || if h1.get().unwrap_or(true) { "visible" } else { "hidden" }>
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
                                    id="n3100"
                                    on:change=change_n3100
                                    prop:checked=move || n3100.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3100">"Verfahrensgebühr, Nr. 3100"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,3"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb13_h1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb49_h1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb13_h1.get() - verfgeb49_h1.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3101"
                                    on:change=change_n3101
                                    prop:checked=move || n3101.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3101">"Ermäßigte Verfahrensgebühr, Nr. 3101"</label>
                                <button popovertarget="ermaessigung3101" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung3101" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
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
                                    id="anr"
                                    on:change=change_anrechnung
                                    prop:checked=move || anr.get().unwrap_or(a.get().unwrap_or(false))
                                />
                            </td>
                            <td colspan="2" class="px-1">
                                <label for="anr">"Anrechnung der Geschäfts- auf die Verfahrensgebühr"</label>
                                <button popovertarget="anrechnung" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="anrechnung" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Vorbemerkung 3 Abs. 4 VV RVG"</h4>
                                    <p>{ popover::ANRECHNUNG }</p>
                                </div>                                
                            </td>
                            <td class="px-1 text-right">
                                <span class="mr-1">"-"</span>
                                { move || format_euro(anrechnung13.get()) }
                            </td>
                            <td class="px-1 text-right">
                                <span class="mr-1">"-"</span>
                                { move || format_euro(anrechnung49.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(anrechnung13.get() - anrechnung49.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3104"
                                    on:change=change_n3104
                                    prop:checked=move || n3104.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3104">"Terminsgebühr, Nr. 3104"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,2"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb13_h1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb49_h1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb13_h1.get() - tgeb49_h1.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="h1_pauschale"
                                    on:change=change_h1_pauschale
                                    prop:checked=move || h1p.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="h1_pauschale">"Auslagenpauschale, Nr. 7002"</label>
                            </td>
                            <td></td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale13_h1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_h1.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_h1.get() - pauschale13_h1.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="h1_auslagen"
                                    on:change=change_h1_auslagen
                                    prop:checked=move || h1a.get().unwrap_or(false)
                                />
                            </td>
                            <td colspan="2" class="px-1">
                                <label for="h1_auslagen">"Sonstige Auslagen"</label>
                                <button popovertarget="h1auslagen" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                                <div id="h1auslagen" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
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
                                    value=move || h1sa.get().unwrap_or(0.0)
                                    on:change=change_h1_sonstige_auslagen
                                    prop:value=move || if h1a.get().unwrap_or(false) { format_euro(h1sa.get().unwrap_or(0.0)) } else { "0,00".to_string() }
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
                                    id="n5110"
                                    on:change=change_n5110
                                    prop:checked=move || n5110.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1 max-w-64">
                                <label for="n5110">"Verfahren im Allgemeinen, Nr. 5110"</label>
                            </td>
                            <td class="px-1 text-right">
                                "3,0"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(gkg_h1.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5111"
                                    on:change=change_n5111
                                    prop:checked=move || n5111.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1 max-w-64">
                                <label for="n5111">"Ermäßigte Gebühr, Nr. 5111"</label>
                                <button popovertarget="ermaessigung5111" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung5111" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Gebühr Nr. 5110"</h4>
                                    <p>{ popover::ERMAESSIGUNG5111 }</p>
                                </div>  
                            </td>
                            <td class="px-1 text-right">
                                "1,0"
                            </td>
                            <td></td>
                        </tr>
                    </tbody>
                </table>
            </p>
            <p class=move || if h2.get().unwrap_or(false) { "visible" } else { "hidden" }>
                <h3 class="text-xl font-medium">
                    "2. Instanz"
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
                                    id="n3200"
                                    on:change=change_n3200
                                    prop:checked=move || n3200.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3200">"Verfahrensgebühr, Nr. 3200"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,6"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb13_h2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb49_h2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb13_h2.get() - verfgeb49_h2.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3201"
                                    on:change=change_n3201
                                    prop:checked=move || n3201.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3201">"Ermäßigte Verfahrensgebühr, Nr. 3201"</label>
                                <button popovertarget="ermaessigung3201" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung3201" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
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
                                    id="n3202"
                                    on:change=change_n3202
                                    prop:checked=move || n3202.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3202">"Terminsgebühr, Nr. 3202"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,2"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb13_h2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb49_h2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb13_h2.get() - tgeb49_h2.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="h2_pauschale"
                                    on:change=change_h2_pauschale
                                    prop:checked=move || h2p.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="h2_pauschale">"Auslagenpauschale, Nr. 7002"</label>
                            </td>
                            <td></td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale13_h2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_h2.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_h2.get() - pauschale13_h2.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="h2_auslagen"
                                    on:change=change_h2_auslagen
                                    prop:checked=move || h2a.get().unwrap_or(false)
                                />
                            </td>
                            <td colspan="2" class="px-1">
                                <label for="h2_auslagen">"Sonstige Auslagen"</label>
                                <button popovertarget="h2auslagen" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                                <div id="h2auslagen" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
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
                                    value=move || h2sa.get().unwrap_or(0.0)
                                    on:change=change_h2_sonstige_auslagen
                                    prop:value=move || if h2a.get().unwrap_or(false) { format_euro(h2sa.get().unwrap_or(0.0)) } else { "0,00".to_string() }
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
                                    id="n5122"
                                    on:change=change_n5122
                                    prop:checked=move || n5122.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1 max-w-128">
                                <label for="n5122">"Verfahren im Allgemeinen, Nr. 5122"</label>
                            </td>
                            <td class="px-1 text-right">
                                "4,0"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(gkg_h2.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5120"
                                    on:change=change_n5120
                                    prop:checked=move || n5120.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1 max-w-128">
                                <label for="n5120">"Verfahren über die Zulassung der Berufung, soweit der Antrag abgeleht wird, Nr. 5120"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,0"
                            </td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5121"
                                    on:change=change_n5121
                                    prop:checked=move || n5121.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1 max-w-128">
                                <label for="n5121">"Verfahren über die Zulassung der Berufung, soweit der Antrag zurückgenommen oder das Verfahren durch anderweitige Erledigung beendet wird, Nr. 5121"</label>
                            </td>
                            <td class="px-1 text-right">
                                "0,5"
                            </td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5123"
                                    on:change=change_n5123
                                    prop:checked=move || n5123.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1 max-w-128">
                                <label for="n5123">"Ermäßigte Gebühr, Nr. 5123"</label>
                                <button popovertarget="ermaessigung5123" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung5123" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Gebühr Nr. 5122"</h4>
                                    <p>{ popover::ERMAESSIGUNG5123 }</p>
                                </div>  
                            </td>
                            <td class="px-1 text-right">
                                "1,0"
                            </td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5124"
                                    on:change=change_n5124
                                    prop:checked=move || n5124.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1 max-w-128">
                                <label for="n5124">"Weitere Beendigung des Verfahrens, Nr. 5124"</label>
                                <button popovertarget="ermaessigung5124" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung5124" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Gebühr Nr. 5122"</h4>
                                    <p>{ popover::ERMAESSIGUNG5124 }</p>
                                </div>  
                            </td>
                            <td class="px-1 text-right">
                                "2,0"
                            </td>
                            <td></td>
                        </tr>
                    </tbody>
                </table>
            </p>
            <p class=move || if h3.get().unwrap_or(false) { "visible" } else { "hidden" }>
                <h3 class="text-xl font-medium">
                    "3. Instanz"
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
                                    id="n3206"
                                    on:change=change_n3206
                                    prop:checked=move || n3206.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3206">"Verfahrensgebühr, Nr. 3206"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,6"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb13_h3.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb49_h3.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb13_h3.get() - verfgeb49_h3.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n3207"
                                    on:change=change_n3207
                                    prop:checked=move || n3207.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3207">"Ermäßigte Verfahrensgebühr, Nr. 3207"</label>
                                <button popovertarget="ermaessigung3207" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung3207" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Verfahrensgebühr Nr. 3206"</h4>
                                    <p>{ popover::ERMAESSIGUNG3207 }</p>
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
                                    id="n3210"
                                    on:change=change_n3210
                                    prop:checked=move || n3210.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="n3210">"Terminsgebühr, Nr. 3210"</label>
                            </td>
                            <td class="px-1 text-right">
                                "1,5"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb13_h3.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb49_h3.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb13_h3.get() - tgeb49_h3.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="h3_pauschale"
                                    on:change=change_h3_pauschale
                                    prop:checked=move || h3p.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">
                                <label for="h3_pauschale">"Auslagenpauschale, Nr. 7002"</label>
                            </td>
                            <td></td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale13_h3.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_h3.get()) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_h3.get() - pauschale13_h3.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="h3_auslagen"
                                    on:change=change_h3_auslagen
                                    prop:checked=move || h3a.get().unwrap_or(false)
                                />
                            </td>
                            <td colspan="2" class="px-1">
                                <label for="h3_auslagen">"Sonstige Auslagen"</label>
                                <button popovertarget="h3auslagen" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                                <div id="h3auslagen" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
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
                                    value=move || h3sa.get().unwrap_or(0.0)
                                    on:change=change_h3_sonstige_auslagen
                                    prop:value=move || if h3a.get().unwrap_or(false) { format_euro(h3sa.get().unwrap_or(0.0)) } else { "0,00".to_string() }
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
                                    id="n5130"
                                    on:change=change_n5130
                                    prop:checked=move || n5130.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1 max-w-128">
                                <label for="n5130">"Verfahren im Allgemeinen, Nr. 5130"</label>
                            </td>
                            <td class="px-1 text-right">
                                "5,0"
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(gkg_h3.get()) }
                            </td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5131"
                                    on:change=change_n5131
                                    prop:checked=move || n5131.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1 max-w-128">
                                <label for="n5131">"Ermäßigte Gebühr, Nr. 5131"</label>
                                <button popovertarget="ermaessigung5131" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung5131" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Gebühr Nr. 5130"</h4>
                                    <p>{ popover::ERMAESSIGUNG5131 }</p>
                                </div>  
                            </td>
                            <td class="px-1 text-right">
                                "1,0"
                            </td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="pr-1">
                                <input
                                    type="checkbox"
                                    id="n5132"
                                    on:change=change_n5132
                                    prop:checked=move || n5132.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1 max-w-128">
                                <label for="n5132">"Weitere Beendigung des Verfahrens, Nr. 5132"</label>
                                <button popovertarget="ermaessigung5132" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="ermaessigung5132" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Ermäßigung der Gebühr Nr. 5130"</h4>
                                    <p>{ popover::ERMAESSIGUNG5132 }</p>
                                </div>  
                            </td>
                            <td class="px-1 text-right">
                                "3,0"
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
                   { move || format_euro(summe_rvg13_h.get()) }
                </div>
                <div></div>
                <div>
                    "Rechtsanwaltsvergütungsgesetz (§ 49 RVG)"
                </div>
                <div class="text-right">
                   { move || format_euro(summe_rvg49_h.get()) }
                </div>
                <div></div>
                <div>
                    "Gerichtskostengesetz"
                </div>
                <div class="text-right">
                   { move || format_euro(summe_gkg_h.get()) }
                </div>
                <div></div>        
            </p>
        </div>
    }
}