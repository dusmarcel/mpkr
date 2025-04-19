use leptos::prelude::*;
use leptos_router::{hooks::query_signal_with_options, location::State, NavigateOptions};

mod popover;
use popover::*;

#[component]
pub fn MPKR() -> impl IntoView {
    let (v, set_v) = query_signal_with_options::<i32>(
        "v", 
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_verfahren = move |ev| set_v.set(Some(event_target_value(&ev).parse::<i32>().unwrap_or(0)));

    let (t, set_t) = query_signal_with_options::<i32>(
        "t",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });
    let change_thema = move |ev| set_t.set(Some(event_target_value(&ev).parse::<i32>().unwrap_or(4)));

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
                <select class="border-2 border-stone-400 rounded-lg p-1" aria-label="Auswahl der Verfahrensart" id="verfahren" on:change=change_verfahren>
                    <option value="0" selected=true>"Nur Hauptsacheverfahren"</option>
                    <option value="1">"Nur Verfahren zum vorläufigen Rechtsschutz"</option>
                    <option value="2">"Hauptsacheverfahren und Verfahren zum vorläufigen Rechtsschutz"</option>
                </select>
            </p>
            <p>
                <label for="thema">"Wähle ein Thema, dann versucht der Rechner, dir die passenden Streitwerte vorzuschlagen.
                Du kannst aber auch manuell selbst Streitwerte angeben."</label>
            </p>
            <p>
                <select class="border-2 border-stone-400 rounded-lg p-1" aria-label="Auswahl des Themas" id="thema" on:change=change_thema>
                    <option value="0">"Asylrecht: Zulässigkeit (z.B. Dublin, Drittstaatenfall, Folgeantrag)"</option>
                    <option value="1">"Asylrecht: Anerkennungsverfahren"</option>
                    <option value="2">"Asylrecht: Widerruf/Rücknahme"</option>
                    <option value="3">"Asylrecht: Untätigkeitsklage"</option>
                    <option value="4" selected=true>"Aufenthaltsrecht: Aufenthaltstitel inkl. Untätigkeitsklage"</option>
                    <option value="5">"Ausweisung"</option>
                    <option value="6">"Pass/Passersatz"</option>
                    <option value="7">"Aufenthaltsrecht: Duldung und Abschiebung inkl. Ausbildungs-/Beschäftigungsduldung,
                        Untätigkeitsklage"</option>
                    <option value="8">"Einbürgerung und Feststellung der Staatsangehörigkeit"</option>
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
                                "Wertgebühr (§ 49 RVG / Prozesskostenhilfe)"
                            </th>
                            // <th class="px-1">
                            //     "Differenz"
                            // </th>
                            <th class="px-1">
                                "Wertgebühr (GKG)"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                            </td>
                            <td  class="px-1">
                                {move || match v.get().unwrap_or(0) {
                                    0 => "Hauptsache",
                                    1 => "vorläufiger Rechtsschutz",
                                    _ => "Hauptsache"
                                }}
                            </td>
                            <td>
                            </td>
                            <td>
                            </td>
                            <td>
                            </td>
                            //<td>
                            //</td>
                        </tr>
                        <tr>
                            <td class="px-1">
                                <input type="number" min="1" value="1" id="anzahl" class="border-2 border-stone-400 rounded-lg px-1" />
                                <button popovertarget="zahl-der-personen" class="border-2 border-stone-400 rounded-lg px-1 ml-1">?</button>
                                <div id="zahl-der-personen" popover class="open:border-2 open:border-stone-400 open:rounded-lg open:p-2 open:mt-60 open:mx-60">
                                    <h4 class="text-xl font-medium">Zahl der Personen</h4>
                                    <p>{PERSONS}</p>
                                </div>
                            </td>
                            <td class="px-1">
                                <input type="text" class="border-2 border-stone-400 rounded-lg px-1" value="5.000,00" id="streitwert" />
                                EUR
                            </td>
                            <td>
                            </td>
                            <td>
                            </td>
                            <td>
                            </td>
                            <td>
                            </td>                            
                        </tr>
                    </tbody>
                </table>
            //     <div class="col-3 d-grid align-items-center">
            //       <div id="div_streitwert">
            //         <div class="input-group">
            //           <input type="text" class="form-control" value="5.000,00" id="streitwert">
            //           <label class="input-group-text" for="streitwert">EUR</label>
            //         </div>
            //       </div>
            //     </div>
            //     <div class="col-2 d-grid align-items-center" id="l_geb13_1"></div>
            //     <div class="col-3 d-grid align-items-center" id="l_geb49_1"></div>
            //     <div class="col-2 d-grid align-items-center" id="l_gkg_1"></div>
            //     <div class="col-1"></div>
            //   </div>
            //   <div class="row collapse" id="div_l_streitwert_v">
            //     <p>
            //     <div class="col-2"></div>
            //     <div class="col-3">
            //       <div>
            //         <label for="streitwert_v">vorläufiger Rechtsschutz</label>
            //       </div>
            //     </div>
            //     <div class="col"></div>
            //     </p>
            //   </div>
            //   <div class="row collapse" id="div_streitwert_v">
            //     <div class="col-2"></div>
            //     <div class="col-3 d-grid align-items-center">
            //       <div class="input-group">
            //         <input type="text" class="form-control" value="2.500,00" id="streitwert_v">
            //         <label class="input-group-text" for="streitwert_v">EUR</label>
            //       </div>
            //     </div>
            //     <div class="col-2 d-grid align-items-center" id="l_geb13_2"></div>
            //     <div class="col-3 d-grid align-items-center" id="l_geb49_2"></div>
            //     <div class="col-2 d-grid align-items-center" id="l_gkg_2"></div>
            </div>
        </div>
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