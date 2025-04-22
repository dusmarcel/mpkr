use fees::pauschale;
use leptos::prelude::*;
use leptos_router::{hooks::query_signal_with_options, location::State, NavigateOptions};

mod popover;
mod fees;
mod utils;
use utils::format_euro;

#[component]
pub fn MPKR() -> impl IntoView {
    // Allgemeine Einstellungen zum Streitwert
    let (v, set_v) = query_signal_with_options::<u32>(
        "v", 
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_verfahren = move |ev| set_v.set(Some(event_target_value(&ev).parse::<u32>().unwrap_or(0)));

    let (t_changed, set_t_changed) = signal(false);
    let (t, set_t) = query_signal_with_options::<u32>(
        "t",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_thema = move |ev| {
        set_t.set(Some(event_target_value(&ev).parse::<u32>().unwrap_or(4)));
        set_t_changed.set(true);
    };

    let (p_changed, set_p_changed) = signal(false);
    let (p, set_p) = query_signal_with_options::<u32>(
        "p",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_personen = move |ev| {
        set_p.set(Some(event_target_value(&ev).parse::<u32>().unwrap_or(1)));
        set_p_changed.set(true);
    };
    
    let (s, set_s) = query_signal_with_options::<f64>(
        "s",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_streitwert = move |ev| set_s.set(Some(event_target_value(&ev).parse::<f64>().unwrap_or(fees::AUFFANGSTREITWERT)));

    let (sv, set_sv) = query_signal_with_options::<f64>(
        "sv",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_streitwert_vorl = move |ev| set_sv.set(Some(event_target_value(&ev).parse::<f64>().unwrap_or(fees::AUFFANGSTREITWERT / 2.0)));

    Effect::new(move |_| {
        if t_changed.get() || p_changed.get() {
            set_s.set(Some(fees::default_streitwert(t.get().unwrap_or(4), p.get().unwrap_or(1))));
            set_sv.set(Some(fees::default_streitwert(t.get().unwrap_or(4), p.get().unwrap_or(1)) / 2.0));
        }
    });

    // Aussergerichtliche Vertretung
    let (a, set_a) = query_signal_with_options::<bool>(
        "a",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_aussergerichtlich = move |ev| {
        if v.get().unwrap_or(0) != 1 {
            set_a.set(Some(event_target_checked(&ev)));
        } else {
            set_a.set(Some(false));
        }
    };

    let (g, set_g) = query_signal_with_options::<bool>(
        "g",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_geschaeftsgebuehr = move |ev| set_g.set(Some(event_target_checked(&ev)));

    let (gs, set_gs) = query_signal_with_options::<f64>(
        "gs",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_gebuehrensatz = move |ev| set_gs.set(Some(event_target_value(&ev).parse::<f64>().unwrap_or(1.3))); 

    let n2300 = Memo::new( move |_| {
        gs.get().unwrap_or(1.3) * fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
    });

    let (ap, set_ap) = query_signal_with_options::<bool>(
        "ap",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_a_pauschale = move |ev| set_ap.set(Some(event_target_checked(&ev)));

    let (aa, set_aa) = query_signal_with_options::<bool>(
        "aa",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_a_auslagen = move |ev| set_aa.set(Some(event_target_checked(&ev)));

    let (asa, set_asa) = query_signal_with_options::<f64>(
        "asa",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_aussergerichtlich_sonstige_auslagen = move |ev| set_asa.set(Some(event_target_value(&ev).parse::<f64>().unwrap_or(0.0))); 

    let summe_aussergerichtlich = Memo::new(move |_| {
        let mut summe = 0.0;
        if g.get().unwrap_or(true) { summe += n2300.get() };
        if ap.get().unwrap_or(true) { summe += pauschale(n2300.get()) };
        if aa.get().unwrap_or(false) { summe += asa.get().unwrap_or(0.0) };
        summe
    });

    // Summen
    let summe_rvg13_netto = Memo::new( move |_| {
        let mut summe = 0.0;
        if a.get().unwrap_or(false) { summe += summe_aussergerichtlich.get() }
        summe
    });

    let summe_rvg49_netto = Memo::new( move |_| {
        let mut summe = 0.0;
        if a.get().unwrap_or(false) { summe += summe_aussergerichtlich.get() }
        summe
    });

    let (u, set_u) = query_signal_with_options::<u32>(
        "u",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_umsatzsteuer = move |ev| {
        set_u.set(Some(event_target_value(&ev).parse::<u32>().unwrap_or(19)));
    };

    view! {
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h1 class="pt-4 text-3xl font-medium">
                "Migrationsrechtlicher Prozesskostenrechner (Stand 2025)"
            </h1>
            <p>
                "Erstellt von "<a class="text-blue-600 hover:underline hover:text-violet-600" href="https://aufentha.lt">"Marcel Keienborg"</a>". Bitte beachte unbedingt auch die
                Hinweise unten auf dieser Seite."
            </p>
        </div>
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Streitwerte"
            </h2>
            <p>
                "Grundlage für die Berechnung der Gebühren ist der sogenannte Streitwert. Wir müssen also zuerst die Streitwerte für deine Angelegenheit ermitteln. Plural, weil gerade in gerichtlichen Verfahren häufig zusätzlich zur Klage noch ein Antrag auf Gewährung vorläufigen Rechtsschutzes erforderlich ist. Manchmal wird auch nur ein Antrag auf vorläufigen Rechtsschutz gestellt. Deswegen kann man hier verschiedene Optionen wählen. Außerdem hängt die konkrete Höhe des Streitwerts auch von der Anzahl der Personen ab, die in einem Verfahren zusammengefasst werden."
            </p>
            <p>
                <label for="verfahren">"Wähle aus, ob die Gebühren nur für ein Hauptsacheverfahren, nur für dein Verfahren
                zum vorläufigen Rechtsschutz, oder für beides berechnet werden sollen."</label>
            </p>
            <p>
                <select class="p-1 border-2 border-stone-400 rounded-lg" aria-label="Auswahl der Verfahrensart" id="verfahren" on:change=change_verfahren>
                    <option value="0" selected=move || v.get().unwrap_or(0) == 0>"Nur Hauptsacheverfahren"</option>
                    <option value="1" selected=move || v.get().unwrap_or(0) == 1>"Nur Verfahren zum vorläufigen Rechtsschutz"</option>
                    <option value="2" selected=move || v.get().unwrap_or(0) == 2>"Hauptsacheverfahren und Verfahren zum vorläufigen Rechtsschutz"</option>
                </select>
            </p>
            <p>
                <label for="thema">"Wähle ein Thema, dann versucht der Rechner, dir die passenden Streitwerte vorzuschlagen.
                Du kannst aber auch manuell selbst Streitwerte angeben."</label>
            </p>
            <p>
                <select class="p-1 border-2 border-stone-400 rounded-lg" aria-label="Auswahl des Themas" id="thema" on:change=change_thema>
                    <option value="0" selected=move || t.get().unwrap_or(4) == 0>"Asylrecht: Zulässigkeit (z.B. Dublin, Drittstaatenfall, Folgeantrag)"</option>
                    <option value="1" selected=move || t.get().unwrap_or(4) == 1>"Asylrecht: Anerkennungsverfahren"</option>
                    <option value="2" selected=move || t.get().unwrap_or(4) == 2>"Asylrecht: Widerruf/Rücknahme"</option>
                    <option value="3" selected=move || t.get().unwrap_or(4) == 3>"Asylrecht: Untätigkeitsklage"</option>
                    <option value="4" selected=move || t.get().unwrap_or(4) == 4>"Aufenthaltsrecht: Aufenthaltstitel inkl. Untätigkeitsklage"</option>
                    <option value="5" selected=move || t.get().unwrap_or(4) == 5>"Ausweisung"</option>
                    <option value="6" selected=move || t.get().unwrap_or(4) == 6>"Pass/Passersatz"</option>
                    <option value="7" selected=move || t.get().unwrap_or(4) == 7>"Aufenthaltsrecht: Duldung und Abschiebung inkl. Ausbildungs-/Beschäftigungsduldung,
                        Untätigkeitsklage"</option>
                    <option value="8" selected=move || t.get().unwrap_or(4) == 8>"Einbürgerung und Feststellung der Staatsangehörigkeit"</option>
                </select>
            </p>
            <div>
                <table class="table-auto">
                    <thead>
                        <tr class="text-left">
                            <th class="px-1">
                                <label for="anzahl">"Anzahl Personen"</label>
                            </th>
                            <th class="px-1">
                                "Streitwerte"
                            </th>
                            <th class="px-1">
                                "Wertgebühr (§ 13 RVG)"
                            </th>
                            <th class="px-1">
                                "Wertgebühr (§ 49 RVG / PKH)"
                            </th>
                            <th class="px-1">
                                "Wertgebühr (GKG)"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td></td>
                            <td  class="px-1">
                                { move || match v.get().unwrap_or(0) {
                                    0 => "Hauptsache",
                                    1 => "vorläufiger Rechtsschutz",
                                    _ => "Hauptsache"
                                }}
                            </td>
                            <td></td>
                            <td></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="px-1">
                                <input type="number" min="1" value=move || p.get().unwrap_or(1) class="border-2 border-stone-400 rounded-lg px-1" on:change=change_personen />
                                <button popovertarget="zahl-der-personen" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="zahl-der-personen" popover class="open:border-2 open:border-stone-400 open:rounded-lg open:p-2 open:mt-60 open:mx-60">
                                    <h4 class="text-xl font-medium">Zahl der Personen</h4>
                                    <p>{ popover::PERSONS }</p>
                                </div>
                            </td>
                            <td class="px-1">
                                <input
                                    type="text"
                                    class="px-1 border-2 border-stone-400 rounded-lg text-right"
                                    value=move || s.get().unwrap_or(fees::default_streitwert(t.get().unwrap_or(4), p.get().unwrap_or(1)))
                                    on:change=change_streitwert
                                    prop:value=move || format_euro(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
                                />
                                <span class="ml-1">EUR</span>
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))) }
                                <span class="ml-1">EUR</span>
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(fees::rvg49_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))) }
                                <span class="ml-1">EUR</span>
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(fees::gkg_geb(t.get().unwrap_or(4), s.get().unwrap_or(fees::AUFFANGSTREITWERT))) }
                                <span class="ml-1">EUR</span>
                            </td>                     
                        </tr>
                        <tr class=move || if v.get().unwrap_or(0) == 2 { "visible" } else { "hidden" }>
                            <td></td>
                            <td class="px-1">
                                vorläufiger Rechtsschutz
                            </td>
                            <td></td>
                            <td></td>
                            <td></td> 
                        </tr>
                        <tr class=move || if v.get().unwrap_or(0) == 2 { "visible" } else { "hidden" }>
                            <td></td>
                            <td class="px-1">
                                <input
                                    type="text"
                                    class="px-1 border-2 border-stone-400 rounded-lg text-right"
                                    value=move || sv.get().unwrap_or(fees::default_streitwert(t.get().unwrap_or(4), p.get().unwrap_or(1)) / 2.0)
                                    on:change=change_streitwert_vorl
                                    prop:value=move || format_euro(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))
                                />
                                <span class="ml-1">EUR</span>                       
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(fees::rvg13_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))) }
                                <span class="ml-1">EUR</span>
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(fees::rvg49_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))) }
                                <span class="ml-1">EUR</span>
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(fees::gkg_geb(t.get().unwrap_or(4), sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))) }
                                <span class="ml-1">EUR</span>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
        // Aussergerichtliche Vertretung
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
                <label for="aussergerichtlich" class="ml-1">Außergerichtliche Vertretung</label>
                <button popovertarget="aussergerichtliche-vertretung" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                <div id="aussergerichtliche-vertretung" popover class="open:border-2 open:border-stone-400 open:rounded-lg open:p-2 open:mt-60 open:mx-60">
                    <h4 class="text-xl font-medium">Außergerichtliche Vertretung</h4>
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
                            <td class="px-1 font-semibold">"Geschäftsgebühr, Nr. 2300 VV RVG"</td>
                            <td class="px-1">
                                Gebührensatz
                                <button popovertarget="gebuehrensatz" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                                <div id="gebuehrensatz" popover class="open:border-2 open:border-stone-400 open:rounded-lg open:p-2 open:mt-60 open:mx-60">
                                    <h4 class="text-xl font-medium">Gebührensatz für die Geschäftsgebühr</h4>
                                    <p>{ popover::GEBUEHRENSATZ }</p>
                                </div>    
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
                            </td>
                            <td class="px-1 text-right">
                                { move || if g.get().unwrap_or(true) { format_euro(n2300.get()) } else { "0,00".to_string() } }
                                <span class="ml-1">EUR</span>
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
                            <td class="px-1 font-semibold">"Auslagenpauschale, Nr. 7002 VV RVG"</td>
                            <td></td>
                            <td></td>
                            <td class="px-1 text-right">
                                { move || if ap.get().unwrap_or(true) { format_euro(pauschale(n2300.get())) } else { "0,00".to_string() } }
                                <span class="ml-1">EUR</span>
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
                                <span class="font-semibold">"Sonstige Auslagen, z. B. Nr. 7000, 7003 ff. VV RVG"</span>
                                <button popovertarget="auslagen" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                                <div id="auslagen" popover class="open:border-2 open:border-stone-400 open:rounded-lg open:p-2 open:mt-60 open:mx-60">
                                    <h4 class="text-xl font-medium">Sonstige Auslagen</h4>
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
                                <span class="ml-1">EUR</span>
                            </td>
                        </tr>                        
                        <tr class="font-semibold italic">
                            <td></td>
                            <td class="px-1">Summe</td>
                            <td></td>
                            <td></td>
                            <td class="px-1 text-right">
                                { move || format_euro(summe_aussergerichtlich.get()) }
                                <span class="ml-1">EUR</span>
                            </td>
                        </tr>  
                    </tbody>
                </table>
            </p>
        </div>  
