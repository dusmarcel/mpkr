use leptos::prelude::*;

use crate::utils::format_euro;
use crate::fees::pauschale;
use crate::popover;

// Aussergerichtliche Vertretung

#[component]
pub fn Extrajudicial(
    v: Memo<Option<u32>>,
    a: Memo<Option<bool>>,
    set_a: SignalSetter<Option<bool>>,
    g: Memo<Option<bool>>,
    set_g: SignalSetter<Option<bool>>,
    gs: Memo<Option<f64>>,
    set_gs: SignalSetter<Option<f64>>,
    n2300: Memo<f64>,
    ap: Memo<Option<bool>>,
    set_ap: SignalSetter<Option<bool>>,
    aa: Memo<Option<bool>>,
    set_aa: SignalSetter<Option<bool>>,
    asa: Memo<Option<f64>>,
    set_asa: SignalSetter<Option<f64>>,
    summe_aussergerichtlich: Memo<f64>
) -> impl IntoView {
    let change_aussergerichtlich = move |ev| {
        if v.get().unwrap_or(0) != 1 {
            set_a.set(Some(event_target_checked(&ev)));
        } else {
            set_a.set(Some(false));
        }
    };
    let change_geschaeftsgebuehr = move |ev| set_g.set(Some(event_target_checked(&ev)));
    let change_gebuehrensatz = move |ev| set_gs.set(Some(event_target_value(&ev).parse::<f64>().unwrap_or(1.3)));
    let change_a_pauschale = move |ev| set_ap.set(Some(event_target_checked(&ev)));
    let change_a_auslagen = move |ev| set_aa.set(Some(event_target_checked(&ev)));
    let change_aussergerichtlich_sonstige_auslagen = move |ev| {
        set_asa.set(Some(event_target_value(&ev).parse::<f64>().unwrap_or(0.0)));
        if event_target_value(&ev).parse::<f64>().unwrap_or(0.0) != 0.0 {
            set_aa.set(Some(true));
        } else {
            set_aa.set(Some(false));
        }
    };

    view! {
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Außergerichtliche Vertretung"
            </h2>
            <p>
                <input
                    type="checkbox"
                    id="aussergerichtlich"
                    on:change=change_aussergerichtlich
                    prop:checked=move || a.get().unwrap_or(false)
                />
                <label for="aussergerichtlich" class="ml-1">"Außergerichtliche Vertretung"</label>
                <button popovertarget="aussergerichtliche-vertretung" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                <div id="aussergerichtliche-vertretung" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                    <h4 class="text-xl font-medium">"Außergerichtliche Vertretung"</h4>
                    <p>{ popover::AUSSERGERICHTLICH }</p>
                </div>            
            </p>
            // Abschnitt für die Berechnung der Gebühren der außergerichtlichen Vertretung.
            // Er soll nur angezeigt werden, wenn die Box für außergerichtliche Vertretung (a)
            // und nicht nur Hauptsacheverfahren (v != 1) ausgewählt wurde
            <p class=move || if a.get().unwrap_or(false) && v.get().unwrap_or(0) != 1 { "visible" } else { "hidden" }>
                <table>
                    <tbody>
                        <tr>
                            <td class="px-1">
                                <input
                                    type="checkbox"
                                    id="geschaeftsgebuehr"
                                    on:change=change_geschaeftsgebuehr
                                    prop:checked=move || g.get().unwrap_or(true)
                                />
                            </td>
                            <td class="px-1">"Geschäftsgebühr, Nr. 2300 VV RVG"</td>
                            <td class="px-1">
                                Gebührensatz
                            </td>
                            <td class="px-1">
                                <input
                                    type="number"
                                    class="p-1 border-2 border-stone-400 rounded-lg"
                                    step="0.1"
                                    min="0.5"
                                    max="2.5"
                                    value="1.3"
                                    on:change=change_gebuehrensatz
                                    prop:value=move || gs.get().unwrap_or(1.3)
                                />
                                <button popovertarget="gebuehrensatz" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                                <div id="gebuehrensatz" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Gebührensatz für die Geschäftsgebühr"</h4>
                                    <p>{ popover::GEBUEHRENSATZ }</p>
                                </div>
                            </td>
                            <td class="px-1 text-right">
                                { move || if g.get().unwrap_or(true) { format_euro(n2300.get()) } else { "0,00".to_string() } }
                            </td>
                        </tr>
                        <tr>
                            <td class="px-1">
                                <input
                                    type="checkbox"
                                    id="a_pauschale"
                                    on:change=change_a_pauschale
                                    prop:checked=move || ap.get().unwrap_or(true)
                                />
                            </td>                            
                            <td class="px-1">"Auslagenpauschale, Nr. 7002 VV RVG"</td>
                            <td></td>
                            <td></td>
                            <td class="px-1 text-right">
                                { move || if ap.get().unwrap_or(true) { format_euro(pauschale(n2300.get())) } else { "0,00".to_string() } }
                            </td>
                        </tr>
                        <tr>
                            <td class="px-1">
                                <input
                                    type="checkbox"
                                    id="a_auslagen"
                                    on:change=change_a_auslagen
                                    prop:checked=move || aa.get().unwrap_or(false)
                                />
                            </td>
                            <td class="px-1">
                                <span>"Sonstige Auslagen, z. B. Nr. 7000, 7003 ff. VV RVG"</span>
                                <button popovertarget="aauslagen" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                                <div id="aauslagen" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
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
                            <td></td>
                            <td></td>
                            <td class="px-1">
                                <input
                                    type="text"
                                    class="px-1 border-2 border-stone-400 rounded-lg text-right"
                                    value=move || asa.get().unwrap_or(0.0)
                                    on:change=change_aussergerichtlich_sonstige_auslagen
                                    prop:value=move || if aa.get().unwrap_or(false) { format_euro(asa.get().unwrap_or(0.0)) } else { "0,00".to_string() }
                                />
                            </td>
                        </tr>                        
                        <tr class="font-semibold">
                            <td></td>
                            <td class="px-1">Summe</td>
                            <td></td>
                            <td></td>
                            <td class="px-1 text-right">
                                { move || format_euro(summe_aussergerichtlich.get()) }
                            </td>
                        </tr>  
                    </tbody>
                </table>
            </p>
        </div>
    }
}