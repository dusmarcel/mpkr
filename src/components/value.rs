use leptos::prelude::*;

use crate::utils::format_euro;
use crate::fees;
use crate::popover;

#[component]
pub fn Value(
    sk: Memo<Option<u32>>,
    v: Memo<Option<u32>>,
    set_v: SignalSetter<Option<u32>>,
    set_t_changed: WriteSignal<bool>,
    t: Memo<Option<u32>>,
    set_t: SignalSetter<Option<u32>>,
    set_p_changed: WriteSignal<bool>,
    p: Memo<Option<u32>>,
    set_p: SignalSetter<Option<u32>>,
    s: Memo<Option<f64>>,
    set_s: SignalSetter<Option<f64>>,
    sv: Memo<Option<f64>>,
    set_sv: SignalSetter<Option<f64>>
) -> impl IntoView {
    let change_verfahren = move |ev| set_v.set(Some(event_target_value(&ev).parse::<u32>().unwrap_or(0)));
    let change_thema = move |ev| {
        set_t.set(Some(event_target_value(&ev).parse::<u32>().unwrap_or(4)));
        set_t_changed.set(true);
    };
    let change_personen = move |ev| {
        set_p.set(Some(event_target_value(&ev).parse::<u32>().unwrap_or(1)));
        set_p_changed.set(true);
    };
    let change_streitwert = move |ev| set_s.set(Some(event_target_value(&ev).parse::<f64>().unwrap_or(fees::AUFFANGSTREITWERT)));
    let change_streitwert_vorl = move |ev| set_sv.set(Some(event_target_value(&ev).parse::<f64>().unwrap_or(fees::AUFFANGSTREITWERT / 2.0)));

    view! {
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
                    <option value="0" selected=move || t.get().unwrap_or(0) == 0>"Befristete Aufenthaltsrechte, z.B. Aufenthaltserlaubnis, inkl. Untätigkeitsklage und Verlust, z.B. durcn Erlöschen oder Ausweisung"</option>
                    <option value="1" selected=move || t.get().unwrap_or(0) == 1>"Unbefristete Aufenthaltsrechte, z.B. Niederlassungserlaubnis, inkl. Untätigkeitsklage und Verlust, z.B. durcn Erlöschen oder Ausweisung"</option>
                    <option value="2" selected=move || t.get().unwrap_or(0) == 2>"Pass/Passersatz"</option>
                    <option value="3" selected=move || t.get().unwrap_or(0) == 3>"Duldung und Abschiebung inkl. Ausbildungs-/Beschäftigungsduldung,
                        Untätigkeitsklage"</option>
                    <option value="4" selected=move || t.get().unwrap_or(0) == 4>"Einbürgerung und Feststellung der Staatsangehörigkeit"</option>
                    <option value="5" selected=move || t.get().unwrap_or(0) == 5>"Asylrecht: Zulässigkeit (z.B. Dublin, Drittstaatenfall, Folgeantrag)"</option>
                    <option value="6" selected=move || t.get().unwrap_or(0) == 6>"Asylrecht: Anerkennungsverfahren"</option>
                    <option value="7" selected=move || t.get().unwrap_or(0) == 7>"Asylrecht: Widerruf/Rücknahme"</option>
                    <option value="8" selected=move || t.get().unwrap_or(0) == 8>"Asylrecht: Untätigkeitsklage"</option>
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
                                <button popovertarget="zahl-der-personen" class="border-2 border-stone-400 rounded-lg px-1 ml-1">"?"</button>
                                <div id="zahl-der-personen" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                                    <h4 class="text-xl font-medium">"Zahl der Personen"</h4>
                                    <p>{ popover::PERSONS }</p>
                                </div>
                            </td>
                            <td class="px-1">
                                <input
                                    type="text"
                                    class="px-1 border-2 border-stone-400 rounded-lg text-right"
                                    value=move || if v.get().unwrap_or(0) != 1 {
                                        s.get().unwrap_or(fees::default_streitwert(t.get().unwrap_or(0), p.get().unwrap_or(1), sk.get().unwrap_or(0)))
                                    } else {
                                        sv.get().unwrap_or(fees::default_streitwert(t.get().unwrap_or(0), p.get().unwrap_or(1), sk.get().unwrap_or(0)) / 2.0)
                                    }
                                    on:change=change_streitwert
                                    prop:value=move || if v.get().unwrap_or(0) != 1 {
                                        format_euro(s.get().unwrap_or(fees::AUFFANGSTREITWERT))
                                    } else {
                                        format_euro(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))
                                    }
                                />
                            </td>
                            <td class="px-1 text-right">
                                { move || if v.get().unwrap_or(0) != 1 {
                                        format_euro(fees::rvg13_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT)))
                                    } else {
                                        format_euro(fees::rvg13_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0)))
                                    }
                                }
                            </td>
                            <td class="px-1 text-right">
                                { move || if v.get().unwrap_or(0) != 1 {
                                        format_euro(fees::rvg49_geb(s.get().unwrap_or(fees::AUFFANGSTREITWERT)))
                                    } else {
                                        format_euro(fees::rvg49_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0)))
                                    }
                                }
                            </td>
                            <td class="px-1 text-right">
                                { move || if v.get().unwrap_or(0) != 1 {
                                        format_euro(fees::gkg_geb(t.get().unwrap_or(4), s.get().unwrap_or(fees::AUFFANGSTREITWERT)))
                                    } else {
                                        format_euro(fees::gkg_geb(t.get().unwrap_or(4), sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0)))
                                    }
                                }
                            </td>                     
                        </tr>
                        <tr class=move || if v.get().unwrap_or(0) == 2 { "visible" } else { "hidden" }>
                            <td></td>
                            <td class="px-1">
                                "vorläufiger Rechtsschutz"
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
                                    value=move || sv.get().unwrap_or(fees::default_streitwert(t.get().unwrap_or(0), p.get().unwrap_or(1), sk.get().unwrap_or(0)) / 2.0)
                                    on:change=change_streitwert_vorl
                                    prop:value=move || format_euro(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))
                                />
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(fees::rvg13_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(fees::rvg49_geb(sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))) }
                            </td>
                            <td class="px-1 text-right">
                                { move || format_euro(fees::gkg_geb(t.get().unwrap_or(4), sv.get().unwrap_or(fees::AUFFANGSTREITWERT / 2.0))) }
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    }
}