//           <div class="container border border-5 rounded p-4 m-4 collapse" id="hauptsache">
//             <h2>Hauptsacheverfahren</h2>
//             <p>
//               <input type="checkbox" id="instanz_h1">
//               <label for="instanz_h1">1. Instanz</label>
//               <input type="checkbox" id="instanz_h2">
//               <label for="instanz_h2">2. Instanz</label>
//               <input type="checkbox" id="instanz_h3">
//               <label for="instanz_h3">3. Instanz</label>
//             </p>
  
//             <!-- 1. Instanz des Hauptsacheverfahrens -->
  
//             <div class="collapse" id="div_instanz_h1">
//               <h3>1. Instanz</h3>
//               <h4>Rechtsanwaltsvergütungsgesetz</h4>
//               <p>
//               <div class="row fw-bold">
//                 <div class="col-1"></div>
//                 <div class="col-4">Gebührentatbestand und Nummer</div>
//                 <div class="col-2">Gebührensatz</div>
//                 <div class="col-2">Wertgebühr (§ 13 RVG)</div>
//                 <div class="col-3">Wertgebühr (§ 49 RVG / Prozesskostenhilfe)</div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h1_3100"></div>
//                 <div class="col-4"><label for="h1_3100">Verfahrensgebühr, Nr. 3100</label></div>
//                 <div class="col-2">1,3</div>
//                 <div class="col-2" id="h1_3100_13"></div>
//                 <div class="col-3" id="h1_3100_49"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h1_3101" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Verfahrensgebühr Nr. 3100"
//                     data-bs-content="Die Ermäßiung tritt insbesondere ein, wenn der Auftrag endigt, bevor die*der Rechtsanwält*in die Klage, den ein Verfahren einleitenden Antrag oder einen Schriftsatz, der Sachanträge, Sachvortrag, die Zurücknahme der Klage oder die Zurücknahme des Antrags enthält, eingereicht oder bevor sie*er einen gerichtlichen Termin wahrgenommen hat.">
//                 </div>
//                 <div class="col-4"><label for="h1_3101" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Verfahrensgebühr Nr. 3100"
//                     data-bs-content="Die Ermäßiung tritt insbesondere ein, wenn der Auftrag endigt, bevor die*der Rechtsanwält*in die Klage, den ein Verfahren einleitenden Antrag oder einen Schriftsatz, der Sachanträge, Sachvortrag, die Zurücknahme der Klage oder die Zurücknahme des Antrags enthält, eingereicht oder bevor sie*er einen gerichtlichen Termin wahrgenommen hat.">Ermäßigte
//                     Verfahrensgebühr, Nr. 3101</label></div>
//                 <div class="col-2" data-bs-toggle="popover" data-bs-trigger="hover"
//                   title="Ermäßigung der Verfahrensgebühr Nr. 3100"
//                   data-bs-content="Die Ermäßiung tritt insbesondere ein, wenn der Auftrag endigt, bevor die*der Rechtsanwält*in die Klage, den ein Verfahren einleitenden Antrag oder einen Schriftsatz, der Sachanträge, Sachvortrag, die Zurücknahme der Klage oder die Zurücknahme des Antrags enthält, eingereicht oder bevor sie*er einen gerichtlichen Termin wahrgenommen hat.">
//                   0,8</div>
//                 <div class="col-5"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h1_anrechnung" data-bs-toggle="popover"
//                     data-bs-trigger="hover" title="Vorbemerkung 3 Abs. 4 VV RVG"
//                     data-bs-content="Soweit wegen desselben Gegenstands eine Geschäftsgebühr nach Teil 2 entsteht, wird diese Gebühr zur Hälfte, bei Wertgebühren jedoch höchstens mit einem Gebührensatz von 0,75, auf die Verfahrensgebühr des gerichtlichen Verfahrens angerechnet. [...]">
//                 </div>
//                 <div class="col-6"><label for="h1_anrechnung" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Vorbemerkung 3 Abs. 4 VV RVG"
//                     data-bs-content="Soweit wegen desselben Gegenstands eine Geschäftsgebühr nach Teil 2 entsteht, wird diese Gebühr zur Hälfte, bei Wertgebühren jedoch höchstens mit einem Gebührensatz von 0,75, auf die Verfahrensgebühr des gerichtlichen Verfahrens angerechnet. [...]">Anrechnung
//                     der Geschäfts- auf die Verfahrensgebühr</label></div>
//                 <div class="col-2" id="h1_anrechnung13"></div>
//                 <div class="col-3" id="h1_anrechnung49"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h1_3104"></div>
//                 <div class="col-4"><label for="h1_3104">Terminsgebühr, Nr. 3104</label></div>
//                 <div class="col-2">1,2</div>
//                 <div class="col-2" id="h1_3104_13"></div>
//                 <div class="col-3" id="h1_3104_49"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h1_7002"></div>
//                 <div class="col-4"><label for="h1_7002">Auslagenpauschale, Nr. 7002</label></div>
//                 <div class="col-2"></div>
//                 <div class="col-2" id="h1_7002_13"></div>
//                 <div class="col-3" id="h1_7002_49"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h1_7000ua" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Sonstige Auslagen" data-bs-content="Zum Beispiel:
//   7000 Pauschale für die Herstellung und Überlassung von Dokumenten:
//   für Kopien und Ausdrucke
//   für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR
//   für jede weitere Seite 0,15 EUR
//   für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR
//   für jede weitere Seite in Farbe 0,30 EUR
//   7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR.
//   Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet.
//   7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe
//   7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise
//   1. von nicht mehr als 4 Stunden 30,00 EUR
//   2. von mehr als 4 bis 8 Stunden 50,00 EUR
//   3. von mehr als 8 Stunden 80,00 EUR
//   Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden.
//   7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe"></div>
//                 <div class="col-4"><label for="h1_auslagen" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Sonstige Auslagen"
//                     data-bs-content="Zum Beispiel:
//   7000 Pauschale für die Herstellung und Überlassung von Dokumenten:
//   für Kopien und Ausdrucke
//   für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR
//   für jede weitere Seite 0,15 EUR
//   für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR
//   für jede weitere Seite in Farbe 0,30 EUR
//   7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR.
//   Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet.
//   7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe
//   7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise
//   1. von nicht mehr als 4 Stunden 30,00 EUR
//   2. von mehr als 4 bis 8 Stunden 50,00 EUR
//   3. von mehr als 8 Stunden 80,00 EUR
//   Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden.
//   7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe">Sonstige Auslagen</label></div>
//                 <div class="col-2"></div>
//                 <div class="col-2">
//                   <div class="input-group">
//                     <input type="text" class="form-control" id="h1_auslagen" data-bs-toggle="popover"
//                       data-bs-trigger="hover" title="Sonstige Auslagen" data-bs-content="Zum Beispiel:
//   7000 Pauschale für die Herstellung und Überlassung von Dokumenten:
//   für Kopien und Ausdrucke
//   für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR
//   für jede weitere Seite 0,15 EUR
//   für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR
//   für jede weitere Seite in Farbe 0,30 EUR
//   7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR.
//   Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet.
//   7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe
//   7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise
//   1. von nicht mehr als 4 Stunden 30,00 EUR
//   2. von mehr als 4 bis 8 Stunden 50,00 EUR
//   3. von mehr als 8 Stunden 80,00 EUR
//   Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden.
//   7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe">
//                     <label class="input-group-text" for="h1_auslagen">EUR</label>
//                   </div>
//                 </div>
//               </div>
//               </p>
//               <h4>Gerichtskostengesetz</h4>
//               <div class="row fw-bold">
//                 <div class="col-1"></div>
//                 <div class="col-4">Gebührentatbestand und Nummer</div>
//                 <div class="col-2">Gebührensatz</div>
//                 <div class="col-5">Wertgebühr</div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h1_5110"></div>
//                 <div class="col-4"><label for="h1_5110">Verfahren im Allgemeinen, Nr. 5110</label></div>
//                 <div class="col-2">3,0</div>
//                 <div class="col-5" id="l_h1_5110"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h1_5111" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Gebühr Nr. 5110"
//                     data-bs-content="Eine solche Ermäßigung tritt beispielsweise in den meisten Fällen ein, wenn die Klage zurückgenommen wird, oder wenn beide Parteien das Verfahren übereinstimmend für erledigt erklären und sich untereinander auf die Verteilung der Kosten verständigen.">
//                 </div>
//                 <div class="col-4"><label for="h1_5111" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Gebühr Nr. 5110"
//                     data-bs-content="Eine solche Ermäßigung tritt beispielsweise in den meisten Fällen ein, wenn die Klage zurückgenommen wird, oder wenn beide Parteien das Verfahren übereinstimmend für erledigt erklären und sich untereinander auf die Verteilung der Kosten verständigen.">Ermäßigte
//                     Gebühr, Nr. 5111</label></div>
//                 <div class="col-2" data-bs-toggle="popover" data-bs-trigger="hover" title="Ermäßigung der Gebühr Nr. 5110"
//                   data-bs-content="Eine solche Ermäßigung tritt beispielsweise in den meisten Fällen ein, wenn die Klage zurückgenommen wird, oder wenn beide Parteien das Verfahren übereinstimmend für erledigt erklären und sich untereinander auf die Verteilung der Kosten verständigen.">
//                   1,0</div>
//                 <div class="col-5"></div>
//               </div>
//               </p>
//             </div>
  
