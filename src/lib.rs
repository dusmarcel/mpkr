use fees::{pauschale, umsatzsteuer_brutto};
use leptos::prelude::*;
use leptos_router::{hooks::query_signal_with_options, location::State, NavigateOptions};

mod popover;
mod fees;
mod utils;
use utils::format_euro;

#[component]
pub fn MPKR() -> impl IntoView {
    // Streitwerte
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

    // Hauptsacheverfahren

    // Welche Instanzen?
    let (h1, set_h1) = query_signal_with_options::<bool>(
        "h1",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_h1 = move |ev| set_h1.set(Some(event_target_checked(&ev)));

    let (h2, set_h2) = query_signal_with_options::<bool>(
        "h2",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_h2 = move |ev| set_h2.set(Some(event_target_checked(&ev)));

    let (h3, set_h3) = query_signal_with_options::<bool>(
        "h3",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_h3 = move |ev| set_h3.set(Some(event_target_checked(&ev)));

    // 1. Instanz Hauptsachverfahren

    //RVG
    let (n3100, set_n3100) = query_signal_with_options::<bool>(
        "n3100",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n3100 = move |ev| set_n3100.set(Some(event_target_checked(&ev)));

    let (n3101, set_n3101) = query_signal_with_options::<bool>(
        "n3101",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n3101 = move |ev| set_n3101.set(Some(event_target_checked(&ev)));

    let verfgeb13_h1 = Memo::new( move |_| {
        if n3101.get().unwrap_or(false) {
            0.8 * fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else {
            if n3100.get().unwrap_or(true) {
                1.3 * fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
            } else {
                0.0
            }
        }
    });

    let verfgeb49_h1 = Memo::new( move |_| {
        if n3101.get().unwrap_or(false) {
            0.8 * fees::rvg49_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else {
            if n3100.get().unwrap_or(true) {
                1.3 * fees::rvg49_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
            } else {
                0.0
            }
        }
    });

    let (anr, set_anr) = query_signal_with_options::<bool>(
        "anr",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_anrechnung = move |ev| set_anr.set(Some(event_target_checked(&ev)));

    let anrechnung13 = Memo::new(move |_| {
        if anr.get().unwrap_or(a.get().unwrap_or(false)) && a.get().unwrap_or(false) && g.get().unwrap_or(true) && (n3100.get().unwrap_or(true) || n3101.get().unwrap_or(false)) {
            let mut anrechnungsbetrag = 0.5 * n2300.get();
            if anrechnungsbetrag > 0.75 * fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT)) {
                anrechnungsbetrag = 0.75 * fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT));
            }
            anrechnungsbetrag
        } else {
            0.0
        }        
    });

    let anrechnung49 = Memo::new(move |_| {
        let mut anrechnungsbetrag = anrechnung13.get();
        let differenz = verfgeb13_h1.get() - verfgeb49_h1.get();
        anrechnungsbetrag -= differenz;
        if anrechnungsbetrag <= 0.0 {
            anrechnungsbetrag = 0.0;
        }
        anrechnungsbetrag      
    });

    let (n3104, set_n3104) = query_signal_with_options::<bool>(
        "n3104",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_n3104 = move |ev| set_n3104.set(Some(event_target_checked(&ev)));

    let tgeb13_h1 = Memo::new( move |_| {
        if n3104.get().unwrap_or(true) {
            1.2 * fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else {
            0.0
        }
    });

    let tgeb49_h1 = Memo::new( move |_| {
        if n3104.get().unwrap_or(true) {
            1.2 * fees::rvg49_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
        } else {
            0.0
        }
    });

    let (h1p, set_h1p) = query_signal_with_options::<bool>(
        "h1p",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_h1_pauschale = move |ev| set_h1p.set(Some(event_target_checked(&ev)));

    let pauschale13_h1 = Memo::new( move |_| {
        if h1p.get().unwrap_or(true) {
            fees::pauschale(verfgeb13_h1.get () + tgeb13_h1.get())
        } else {
            0.0
        }
    });

    let pauschale49_h1 = Memo::new( move |_| {
        if h1p.get().unwrap_or(true) {
            fees::pauschale(verfgeb49_h1.get () + tgeb49_h1.get())
        } else {
            0.0
        }
    });

    let (h1a, set_h1a) = query_signal_with_options::<bool>(
        "h1a",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_h1_auslagen = move |ev| set_h1a.set(Some(event_target_checked(&ev)));

    let (h1sa, set_h1sa) = query_signal_with_options::<f64>(
        "h1sa",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_h1_sonstige_auslagen = move |ev| set_h1sa.set(Some(event_target_value(&ev).parse::<f64>().unwrap_or(0.0))); 

    // GKG

    // 2. Instanz Hauptsacheverfahren

    // RVG

    // GKG

    // 3. Instanz Hauptsacheverfahren

    // RVG

    // GKG

    // Summen Hauptsacheverfahren
    let summe_rvg13_h = Memo::new(move |_| {
        let mut summe = verfgeb13_h1.get() - anrechnung13.get() + tgeb13_h1.get() + pauschale13_h1.get();
        if h1a.get().unwrap_or(false) {summe += h1sa.get().unwrap_or(0.0) }
        summe
    });

    let summe_rvg49_h = Memo::new(move |_| {
        let mut summe = verfgeb49_h1.get() - anrechnung49.get() + tgeb49_h1.get() + pauschale49_h1.get();
        if h1a.get().unwrap_or(false) {summe += h1sa.get().unwrap_or(0.0) }
        summe        
    });

    let summe_gkg_h = Memo::new(move |_| {
        0.0
    });

    // Vorläufiger Rechtsschutz

    // 1. Instanz

    // RVG

    // GKG

    // 2. Instanz

    // RVG

    // GKG

    // Welche Instanzen?
    let (v1, set_v1) = query_signal_with_options::<bool>(
        "v1",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_v1 = move |ev| set_v1.set(Some(event_target_checked(&ev)));

    let (v2, set_v2) = query_signal_with_options::<bool>(
        "v2",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_v2 = move |ev| set_v2.set(Some(event_target_checked(&ev)));

    // Summen vorläufiger Rechtsschutz
    let summe_rvg13_v = Memo::new(move |_| {
        0.0
    });

    let summe_rvg49_v = Memo::new(move |_| {
        0.0
    });

    let summe_gkg_v = Memo::new(move |_| {
        0.0
    });

    // Summen
    let summe_rvg13_netto = Memo::new( move |_| {
        let mut summe = 0.0;
        if a.get().unwrap_or(false) { summe += summe_aussergerichtlich.get() };
        if v.get().unwrap_or(0) != 1 { summe += summe_rvg13_h.get() };
        if v.get().unwrap_or(0) != 0 { summe += summe_rvg13_v.get() };
        summe
    });

    let summe_rvg49_netto = Memo::new( move |_| {
        let mut summe = 0.0;
        if a.get().unwrap_or(false) { summe += summe_aussergerichtlich.get() };
        if v.get().unwrap_or(0) != 1 { summe += summe_rvg49_h.get() };
        if v.get().unwrap_or(0) != 0 { summe += summe_rvg49_v.get() };
        summe
    });

    let (u, set_u) = query_signal_with_options::<u32>(
        "u",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_umsatzsteuer = move |ev| {
        set_u.set(Some(event_target_value(&ev).parse::<u32>().unwrap_or(19)));
    };

    let summe_gkg = Memo::new( move |_| {
        let mut summe = 0.0;
        if v.get().unwrap_or(0) != 1 { summe += summe_gkg_h.get() };
        if v.get().unwrap_or(0) != 0 { summe += summe_gkg_v.get() };
        summe
    });

    let gesamtsumme13 = Memo::new ( move |_| {
        let mut summe = 0.0;
        summe += umsatzsteuer_brutto(u.get().unwrap_or(19), summe_rvg13_netto.get());
        summe += summe_gkg.get();
        summe
    });

    let gesamtsumme49 = Memo::new ( move |_| {
        let mut summe = 0.0;
        summe += umsatzsteuer_brutto(u.get().unwrap_or(19), summe_rvg49_netto.get());
        summe += summe_gkg.get();
        summe
    });

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
                    <option value="4" selected=move || t.get().unwrap_or(4) == 4>"Aufenthaltstitel inkl. Untätigkeitsklage"</option>
                    <option value="5" selected=move || t.get().unwrap_or(4) == 5>"Ausweisung"</option>
                    <option value="6" selected=move || t.get().unwrap_or(4) == 6>"Pass/Passersatz"</option>
                    <option value="7" selected=move || t.get().unwrap_or(4) == 7>"Duldung und Abschiebung inkl. Ausbildungs-/Beschäftigungsduldung,
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
                                <div id="gebuehrensatz" popover class="open:border-2 open:border-stone-400 open:rounded-lg open:p-2 open:mt-60 open:mx-60">
                                    <h4 class="text-xl font-medium">Gebührensatz für die Geschäftsgebühr</h4>
                                    <p>{ popover::GEBUEHRENSATZ }</p>
                                </div>
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
                            <td class="px-1">"Auslagenpauschale, Nr. 7002 VV RVG"</td>
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
                                <span>"Sonstige Auslagen, z. B. Nr. 7000, 7003 ff. VV RVG"</span>
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
                        <tr class="font-semibold">
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

        // Hauptsacheverfahren
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
                        <tr>
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
                                <span class="ml-1">EUR</span>
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb49_h1.get()) }
                                <span class="ml-1">EUR</span>
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(verfgeb13_h1.get() - verfgeb49_h1.get()) }
                                <span class="ml-1">EUR</span>
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
                                <div id="ermaessigung3101" popover class="open:border-2 open:border-stone-400 open:rounded-lg open:p-2 open:mt-60 open:mx-60">
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
                                <div id="anrechnung" popover class="open:border-2 open:border-stone-400 open:rounded-lg open:p-2 open:mt-60 open:mx-60">
                                    <h4 class="text-xl font-medium">"Vorbemerkung 3 Abs. 4 VV RVG"</h4>
                                    <p>{ popover::ANRECHNUNG }</p>
                                </div>                                
                            </td>
                            <td class="px-1 text-right">
                                <span class="mr-1">"-"</span>
                                { move || format_euro(anrechnung13.get()) }
                                <span class="ml-1">"EUR"</span>
                            </td>
                            <td class="px-1 text-right">
                                <span class="mr-1">"-"</span>
                                { move || format_euro(anrechnung49.get()) }
                                <span class="ml-1">"EUR"</span>
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(anrechnung13.get() - anrechnung49.get()) }
                                <span class="ml-1">EUR</span>
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
                                <span class="ml-1">EUR</span>
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb49_h1.get()) }
                                <span class="ml-1">EUR</span>
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(tgeb13_h1.get() - tgeb49_h1.get()) }
                                <span class="ml-1">EUR</span>
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
                                <span class="ml-1">EUR</span>
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_h1.get()) }
                                <span class="ml-1">EUR</span>
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(pauschale49_h1.get() - pauschale13_h1.get()) }
                                <span class="ml-1">EUR</span>
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
                                <button popovertarget="auslagen" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                            </td>
                            <td class="px-1 text-right">
                                <input
                                    type="text"
                                    class="px-1 border-2 border-stone-400 rounded-lg text-right"
                                    value=move || h1sa.get().unwrap_or(0.0)
                                    on:change=change_h1_sonstige_auslagen
                                    prop:value=move || if h1a.get().unwrap_or(false) { format_euro(h1sa.get().unwrap_or(0.0)) } else { "0,00".to_string() }
                                />
                                <span class="ml-1">EUR</span>
                            </td>
                            <td colspan="2"></td>
                        </tr>
                    </tbody>
                </table>
                <h4 class="text-l font-bold">
                    "Gerichtskostengesetz"
                </h4>
                <p>to be done...</p>               
            </p>
            <p class=move || if h2.get().unwrap_or(false) { "visible" } else { "hidden" }>
                <h3 class="text-xl font-medium">
                    "2. Instanz"
                </h3>
                <h4 class="text-l font-bold">
                    "Rechtsanwaltsvergütungsgesetz"
                </h4>
                <p>to be done...</p>
                <h4 class="text-l font-bold">
                    "Gerichtskostengesetz"
                </h4>
                <p>to be done...</p>  
            </p>
            <p class=move || if h3.get().unwrap_or(false) { "visible" } else { "hidden" }>
                <h3 class="text-xl font-medium">
                    "3. Instanz"
                </h3>
                <h4 class="text-l font-bold">
                    "Rechtsanwaltsvergütungsgesetz"
                </h4>
                <p>to be done...</p>
                <h4 class="text-l font-bold">
                    "Gerichtskostengesetz"
                </h4>
                <p>to be done...</p>
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
                    <span class="ml-1">EUR</span>
                </div>
                <div></div>
                <div>
                    "Rechtsanwaltsvergütungsgesetz (§ 49 RVG)"
                </div>
                <div class="text-right">
                   { move || format_euro(summe_rvg49_h.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div></div>
                <div>
                    "Gerichtskostengesetz"
                </div>
                <div class="text-right">
                   { move || format_euro(summe_gkg_h.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div></div>        
            </p>
        </div>

        // Vorläufiger Rechtsschutz
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
                <h4 class="text-l font-medium">
                    "Rechtsanwaltsvergütungsgesetz"
                </h4>
                <p>to be done...</p>
                <h4 class="text-l font-medium">
                    "Gerichtskostengesetz"
                </h4>
                <p>to be done...</p>
            </p>
            <p class=move || if v2.get().unwrap_or(false) { "visible" } else { "hidden" }>
                <h3 class="text-xl font-medium">
                    "2. Instanz"
                </h3>
                <h4 class="text-l font-medium">
                    "Rechtsanwaltsvergütungsgesetz"
                </h4>
                <p>to be done...</p>
                <h4 class="text-l font-medium">
                    "Gerichtskostengesetz"
                </h4>
                <p>to be done...</p>
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
                    <span class="ml-1">EUR</span>
                </div>
                <div></div>
                <div>
                    "Rechtsanwaltsvergütungsgesetz (§ 49 RVG)"
                </div>
                <div class="text-right">
                   { move || format_euro(summe_rvg49_v.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div></div>
                <div>
                    "Gerichtskostengesetz"
                </div>
                <div class="text-right">
                   { move || format_euro(summe_gkg_v.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div></div>        
            </p>
        </div>

        // Summen
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Summe"
            </h2>
            <div class="grid grid-cols-5">
                <div class="col-span-2"></div>
                <div class="text-right font-semibold">
                    "Wertgebühren (§ 13 RVG)"
                </div>
                <div class="text-right font-semibold">
                    "Wertgebühren (§ 49 RVG / PKH)"
                </div>
                <div class="text-right font-semibold">
                    "Differenz"
                </div>
                <div class="col-span-5 pt-4 text-xl font-medium">
                    "Summe Rechtsanwaltsvergütungsgesetz"
                </div>
                <div class=move || if a.get().unwrap_or(false) == true && v.get().unwrap_or(0) != 1 { "visible col-span-2" } else { "hidden col-span-2" }>
                    "Außergerichtliche Vertretung"
                </div>
                <div class=move || if a.get().unwrap_or(false) == true  && v.get().unwrap_or(0) != 1 { "visible text-right" } else { "hidden" }>
                    { move || format_euro(summe_aussergerichtlich.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class=move || if a.get().unwrap_or(false) == true  && v.get().unwrap_or(0) != 1 { "visible text-right" } else { "hidden" }>
                    { move || format_euro(summe_aussergerichtlich.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class=move || if a.get().unwrap_or(false) == true  && v.get().unwrap_or(0) != 1 { "visible text-right" } else { "hidden" }>
                </div>
                <div class=move || if v.get().unwrap_or(0) != 1 { "visible col-span-2"} else { "hidden col-span-2" }>
                    "Hauptsacheverfahren"
                </div>
                <div class=move || if v.get().unwrap_or(0) != 1 { "visible text-right"} else { "hidden" }>
                    { move || format_euro(summe_rvg13_h.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class=move || if v.get().unwrap_or(0) != 1 { "visible text-right"} else { "hidden" }>
                    { move || format_euro(summe_rvg49_h.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class=move || if v.get().unwrap_or(0) != 1 { "visible text-right"} else { "hidden" }>
                    { move || format_euro(summe_rvg13_h.get() - summe_rvg49_h.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class=move || if v.get().unwrap_or(0) != 0 { "visible col-span-2"} else { "hidden col-span-2" }>
                    "Vorläufiger Rechtsschutz"
                </div>
                <div class=move || if v.get().unwrap_or(0) != 0 { "visibl text-right"} else { "hidden" }>
                    { move || format_euro(summe_rvg13_v.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class=move || if v.get().unwrap_or(0) != 0 { "visible text-right"} else { "hidden" }>
                    { move || format_euro(summe_rvg49_v.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class=move || if v.get().unwrap_or(0) != 0 { "visible text-right" } else { "hidden" }>
                    { move || format_euro(summe_rvg13_v.get() - summe_rvg49_v.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class="col-span-2 italic">
                    "Summe netto"
                </div>
                <div class="text-right italic">
                    { move || format_euro(summe_rvg13_netto.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class="text-right italic">
                    { move || format_euro(summe_rvg49_netto.get()) }
                    <span class="ml-1">EUR</span>
                </div>            
                <div class="text-right italic">
                    { move || format_euro(summe_rvg13_netto.get() - summe_rvg49_netto.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div>
                    "Umsatzsteuer, Nr. 7008 VV RVG"
                </div>
                <div>
                    <input type="number" min="0" value=move || u.get().unwrap_or(19) class="px-1 w-16 border-2 border-stone-400 rounded-lg" on:change=change_umsatzsteuer />
                    <span class="ml-1">%</span>
                    <button popovertarget="umsatzsteuer" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                    <div id="umsatzsteuer" popover class="open:border-2 open:border-stone-400 open:rounded-lg open:p-2 open:mt-60 open:mx-60">
                        <h4 class="text-xl font-medium">Steuersatz der Umsatzsteuer</h4>
                        <p>{ popover::UMSATZSTEUER }</p>
                    </div>
                </div>
                <div class="text-right">
                    { move || format_euro(fees::umsatzsteuer(u.get().unwrap_or(19), summe_rvg13_netto.get())) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class="text-right">
                    { move || format_euro(fees::umsatzsteuer(u.get().unwrap_or(19), summe_rvg49_netto.get())) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class="text-right">
                    { move || format_euro(fees::umsatzsteuer(u.get().unwrap_or(19), summe_rvg13_netto.get()) - fees::umsatzsteuer(u.get().unwrap_or(19), summe_rvg49_netto.get())) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class="col-span-2 font-semibold">
                    "Summe brutto"
                </div>
                <div class="text-right font-semibold">
                    { move || format_euro(fees::umsatzsteuer_brutto(u.get().unwrap_or(19), summe_rvg13_netto.get())) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class="text-right font-semibold">
                    { move || format_euro(fees::umsatzsteuer_brutto(u.get().unwrap_or(19), summe_rvg49_netto.get())) }
                    <span class="ml-1">EUR</span>
                </div>            
                <div class="text-right font-semibold">
                    { move || format_euro(fees::umsatzsteuer_brutto(u.get().unwrap_or(19), summe_rvg13_netto.get()) - fees::umsatzsteuer_brutto(u.get().unwrap_or(19), summe_rvg49_netto.get())) }
                    <span class="ml-1">EUR</span>
                </div> 
                <div class="col-span-5 pt-4 text-xl font-medium">
                    "Summe Gerichtskostengesetz"
                </div>
                <div class=move || if v.get().unwrap_or(0) != 1 { "visible col-span-2"} else { "hidden col-span-2" }>
                    "Hauptsacheverfahren"
                </div>
                <div class=move || if v.get().unwrap_or(0) != 1 { "visible text-right"} else { "hidden" }>
                    { move || format_euro(summe_gkg_h.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class=move || if v.get().unwrap_or(0) != 1 { "visible text-right"} else { "hidden" }>
                    { move || format_euro(summe_gkg_h.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class=move || if v.get().unwrap_or(0) != 1 { "visible"} else { "hidden" }>
                </div>
                <div class=move || if v.get().unwrap_or(0) != 0 { "visible col-span-2"} else { "hidden col-span-2" }>
                    "Vorläufiger Rechtsschutz"
                </div>
                <div class=move || if v.get().unwrap_or(0) != 0 { "visibl text-right"} else { "hidden" }>
                    { move || format_euro(summe_gkg_v.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class=move || if v.get().unwrap_or(0) != 0 { "visible text-right"} else { "hidden" }>
                    { move || format_euro(summe_gkg_v.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class=move || if v.get().unwrap_or(0) != 0 { "visible" } else { "hidden" }>
                </div>
                <div class="col-span-2 font-semibold">
                    "Summe"
                </div>
                <div class="text-right font-semibold">
                    { move || format_euro(summe_gkg.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class="text-right font-semibold">
                    { move || format_euro(summe_gkg.get()) }
                    <span class="ml-1">EUR</span>
                </div>            
                <div></div>             
                <div class="col-span-2 pt-4 text-xl font-medium">
                    "Gesamtsumme"
                </div>
                <div class="pt-4 text-right text-xl font-medium">
                    { move || format_euro(gesamtsumme13.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class="pt-4 text-right text-xl font-medium">
                    { move || format_euro(gesamtsumme49.get()) }
                    <span class="ml-1">EUR</span>
                </div>
                <div class="pt-4 text-right text-xl font-medium">
                    { move || format_euro(gesamtsumme13.get() - gesamtsumme49.get()) }
                    <span class="ml-1">EUR</span>
                </div>
            </div>
        </div>

        // Rechtliche Hinweise
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