//             <!-- 2. Instanz des Hauptsacheverfahrens -->
  
//             <div class="collapse" id="div_instanz_h2">
//               <h3>2. Instanz</h3>
//               <h4>Rechtsanwaltsvergütungsgesetz</h4>
//               <p>
//               <div class="row fw-bold">
//                 <div class="col-1"></div>
//                 <div class="col-4">Gebührentatbestand und Nummer</div>
//                 <div class="col-2">Gebührensatz</div>
//                 <div class="col-2">Wertgebühr (§ 13 RVG)</div>
//                 <div class="col-3">Wertgebühr (§ 49 RVG / Prozesskostenhilfe)</div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h2_3200"></div>
//                 <div class="col-4"><label for="h2_3200">Verfahrensgebühr, Nr. 3200</label></div>
//                 <div class="col-2">1,6</div>
//                 <div class="col-2" id="h2_3200_13"></div>
//                 <div class="col-3" id="h2_3200_49"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h2_3201" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Verfahrensgebühr Nr. 3200"
//                     data-bs-content="Die Ermäßiung tritt insbesondere ein, wenn der Auftrag endigt, bevor die*der Rechtsanwält*in das Rechtsmittel eingelegt oder einen Schriftsatz, der Sachanträge, Sachvortrag, die Zurücknahme der Klage oder die Zurücknahme des Rechtsmittels enthält, eingereicht oder bevor er einen gerichtlichen Termin wahrgenommen hat.">
//                 </div>
//                 <div class="col-4"><label for="h2_3201" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Verfahrensgebühr Nr. 3200"
//                     data-bs-content="Die Ermäßiung tritt insbesondere ein, wenn der Auftrag endigt, bevor die*der Rechtsanwält*in das Rechtsmittel eingelegt oder einen Schriftsatz, der Sachanträge, Sachvortrag, die Zurücknahme der Klage oder die Zurücknahme des Rechtsmittels enthält, eingereicht oder bevor er einen gerichtlichen Termin wahrgenommen hat.">Ermäßigte
//                     Verfahrensgebühr, Nr. 3201</label></div>
//                 <div class="col-2" data-bs-toggle="popover" data-bs-trigger="hover"
//                   title="Ermäßigung der Verfahrensgebühr Nr. 3200"
//                   data-bs-content="Die Ermäßiung tritt insbesondere ein, wenn der Auftrag endigt, bevor die*der Rechtsanwält*in das Rechtsmittel eingelegt oder einen Schriftsatz, der Sachanträge, Sachvortrag, die Zurücknahme der Klage oder die Zurücknahme des Rechtsmittels enthält, eingereicht oder bevor er einen gerichtlichen Termin wahrgenommen hat.">
//                   1,1</div>
//                 <div class="col-5"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h2_3202"></div>
//                 <div class="col-4"><label for="h2_3202">Terminsgebühr, Nr. 3202</label></div>
//                 <div class="col-2">1,2</div>
//                 <div class="col-2" id="h2_3202_13"></div>
//                 <div class="col-3" id="h2_3202_49"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h2_7002"></div>
//                 <div class="col-4"><label for="h2_7002">Auslagenpauschale, Nr. 7002</label></div>
//                 <div class="col-2"></div>
//                 <div class="col-2" id="h2_7002_13"></div>
//                 <div class="col-3" id="h2_7002_49"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h2_7000ua" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Sonstige Auslagen" data-bs-content="Zum Beispiel:
//   7000 Pauschale für die Herstellung und Überlassung von Dokumenten:
//   für Kopien und Ausdrucke
//   für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR
//   für jede weitere Seite 0,15 EUR
//   für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR
//   für jede weitere Seite in Farbe 0,30 EUR
//   7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR.
//   Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet.
//   7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe
//   7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise
//   1. von nicht mehr als 4 Stunden 30,00 EUR
//   2. von mehr als 4 bis 8 Stunden 50,00 EUR
//   3. von mehr als 8 Stunden 80,00 EUR
//   Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden.
//   7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe"></div>
//                 <div class="col-4"><label for="h2_auslagen" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Sonstige Auslagen"
//                     data-bs-content="Zum Beispiel:
//   7000 Pauschale für die Herstellung und Überlassung von Dokumenten:
//   für Kopien und Ausdrucke
//   für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR
//   für jede weitere Seite 0,15 EUR
//   für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR
//   für jede weitere Seite in Farbe 0,30 EUR
//   7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR.
//   Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet.
//   7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe
//   7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise
//   1. von nicht mehr als 4 Stunden 30,00 EUR
//   2. von mehr als 4 bis 8 Stunden 50,00 EUR
//   3. von mehr als 8 Stunden 80,00 EUR
//   Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden.
//   7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe">Sonstige Auslagen</label></div>
//                 <div class="col-2"></div>
//                 <div class="col-2">
//                   <div class="input-group">
//                     <input type="text" class="form-control" id="h2_auslagen" data-bs-toggle="popover"
//                       data-bs-trigger="hover" title="Sonstige Auslagen" data-bs-content="Zum Beispiel:
//   7000 Pauschale für die Herstellung und Überlassung von Dokumenten:
//   für Kopien und Ausdrucke
//   für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR
//   für jede weitere Seite 0,15 EUR
//   für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR
//   für jede weitere Seite in Farbe 0,30 EUR
//   7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR.
//   Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet.
//   7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe
//   7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise
//   1. von nicht mehr als 4 Stunden 30,00 EUR
//   2. von mehr als 4 bis 8 Stunden 50,00 EUR
//   3. von mehr als 8 Stunden 80,00 EUR
//   Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden.
//   7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe">
//                     <label class="input-group-text" for="h2_auslagen">EUR</label>
//                   </div>
//                 </div>
//               </div>
//               </p>
//               <h4>Gerichtskostengesetz</h4>
//               <div class="row fw-bold">
//                 <div class="col-1"></div>
//                 <div class="col-4">Gebührentatbestand und Nummer</div>
//                 <div class="col-2">Gebührensatz</div>
//                 <div class="col-5">Wertgebühr</div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h2_5122"></div>
//                 <div class="col-4"><label for="h2_5122">Verfahren im Allgemeinen, Nr. 5122</label></div>
//                 <div class="col-2">4,0</div>
//                 <div class="col-5" id="l_h2_5122"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h2_5120">
//                 </div>
//                 <div class="col-4"><label for="h2_5120">Verfahren über die Zulassung der Berufung, soweit der Antrag
//                     abgeleht wird, Nr. 5120</label></div>
//                 <div class="col-2">1,0</div>
//                 <div class="col-5"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h2_5121">
//                 </div>
//                 <div class="col-4"><label for="h2_5121">Verfahren über die Zulassung der Berufung, soweit der Antrag
//                     zurückgenommen oder das Verfahren durch anderweitige Erledigung beendet wird, Nr. 5121</label></div>
//                 <div class="col-2">0,5</div>
//                 <div class="col-5"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h2_5123" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Gebühr Nr. 5122"
//                     data-bs-content="Die Ermäßigung tritt ein, wenn das Verfahren durch Rücknahme der Klage oder Berufung beendigt wird, bevor die Berufung begründet wurde. Erledigungserklärungen nach § 161 Abs. 2 VwGO stehen der Zurücknahme gleich, wenn keine Entscheidung über die Kosten ergeht oder die Entscheidung einer zuvor mitgeteilten Einigung der Beteiligten über die Kostentragung oder der Kostenübernahmeerklärung eines Beteiligten folgt.">
//                 </div>
//                 <div class="col-4"><label for="h2_5123" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Gebühr Nr. 5122"
//                     data-bs-content="Die Ermäßigung tritt ein, wenn das Verfahren durch Rücknahme der Klage oder Berufung beendigt wird, bevor die Berufung begründet wurde. Erledigungserklärungen nach § 161 Abs. 2 VwGO stehen der Zurücknahme gleich, wenn keine Entscheidung über die Kosten ergeht oder die Entscheidung einer zuvor mitgeteilten Einigung der Beteiligten über die Kostentragung oder der Kostenübernahmeerklärung eines Beteiligten folgt.">Ermäßigte
//                     Gebühr, Nr. 5123</label></div>
//                 <div class="col-2" data-bs-toggle="popover" data-bs-trigger="hover" title="Ermäßigung der Gebühr Nr. 5122"
//                   data-bs-content="Die Ermäßigung tritt ein, wenn das Verfahren durch Rücknahme der Klage oder Berufung beendigt wird, bevor die Berufung begründet wurde. Erledigungserklärungen nach § 161 Abs. 2 VwGO stehen der Zurücknahme gleich, wenn keine Entscheidung über die Kosten ergeht oder die Entscheidung einer zuvor mitgeteilten Einigung der Beteiligten über die Kostentragung oder der Kostenübernahmeerklärung eines Beteiligten folgt.">
//                   1,0</div>
//                 <div class="col-5"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h2_5124" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Gebühr Nr. 5122"
//                     data-bs-content="Diese Ermäßigung tritt in der Regel ein, wenn die Klage oder die Berufung zurückgenommen wird, oder wenn beide Parteien das Verfahren übereinstimmend für erledigt erklären und sich untereinander auf die Verteilung der Kosten verständigen, nachdem die Berufung aber schon begründet wurde.">
//                 </div>
//                 <div class="col-4"><label for="h2_5124" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Gebühr Nr. 5122"
//                     data-bs-content="Diese Ermäßigung tritt in der Regel ein, wenn die Klage oder die Berufung zurückgenommen wird, oder wenn beide Parteien das Verfahren übereinstimmend für erledigt erklären und sich untereinander auf die Verteilung der Kosten verständigen, nachdem die Berufung aber schon begründet wurde.">Weitere
//                     Beendigung des Verfahrens, Nr. 5124</label></div>
//                 <div class="col-2" data-bs-toggle="popover" data-bs-trigger="hover" title="Ermäßigung der Gebühr Nr. 5122"
//                   data-bs-content="Diese Ermäßigung tritt in der Regel ein, wenn die Klage oder die Berufung zurückgenommen wird, oder wenn beide Parteien das Verfahren übereinstimmend für erledigt erklären und sich untereinander auf die Verteilung der Kosten verständigen, nachdem die Berufung aber schon begründet wurde.">
//                   2,0</div>
//                 <div class="col-5"></div>
//               </div>
//               </p>
//             </div>
  
//             <!-- 3. Instanz des Hauptsacheverfahrens -->
  
//             <div class="collapse" id="div_instanz_h3">
//               <h3>3. Instanz</h3>
//               <h4>Rechtsanwaltsvergütungsgesetz</h4>
//               <p>
//               <div class="row fw-bold">
//                 <div class="col-1"></div>
//                 <div class="col-4">Gebührentatbestand und Nummer</div>
//                 <div class="col-2">Gebührensatz</div>
//                 <div class="col-2">Wertgebühr (§ 13 RVG)</div>
//                 <div class="col-3">Wertgebühr (§ 49 RVG / Prozesskostenhilfe)</div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h3_3206"></div>
//                 <div class="col-4"><label for="h3_3206">Verfahrensgebühr, Nr. 3206</label></div>
//                 <div class="col-2">1,6</div>
//                 <div class="col-2" id="h3_3206_13"></div>
//                 <div class="col-3" id="h3_3206_49"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h3_3207" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Verfahrensgebühr Nr. 3206"
//                     data-bs-content="Die Ermäßiung tritt insbesondere ein, wenn der Auftrag endigt, bevor die*der Rechtsanwält*in das Rechtsmittel eingelegt oder einen Schriftsatz, der Sachanträge, Sachvortrag, die Zurücknahme der Klage oder die Zurücknahme des Rechtsmittels enthält, eingereicht oder bevor er einen gerichtlichen Termin wahrgenommen hat.">
//                 </div>
//                 <div class="col-4"><label for="h3_3207" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Verfahrensgebühr Nr. 3206"
//                     data-bs-content="Die Ermäßiung tritt insbesondere ein, wenn der Auftrag endigt, bevor die*der Rechtsanwält*in das Rechtsmittel eingelegt oder einen Schriftsatz, der Sachanträge, Sachvortrag, die Zurücknahme der Klage oder die Zurücknahme des Rechtsmittels enthält, eingereicht oder bevor er einen gerichtlichen Termin wahrgenommen hat.">Ermäßigte
//                     Verfahrensgebühr, Nr. 3207</label></div>
//                 <div class="col-2" data-bs-toggle="popover" data-bs-trigger="hover"
//                   title="Ermäßigung der Verfahrensgebühr Nr. 3206"
//                   data-bs-content="Die Ermäßiung tritt insbesondere ein, wenn der Auftrag endigt, bevor die*der Rechtsanwält*in das Rechtsmittel eingelegt oder einen Schriftsatz, der Sachanträge, Sachvortrag, die Zurücknahme der Klage oder die Zurücknahme des Rechtsmittels enthält, eingereicht oder bevor er einen gerichtlichen Termin wahrgenommen hat.">
//                   1,1</div>
//                 <div class="col-5"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h3_3210"></div>
//                 <div class="col-4"><label for="h3_3210">Terminsgebühr, Nr. 3210</label></div>
//                 <div class="col-2">1,5</div>
//                 <div class="col-2" id="h3_3210_13"></div>
//                 <div class="col-3" id="h3_3210_49"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h3_7002"></div>
//                 <div class="col-4"><label for="h3_7002">Auslagenpauschale, Nr. 7002</label></div>
//                 <div class="col-2"></div>
//                 <div class="col-2" id="h3_7002_13"></div>
//                 <div class="col-3" id="h3_7002_49"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h3_7000ua" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Sonstige Auslagen" data-bs-content="Zum Beispiel:
//   7000 Pauschale für die Herstellung und Überlassung von Dokumenten:
//   für Kopien und Ausdrucke
//   für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR
//   für jede weitere Seite 0,15 EUR
//   für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR
//   für jede weitere Seite in Farbe 0,30 EUR
//   7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR.
//   Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet.
//   7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe
//   7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise
//   1. von nicht mehr als 4 Stunden 30,00 EUR
//   2. von mehr als 4 bis 8 Stunden 50,00 EUR
//   3. von mehr als 8 Stunden 80,00 EUR
//   Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden.
//   7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe"></div>
//                 <div class="col-4"><label for="h3_auslagen" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Sonstige Auslagen"
//                     data-bs-content="Zum Beispiel:
//   7000 Pauschale für die Herstellung und Überlassung von Dokumenten:
//   für Kopien und Ausdrucke
//   für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR
//   für jede weitere Seite 0,15 EUR
//   für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR
//   für jede weitere Seite in Farbe 0,30 EUR
//   7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR.
//   Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet.
//   7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe
//   7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise
//   1. von nicht mehr als 4 Stunden 30,00 EUR
//   2. von mehr als 4 bis 8 Stunden 50,00 EUR
//   3. von mehr als 8 Stunden 80,00 EUR
//   Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden.
//   7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe">Sonstige Auslagen</label></div>
//                 <div class="col-2"></div>
//                 <div class="col-2">
//                   <div class="input-group">
//                     <input type="text" class="form-control" id="h3_auslagen" data-bs-toggle="popover"
//                       data-bs-trigger="hover" title="Sonstige Auslagen" data-bs-content="Zum Beispiel:
//   7000 Pauschale für die Herstellung und Überlassung von Dokumenten:
//   für Kopien und Ausdrucke
//   für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR
//   für jede weitere Seite 0,15 EUR
//   für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR
//   für jede weitere Seite in Farbe 0,30 EUR
//   7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR.
//   Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet.
//   7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe
//   7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise
//   1. von nicht mehr als 4 Stunden 30,00 EUR
//   2. von mehr als 4 bis 8 Stunden 50,00 EUR
//   3. von mehr als 8 Stunden 80,00 EUR
//   Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden.
//   7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe">
//                     <label class="input-group-text" for="h3_auslagen">EUR</label>
//                   </div>
//                 </div>
//               </div>
//               </p>
//               <h4>Gerichtskostengesetz</h4>
//               <div class="row fw-bold">
//                 <div class="col-1"></div>
//                 <div class="col-4">Gebührentatbestand und Nummer</div>
//                 <div class="col-2">Gebührensatz</div>
//                 <div class="col-5">Wertgebühr</div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h3_5130"></div>
//                 <div class="col-4"><label for="h3_5130">Verfahren im Allgemeinen, Nr. 5130</label></div>
//                 <div class="col-2">5,0</div>
//                 <div class="col-5" id="l_h3_5130"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h3_5131" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Gebühr Nr. 5130"
//                     data-bs-content="Die Ermäßigung tritt ein, wenn das Verfahren durch Rücknahme der Klage oder Revision beendigt wird, bevor die Revision begründet wurde. Erledigungserklärungen nach § 161 Abs. 2 VwGO stehen der Zurücknahme gleich, wenn keine Entscheidung über die Kosten ergeht oder die Entscheidung einer zuvor mitgeteilten Einigung der Beteiligten über die Kostentragung oder der Kostenübernahmeerklärung eines Beteiligten folgt.">
//                 </div>
//                 <div class="col-4"><label for="h3_5131" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Gebühr Nr. 5130"
//                     data-bs-content="Die Ermäßigung tritt ein, wenn das Verfahren durch Rücknahme der Klage oder Revision beendigt wird, bevor die Revision begründet wurde. Erledigungserklärungen nach § 161 Abs. 2 VwGO stehen der Zurücknahme gleich, wenn keine Entscheidung über die Kosten ergeht oder die Entscheidung einer zuvor mitgeteilten Einigung der Beteiligten über die Kostentragung oder der Kostenübernahmeerklärung eines Beteiligten folgt.">Ermäßigte
//                     Gebühr, Nr. 5131</label></div>
//                 <div class="col-2" data-bs-toggle="popover" data-bs-trigger="hover" title="Ermäßigung der Gebühr Nr. 5131"
//                   data-bs-content="Die Ermäßigung tritt ein, wenn das Verfahren durch Rücknahme der Klage oder Revision beendigt wird, bevor die Revision begründet wurde. Erledigungserklärungen nach § 161 Abs. 2 VwGO stehen der Zurücknahme gleich, wenn keine Entscheidung über die Kosten ergeht oder die Entscheidung einer zuvor mitgeteilten Einigung der Beteiligten über die Kostentragung oder der Kostenübernahmeerklärung eines Beteiligten folgt.">
//                   1,0</div>
//                 <div class="col-5"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="h3_5132" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Gebühr Nr. 5130"
//                     data-bs-content="Diese Ermäßigung tritt in der Regel ein, wenn die Klage oder die Revision zurückgenommen wird, oder wenn beide Parteien das Verfahren übereinstimmend für erledigt erklären und sich untereinander auf die Verteilung der Kosten verständigen, nachdem die Revision aber schon begründet wurde.">
//                 </div>
//                 <div class="col-4"><label for="h3_5132" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Gebühr Nr. 5130"
//                     data-bs-content="Diese Ermäßigung tritt in der Regel ein, wenn die Klage oder die Revision zurückgenommen wird, oder wenn beide Parteien das Verfahren übereinstimmend für erledigt erklären und sich untereinander auf die Verteilung der Kosten verständigen, nachdem die Revision aber schon begründet wurde.">Weitere
//                     Beendigung des Verfahrens, Nr. 5132</label></div>
//                 <div class="col-2" data-bs-toggle="popover" data-bs-trigger="hover" title="Ermäßigung der Gebühr Nr. 5130"
//                   data-bs-content="Diese Ermäßigung tritt in der Regel ein, wenn die Klage oder die Revision zurückgenommen wird, oder wenn beide Parteien das Verfahren übereinstimmend für erledigt erklären und sich untereinander auf die Verteilung der Kosten verständigen, nachdem die Revision aber schon begründet wurde.">
//                   3,0</div>
//                 <div class="col-5"></div>
//               </div>
//               </p>
//             </div>
  
//             <h3>Summen</h3>
//             <div class="row fw-bold">
//               <div class="col-6">
//                 Rechtsanwaltsvergütungsgesetz (§ 13 RVG)
//               </div>
//               <div class="col-3 text-end" id="summe_rvg13_h"></div>
//               <div class="col-3"></div>
//             </div>
//             <div class="row fw-bold">
//               <div class="col-6">
//                 Rechtsanwaltsvergütungsgesetz (§ 49 RVG)
//               </div>
//               <div class="col-3 text-end" id="summe_rvg49_h"></div>
//               <div class="col-3"></div>
//             </div>
//             <div class="row fw-bold">
//               <div class="col-6">
//                 Gerichtskostengesetz
//               </div>
//               <div class="col-3 text-end" id="summe_gkg_h"></div>
//               <div class="col-3"></div>
//             </div>
//           </div>
  
//           <div class="container border border-5 rounded p-4 m-4 collapse" id="vorlaeufig">
//             <h2>Vorläufiger Rechtsschutz</h2>
//             <p>
//               <input type="checkbox" id="instanz_v1">
//               <label for="instanz_v1">1. Instanz</label>
//               <input type="checkbox" id="instanz_v2">
//               <label for="instanz_v2">2. Instanz</label>
//             </p>
  
//             <!-- 1. Instanz des Verfahrens zum vorläufigen Rechtsschutz -->
  
//             <div class="collapse" id="div_instanz_v1">
//               <h3>1. Instanz</h3>
//               <h4>Rechtsanwaltsvergütungsgesetz</h4>
//               <p>
//               <div class="row fw-bold">
//                 <div class="col-1"></div>
//                 <div class="col-4">Gebührentatbestand und Nummer</div>
//                 <div class="col-2">Gebührensatz</div>
//                 <div class="col-2">Wertgebühr (§ 13 RVG)</div>
//                 <div class="col-3">Wertgebühr (§ 49 RVG / Prozesskostenhilfe)</div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="v1_3100"></div>
//                 <div class="col-4"><label for="v1_3100">Verfahrensgebühr, Nr. 3100</label></div>
//                 <div class="col-2">1,3</div>
//                 <div class="col-2" id="v1_3100_13"></div>
//                 <div class="col-3" id="v1_3100_49"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="v1_3101" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Verfahrensgebühr Nr. 3100"
//                     data-bs-content="Die Ermäßiung tritt insbesondere ein, wenn der Auftrag endigt, bevor die*der Rechtsanwält*in die Klage, den ein Verfahren einleitenden Antrag oder einen Schriftsatz, der Sachanträge, Sachvortrag, die Zurücknahme der Klage oder die Zurücknahme des Antrags enthält, eingereicht oder bevor sie*er einen gerichtlichen Termin wahrgenommen hat.">
//                 </div>
//                 <div class="col-4"><label for="h1_3101" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Verfahrensgebühr Nr. 3100"
//                     data-bs-content="Die Ermäßiung tritt insbesondere ein, wenn der Auftrag endigt, bevor die*der Rechtsanwält*in die Klage, den ein Verfahren einleitenden Antrag oder einen Schriftsatz, der Sachanträge, Sachvortrag, die Zurücknahme der Klage oder die Zurücknahme des Antrags enthält, eingereicht oder bevor sie*er einen gerichtlichen Termin wahrgenommen hat.">Ermäßigte
//                     Verfahrensgebühr, Nr. 3101</label></div>
//                 <div class="col-2" data-bs-toggle="popover" data-bs-trigger="hover"
//                   title="Ermäßigung der Verfahrensgebühr Nr. 3100"
//                   data-bs-content="Die Ermäßiung tritt insbesondere ein, wenn der Auftrag endigt, bevor die*der Rechtsanwält*in die Klage, den ein Verfahren einleitenden Antrag oder einen Schriftsatz, der Sachanträge, Sachvortrag, die Zurücknahme der Klage oder die Zurücknahme des Antrags enthält, eingereicht oder bevor sie*er einen gerichtlichen Termin wahrgenommen hat.">
//                   0,8</div>
//                 <div class="col-5"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="v1_3104"></div>
//                 <div class="col-4"><label for="v1_3104">Terminsgebühr, Nr. 3104</label></div>
//                 <div class="col-2">1,2</div>
//                 <div class="col-2" id="v1_3104_13"></div>
//                 <div class="col-3" id="v1_3104_49"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="v1_7002"></div>
//                 <div class="col-4"><label for="v1_7002">Auslagenpauschale, Nr. 7002</label></div>
//                 <div class="col-2"></div>
//                 <div class="col-2" id="v1_7002_13"></div>
//                 <div class="col-3" id="v1_7002_49"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="v1_7000ua" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Sonstige Auslagen" data-bs-content="Zum Beispiel:
//   7000 Pauschale für die Herstellung und Überlassung von Dokumenten:
//   für Kopien und Ausdrucke
//   für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR
//   für jede weitere Seite 0,15 EUR
//   für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR
//   für jede weitere Seite in Farbe 0,30 EUR
//   7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR.
//   Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet.
//   7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe
//   7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise
//   1. von nicht mehr als 4 Stunden 30,00 EUR
//   2. von mehr als 4 bis 8 Stunden 50,00 EUR
//   3. von mehr als 8 Stunden 80,00 EUR
//   Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden.
//   7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe"></div>
//                 <div class="col-4"><label for="v1_auslagen" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Sonstige Auslagen"
//                     data-bs-content="Zum Beispiel:
//   7000 Pauschale für die Herstellung und Überlassung von Dokumenten:
//   für Kopien und Ausdrucke
//   für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR
//   für jede weitere Seite 0,15 EUR
//   für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR
//   für jede weitere Seite in Farbe 0,30 EUR
//   7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR.
//   Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet.
//   7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe
//   7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise
//   1. von nicht mehr als 4 Stunden 30,00 EUR
//   2. von mehr als 4 bis 8 Stunden 50,00 EUR
//   3. von mehr als 8 Stunden 80,00 EUR
//   Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden.
//   7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe">Sonstige Auslagen</label></div>
//                 <div class="col-2"></div>
//                 <div class="col-2">
//                   <div class="input-group">
//                     <input type="text" class="form-control" id="v1_auslagen" data-bs-toggle="popover"
//                       data-bs-trigger="hover" title="Sonstige Auslagen" data-bs-content="Zum Beispiel:
//   7000 Pauschale für die Herstellung und Überlassung von Dokumenten:
//   für Kopien und Ausdrucke
//   für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR
//   für jede weitere Seite 0,15 EUR
//   für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR
//   für jede weitere Seite in Farbe 0,30 EUR
//   7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR.
//   Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet.
//   7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe
//   7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise
//   1. von nicht mehr als 4 Stunden 30,00 EUR
//   2. von mehr als 4 bis 8 Stunden 50,00 EUR
//   3. von mehr als 8 Stunden 80,00 EUR
//   Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden.
//   7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe">
//                     <label class="input-group-text" for="v1_auslagen">EUR</label>
//                   </div>
//                 </div>
//               </div>
//               </p>
//               <h4>Gerichtskostengesetz</h4>
//               <div class="row fw-bold">
//                 <div class="col-1"></div>
//                 <div class="col-4">Gebührentatbestand und Nummer</div>
//                 <div class="col-2">Gebührensatz</div>
//                 <div class="col-5">Wertgebühr</div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="v1_5210"></div>
//                 <div class="col-4"><label for="v1_5210">Verfahren im Allgemeinen, Nr. 5210</label></div>
//                 <div class="col-2">1,5</div>
//                 <div class="col-5" id="l_v1_5210"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="v1_5211" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Gebühr Nr. 5210"
//                     data-bs-content="Eine solche Ermäßigung tritt beispielsweise in den meisten Fällen ein, wenn der Antrag zurückgenommen wird, oder wenn beide Parteien das Verfahren übereinstimmend für erledigt erklären und sich untereinander auf die Verteilung der Kosten verständigen.">
//                 </div>
//                 <div class="col-4"><label for="v1_5211" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Gebühr Nr. 5210"
//                     data-bs-content="Eine solche Ermäßigung tritt beispielsweise in den meisten Fällen ein, wenn der Antrag zurückgenommen wird, oder wenn beide Parteien das Verfahren übereinstimmend für erledigt erklären und sich untereinander auf die Verteilung der Kosten verständigen.">Ermäßigte
//                     Gebühr, Nr. 5211</label></div>
//                 <div class="col-2" data-bs-toggle="popover" data-bs-trigger="hover" title="Ermäßigung der Gebühr Nr. 5210"
//                   data-bs-content="Eine solche Ermäßigung tritt beispielsweise in den meisten Fällen ein, wenn der Antrag zurückgenommen wird, oder wenn beide Parteien das Verfahren übereinstimmend für erledigt erklären und sich untereinander auf die Verteilung der Kosten verständigen.">
//                   0,5</div>
//                 <div class="col-5"></div>
//               </div>
//               </p>
//             </div>
  
//             <!-- 2. Instanz des Verfahrens zum vorläufigen Rechtsschutz -->
  
//             <div class="collapse" id="div_instanz_v2">
//               <h3>2. Instanz</h3>
//               <h4>Rechtsanwaltsvergütungsgesetz</h4>
//               <p>
//               <div class="row fw-bold">
//                 <div class="col-1"></div>
//                 <div class="col-4">Gebührentatbestand und Nummer</div>
//                 <div class="col-2">Gebührensatz</div>
//                 <div class="col-2">Wertgebühr (§ 13 RVG)</div>
//                 <div class="col-3">Wertgebühr (§ 49 RVG / Prozesskostenhilfe)</div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="v2_3200"></div>
//                 <div class="col-4"><label for="v2_3200">Verfahrensgebühr, Nr. 3100</label></div>
//                 <div class="col-2">1,6</div>
//                 <div class="col-2" id="v2_3200_13"></div>
//                 <div class="col-3" id="v2_3200_49"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="v2_3201" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Verfahrensgebühr Nr. 3200"
//                     data-bs-content="Die Ermäßiung tritt insbesondere ein, wenn der Auftrag endigt, bevor die*der Rechtsanwält*in das Rechtsmittel eingelegt oder einen Schriftsatz, der Sachanträge, Sachvortrag, die Zurücknahme der Klage oder die Zurücknahme des Rechtsmittels enthält, eingereicht oder bevor er einen gerichtlichen Termin wahrgenommen hat.">
//                 </div>
//                 <div class="col-4"><label for="v2_3201" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Verfahrensgebühr Nr. 3200"
//                     data-bs-content="Die Ermäßiung tritt insbesondere ein, wenn der Auftrag endigt, bevor die*der Rechtsanwält*in das Rechtsmittel eingelegt oder einen Schriftsatz, der Sachanträge, Sachvortrag, die Zurücknahme der Klage oder die Zurücknahme des Rechtsmittels enthält, eingereicht oder bevor er einen gerichtlichen Termin wahrgenommen hat.">Ermäßigte
//                     Verfahrensgebühr, Nr. 3201</label></div>
//                 <div class="col-2" data-bs-toggle="popover" data-bs-trigger="hover"
//                   title="Ermäßigung der Verfahrensgebühr Nr. 3200"
//                   data-bs-content="Die Ermäßiung tritt insbesondere ein, wenn der Auftrag endigt, bevor die*der Rechtsanwält*in das Rechtsmittel eingelegt oder einen Schriftsatz, der Sachanträge, Sachvortrag, die Zurücknahme der Klage oder die Zurücknahme des Rechtsmittels enthält, eingereicht oder bevor er einen gerichtlichen Termin wahrgenommen hat.">
//                   1,1</div>
//                 <div class="col-5"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="v2_3202"></div>
//                 <div class="col-4"><label for="v2_3202">Terminsgebühr, Nr. 3202</label></div>
//                 <div class="col-2">1,2</div>
//                 <div class="col-2" id="v2_3202_13"></div>
//                 <div class="col-3" id="v2_3202_49"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="v2_7002"></div>
//                 <div class="col-4"><label for="v2_7002">Auslagenpauschale, Nr. 7002</label></div>
//                 <div class="col-2"></div>
//                 <div class="col-2" id="v2_7002_13"></div>
//                 <div class="col-3" id="v2_7002_49"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="v2_7000ua" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Sonstige Auslagen" data-bs-content="Zum Beispiel:
//   7000 Pauschale für die Herstellung und Überlassung von Dokumenten:
//   für Kopien und Ausdrucke
//   für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR
//   für jede weitere Seite 0,15 EUR
//   für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR
//   für jede weitere Seite in Farbe 0,30 EUR
//   7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR.
//   Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet.
//   7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe
//   7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise
//   1. von nicht mehr als 4 Stunden 30,00 EUR
//   2. von mehr als 4 bis 8 Stunden 50,00 EUR
//   3. von mehr als 8 Stunden 80,00 EUR
//   Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden.
//   7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe"></div>
//                 <div class="col-4"><label for="v2_auslagen" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Sonstige Auslagen"
//                     data-bs-content="Zum Beispiel:
//   7000 Pauschale für die Herstellung und Überlassung von Dokumenten:
//   für Kopien und Ausdrucke
//   für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR
//   für jede weitere Seite 0,15 EUR
//   für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR
//   für jede weitere Seite in Farbe 0,30 EUR
//   7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR.
//   Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet.
//   7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe
//   7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise
//   1. von nicht mehr als 4 Stunden 30,00 EUR
//   2. von mehr als 4 bis 8 Stunden 50,00 EUR
//   3. von mehr als 8 Stunden 80,00 EUR
//   Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden.
//   7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe">Sonstige Auslagen</label></div>
//                 <div class="col-2"></div>
//                 <div class="col-2">
//                   <div class="input-group">
//                     <input type="text" class="form-control" id="v2_auslagen" data-bs-toggle="popover"
//                       data-bs-trigger="hover" title="Sonstige Auslagen" data-bs-content="Zum Beispiel:
//   7000 Pauschale für die Herstellung und Überlassung von Dokumenten:
//   für Kopien und Ausdrucke
//   für die ersten 50 abzurechnenden Seiten je Seite 0,50 EUR
//   für jede weitere Seite 0,15 EUR
//   für die ersten 50 abzurechnenden Seiten in Farbe je Seite 1,00 EUR
//   für jede weitere Seite in Farbe 0,30 EUR
//   7003 Fahrtkosten für eine Geschäftsreise bei Benutzung eines eigenen Kraftfahrzeugs für jeden gefahrenen Kilometer 0,42 EUR.
//   Die Umsatzsteuer (Nr. 7008) VV RVG wird unten, unter „Summe“ berechnet.
//   7004 Fahrtkosten für eine Geschäftsreise bei Benutzung eines anderen Verkehrsmittels, soweit sie angemessen sind in voller Höhe
//   7005 Tage- und Abwesenheitsgeld bei einer Geschäftsreise
//   1. von nicht mehr als 4 Stunden 30,00 EUR
//   2. von mehr als 4 bis 8 Stunden 50,00 EUR
//   3. von mehr als 8 Stunden 80,00 EUR
//   Bei Auslandsreisen kann zu diesen Beträgen ein Zuschlag von 50 % berechnet werden.
//   7006 Sonstige Auslagen (z.B. Hotel) anlässlich einer Geschäftsreise, soweit sie angemessen sind in voller Höhe">
//                     <label class="input-group-text" for="v2_auslagen">EUR</label>
//                   </div>
//                 </div>
//               </div>
//               </p>
//               <h4>Gerichtskostengesetz</h4>
//               <div class="row fw-bold">
//                 <div class="col-1"></div>
//                 <div class="col-4">Gebührentatbestand und Nummer</div>
//                 <div class="col-2">Gebührensatz</div>
//                 <div class="col-5">Wertgebühr</div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="v2_5240"></div>
//                 <div class="col-4"><label for="v2_5240">Verfahren im Allgemeinen, Nr. 5240</label></div>
//                 <div class="col-2">2,0</div>
//                 <div class="col-5" id="l_v2_5240"></div>
//               </div>
//               </p>
//               <p>
//               <div class="row">
//                 <div class="col-1"><input type="checkbox" id="v2_5241" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Gebühr Nr. 5240"
//                     data-bs-content="Die Ermäßigung tritt ein, wenn das Verfahren durch Rücknahme der Beschwerde beendigt wird.">
//                 </div>
//                 <div class="col-4"><label for="v2_5241" data-bs-toggle="popover" data-bs-trigger="hover"
//                     title="Ermäßigung der Gebühr Nr. 5240"
//                     data-bs-content="Die Ermäßigung tritt ein, wenn das Verfahren durch Rücknahme der Beschwerde beendigt wird.">Ermäßigte
//                     Gebühr, Nr. 5241</label></div>
//                 <div class="col-2" data-bs-toggle="popover" data-bs-trigger="hover" title="Ermäßigung der Gebühr Nr. 5240"
//                   data-bs-content="Die Ermäßigung tritt ein, wenn das Verfahren durch Rücknahme der Beschwerde beendigt wird.">
//                   1,0</div>
//                 <div class="col-5"></div>
//               </div>
//               </p>
//             </div>
  
//             <h3>Summen</h3>
//             <div class="row fw-bold">
//               <div class="col-6">
//                 Rechtsanwaltsvergütungsgesetz (§ 13 RVG)
//               </div>
//               <div class="col-3 text-end" id="summe_rvg13_v"></div>
//               <div class="col-3"></div>
//             </div>
//             <div class="row fw-bold">
//               <div class="col-6">
//                 Rechtsanwaltsvergütungsgesetz (§ 49 RVG)
//               </div>
//               <div class="col-3 text-end" id="summe_rvg49_v"></div>
//               <div class="col-3"></div>
//             </div>
//             <div class="row fw-bold">
//               <div class="col-6">
//                 Gerichtskostengesetz
//               </div>
//               <div class="col-3 text-end" id="summe_gkg_v"></div>
//               <div class="col-3"></div>
//             </div>
//           </div>
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Summe"
            </h2>
            <div class="grid grid-cols-4">
                <div class="col-span-4 pt-4 text-xl font-medium">
                    "Summe Rechtsanwaltsvergütungsgesetz"
                </div>
                <div></div>
                <div class="font-semibold">
                    "Wertgebühren (§ 13 RVG)"
                </div>
                <div class="font-semibold">
                    "Wertgebühren (§ 49 RVG / PKH)"
                </div>
                <div class="font-semibold">
                    "Differenz"
                </div>
                <div class=move || if a.get().unwrap_or(false) == true { "visible" } else { "hidden" }>
                    "Außergerichtliche Vertretung"
                </div>
                <div class=move || if a.get().unwrap_or(false) == true { "visible" } else { "hidden" }>
                    { move || format_euro(summe_aussergerichtlich.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class=move || if a.get().unwrap_or(false) == true { "visible" } else { "hidden" }>
                    { move || format_euro(summe_aussergerichtlich.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div></div>
                <div class=move || if v.get().unwrap_or(0) != 1 { "visible col-span-4"} else { "hidden col-span-4" }>
                    "Hauptsacheverfahren"
                </div>
                <div class=move || if v.get().unwrap_or(0) != 0 { "visible col-span-4"} else { "hidden col-span-4" }>
                    "Vorläufiger Rechtsschutz"
                </div>
                <div class="italic">
                    "Summe netto"
                </div>
                <div class="italic">
                    { move || format_euro(summe_rvg13_netto.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class="italic">
                    { move || format_euro(summe_rvg49_netto.get()) }
                    <span class="ml-1">EUR</span>
                </div>            
                <div class="italic">
                    { move || format_euro(summe_rvg13_netto.get() - summe_rvg49_netto.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div>
                    "Umsatzsteuer, Nr. 7008 VV RVG"
                    <input type="number" min="0" value=move || u.get().unwrap_or(19) class="border-2 border-stone-400 rounded-lg px-1" on:change=change_umsatzsteuer />
                    <span class="ml-1">%</span>
                    <button popovertarget="umsatzsteuer" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                    <div id="umsatzsteuer" popover class="open:border-2 open:border-stone-400 open:rounded-lg open:p-2 open:mt-60 open:mx-60">
                        <h4 class="text-xl font-medium">Steuersatz der Umsatzsteuer</h4>
                        <p>{ popover::UMSATZSTEUER }</p>
                    </div>
                </div>
                <div>
                </div>
                <div>
                </div>
                <div>
                </div>
                // <div class="col-span-4 pt-4 text-xl font-medium">
                //     "Summe Gerichtskostengesetz"
                // </div>
                <div class="pt-4 text-xl font-medium">
                    "Gesamtsumme"
                </div>
                <div class="pt-4 text-xl font-medium">
                </div>
                <div class="pt-4 text-xl font-medium">
                </div>
                <div class="pt-4 text-xl font-medium">
                </div>
            </div>
        </div>  

//                   <label class="input-group-text" for="steuersatz">%</label>
//                 </div>
//               </div>
//               <div class="col-3 d-grid align-items-center text-end">
//                 <div id="umsatzsteuer"></div>
//               </div>
//               <div class="col-3"></div>
//             </div>
  
//             <div class="row">
//               <div class="col-4 fw-bold">
//                 <label>Summe brutto</label>
//               </div>
//               <div class="col-2"></div>
//               <div class="col-3 fw-bold text-end">
//                 <div id="summe_brutto"></div>
//               </div>
//               <div class="col-3"></div>
//             </div>
  
//             <h3>Summe Gerichtskostengesetz</h3>
//             <div class="row collapse" id="row_summe_gkg_h">
//               <div class="col-4">
//                 <label>Hauptsacheverfahren</label>
//               </div>
//               <div class="col-2"></div>
//               <div class="col-3 text-end" id="l_summe_gkg_h"></div>
//               <div class="col-3"></div>
//             </div>
  
//             <div class="row collapse" id="row_summe_gkg_v">
//               <div class="col-4">
//                 <label>Vorläufiger Rechtsschutz</label>
//               </div>
//               <div class="col-2"></div>
//               <div class="col-3 text-end" id="l_summe_gkg_v"></div>
//               <div class="col-3"></div>
//             </div>
  
//             <div class="row">
//               <div class="col-4 fw-bold">
//                 <label>Summe</label>
//               </div>
//               <div class="col-2"></div>
//               <div class="col-3 fw-bold text-end" id="l_summe_gkg"></div>
//               <div class="col-3"></div>
//             </div>
  
//             <div class="row">
//               <div class="col-4">
//                 <h3>Gesamtsumme</h3>
//               </div>
//               <div class="col-2"></div>
//               <div class="col-3 text-end" id="l_summe_total"></div>
//               <div class="col-3"></div>
//             </div>
//           </div>
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Rechtliche Hinweise"
            </h2>
            <p>
                "Dieser Prozesskostenrechner berechnet gesetzliche Gebühren auf der Grundlage des 
                Rechtsanwaltsvergütungsgesetzes ("<a class="text-blue-600 hover:underline hover:text-violet-600" href="https://dejure.org/gesetze/RVG">"RVG"</a>
                "), des Gerichtskostengesetzes ("<a class="text-blue-600 hover:underline hover:text-violet-600" href="https://dejure.org/gesetze/GKG">"GKG"</a>
                "), des "<a class="text-blue-600 hover:underline hover:text-violet-600" href="https://www.bverwg.de/rechtsprechung/streitwertkatalog">"Streitwertkatalogs"</a>
                " des Bundesverwaltungsgerichts und meiner Erfahrung mit der Interpretation dieser Vorgaben durch die
                Verwaltungsgerichte vornehmlich in NRW. Der Rechner dient nur einer unverbindlichen
                Orientierung und kann eine fachkundige Beratung nicht ersetzen. Seine Nutzung erfolgt insofern auf eigene
                Gefahr. Es kann eine Vielzahl von Gründen geben, warum ein Gericht höhere oder niedrigere Kosten festsetzt,
                als von diesem Rechner ermittelt. Auch möchten viele Rechtsanwält*innen Vergütungsvereinbarungen schließen,
                die zum Teil deutlich von den gesetzlich vorgesehenen Gebühren abweichen. Der Rechner geht zudem auch davon
                aus, dass sich die Behördenseite nicht anwaltlich vertreten lässt. Tatsächlich lassen sich Behörden in
                migrationsrechtlichen Streitigkeiten erfahungsgemäß selten anwaltlich vertreten. Völlig ausgeschlossen ist
                es aber auch nicht. Ggf. würden hierdurch weitere Kosten entstehen, die dieser Rechner nicht berücksichtigt.
                Außerdem berücksichtigt dieser Rechner keine Gebühren, die durch Behörden im Verwaltungsverfahren erhoben
                werden. Widerspruchs- und Remonstrationsverfahren werden ebenfalls (noch?) nicht abgebildet, was vor allem
                daran liegt, dass ich hauptsächlich in NRW tätig bin, wo es kaum noch Widerspruchsverfahren gibt. Die
                Anrechnung der Geschäftsgebühr auf die Verfahrensgebühr (Vorbemerkung 4 zu Teil 3 VV RVG) erfolgt immer auf
                die 1. Instanz des Hauptsacheverfahrens, da dies auch in der Praxis nahezu immer der Fall sein wird. Soweit
                zumindest theoretisch auch Fälle konstruiert werden können, in denen die Anrechnung auf die Verfahrensgebühr
                in einer höheren Insatz erfolgt, bleiben diese Fälle hier um der Einfachheit willen unberücksichtigt."
            </p>
            <p>
                "Die Abkürzung PKH steht für Prozesskostenhilfe. Da die Wertgebühren bei Prozesskostenhilfe teilweise abweichen,
                werden die entsprechenden Gebühren gesondert ausgewiesen. Für außergerichtliche Vertretung gibt es keine
                Prozesskostenhilfe, daher werden für die außergerichtliche Vertretung immer die Wergebühren nach § 13 RVG ausgewiesen."
            </p>
            <p>
                "Der Rechner geht äußerst sparsam mit deinen Daten um. Zwar werden einige technisch benötigte Daten,
                insbesondere deine IP-Adresse, an meinen Server gesendet und von meinem Server verarbeitet. Das ist nötig,
                um die zum Rechner gehörenden Dateien an deinen Browser oder dein sonstiges Gerät, mit welchem du den
                Rechner ausführst, schicken zu können. Der Rechner wird aber vollständig auf deinem Gerät ausgeführt. Das
                bedeutet, dass alle Daten, die du in den Rechner eingibst, und die Ergebnisse, die von meinem Rechner
                erzeugt werden, vollständig bei dir verbleiben, und erst gar nicht an meinen Server geschickt, geschweige
                denn verarbeitet oder gespeichert werden."
            </p>
            <p>
                "Der Rechner ist zudem auch als Freie Software unter den Lizenzen Apache, Version 2.0, und MIT
                veröffentlicht. Du kannst dir die Software also auch aus dem "<a class="text-blue-600 hover:underline hover:text-violet-600" href="https://github.com/dusmarcel/mpkr25">
                "Repository"</a>" herunterladen und sie dann ganz auf einem Gerät deiner Wahl ausführen.
                In diesem Falle hast du mit meinem Server gar nichts mehr zu tun, und die Notwendigkeit, Daten an meinen
                Server zu übertragen, entfällt ganz."
            </p>
            <p>
                "Und schließlich geht es hier noch zu meinem „"<a class="text-blue-600 hover:underline hover:text-violet-600" href="https://aufentha.lt/index.php/impressum/">"Impressum"</a>"“."
            </p>
        </div>
    }
}