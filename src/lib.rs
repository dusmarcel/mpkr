use leptos::prelude::*;
use leptos_router::hooks::query_signal;
//use leptos_router::components::Form;

#[component]
pub fn MPKR() -> impl IntoView {
    let (v, set_v) = query_signal::<i32>("v");
    let change_verfahren = move |ev| set_v.set(Some(event_target_value(&ev).parse::<i32>().unwrap_or(0)));

    view! {
        <div class="container">
            //<Form method="GET" action="">
                <div class="graydient">
                    <h1>
                        "Migrationsrechtlicher Prozesskostenrechner (Stand 2025)"
                    </h1>
                    <p  class="mb-0">
                        "Erstellt von "<a href="https://aufentha.lt">"Marcel Keienborg"</a>". Bitte beachte unbedingt auch die
                        Hinweise unten auf dieser Seite."
                    </p>
                </div>
                <div class="graydient">
                    <h2>
                        "Streitwerte"
                    </h2>
                    <p class="mb-2">
                        "Grundlage für die Berechnung der Gebühren ist der sogenannte Streitwert. Wir müssen also zuerst die Streitwerte für deine Angelegenheit ermitteln. Plural, weil gerade in gerichtlichen Verfahren häufig zusätzlich zur Klage noch ein Antrag auf Gewährung vorläufigen Rechtsschutzes erforderlich ist. Manchmal wird auch nur ein Antrag auf vorläufigen Rechtsschutz gestellt. Deswegen kann man hier verschiedene Optionen wählen. Außerdem hängt die konkrete Höhe des Streitwerts auch von der Anzahl der Personen ab, die in einem Verfahren zusammengefasst werden."
                    </p>
                    <p class="mb-1">
                        <label for="verfahren">"Wähle aus, ob die Gebühren nur für ein Hauptsacheverfahren, nur für dein Verfahren
                        zum vorläufigen Rechtsschutz, oder für beides berechnet werden sollen."</label>
                    </p>
                    <p class="mb-2">
                        <select aria-label="Auswahl der Verfahrensart" id="verfahren" on:change=change_verfahren>
                            <option value="0" selected=true>"Nur Hauptsacheverfahren"</option>
                            <option value="1">"Nur Verfahren zum vorläufigen Rechtsschutz"</option>
                            <option value="2">"Hauptsacheverfahren und Verfahren zum vorläufigen Rechtsschutz"</option>
                        </select>
                    </p>
                    <p class="mb-1">
                        <label for="thema">"Wähle ein Thema, dann versucht der Rechner, dir die passenden Streitwerte vorzuschlagen.
                        Du kannst aber auch manuell selbst Streitwerte angeben."</label>
                    </p>
                    <p class="mb-2">
                        <select aria-label="Auswahl des Themas" id="thema">
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
                    <div class="row fw-bold">
                        <div class="col-2">
                            <p><label for="anzahl">"Anzahl Personen"</label></p>
                        </div>
                        <div class="col-2">
                            <p><label>"Streitwerte"</label></p>
                        </div>
                        <div class="col-2">
                            <p><label>"Wertgebühr (§ 13 RVG)"</label></p>
                        </div>
                        <div class="col-2">
                            <p><label>"Wertgebühr (§ 49 RVG / Prozesskostenhilfe)"</label></p>
                        </div>
                        <div class="col-2">
                            <p><label>"Differenz"</label></p>
                        </div>
                        <div class="col-2">
                            <p><label>"Wertgebühr (GKG)"</label></p>
                        </div>
                    </div>
                    <div class="row">
                        <div class="col-2"></div>
                        <div class="col-2"><label>{move || match v.get().unwrap_or(0) {
                                0 => "Hauptsache",
                                1 => "vorläufiger Rechtsschutz",
                                _ => "Hauptsache"
                            }
                        }</label></div>
                        <div class="col-8"></div>
                    </div>
                </div>
                <div class="graydient">
                    <h2>
                        "Rechtliche Hinweise"
                    </h2>
                    <p class="mb-2">
                        "Dieser Prozesskostenrechner berechnet gesetzliche Gebühren auf der Grundlage des 
                        Rechtsanwaltsvergütungsgesetzes ("<a href="https://dejure.org/gesetze/RVG">"RVG"</a>
                        "), des Gerichtskostengesetzes ("<a href="https://dejure.org/gesetze/GKG">"GKG"</a>
                        "), des "<a href="https://www.bverwg.de/rechtsprechung/streitwertkatalog">"Streitwertkatalogs"</a>
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
                    <p class="mb-2">
                        "Der Rechner geht äußerst sparsam mit deinen Daten um. Zwar werden einige technisch benötigte Daten,
                        insbesondere deine IP-Adresse, an meinen Server gesendet und von meinem Server verarbeitet. Das ist nötig,
                        um die zum Rechner gehörenden Dateien an deinen Browser oder dein sonstiges Gerät, mit welchem du den
                        Rechner ausführst, schicken zu können. Der Rechner wird aber vollständig auf deinem Gerät ausgeführt. Das
                        bedeutet, dass alle Daten, die du in den Rechner eingibst, und die Ergebnisse, die von meinem Rechner
                        erzeugt werden, vollständig bei dir verbleiben, und erst gar nicht an meinen Server geschickt, geschweige
                        denn verarbeitet oder gespeichert werden."
                    </p>
                    <p class="mb-2">
                        "Der Rechner ist zudem auch als Freie Software unter den Lizenzen Apache, Version 2.0, und MIT
                        veröffentlicht. Du kannst dir die Software also auch aus dem "<a href="https://github.com/dusmarcel/mpkr25">
                        "Repository"</a>" herunterladen und sie dann ganz auf einem Gerät deiner Wahl ausführen.
                        In diesem Falle hast du mit meinem Server gar nichts mehr zu tun, und die Notwendigkeit, Daten an meinen
                        Server zu übertragen, entfällt ganz."
                    </p>
                    <p>
                        "Und schließlich geht es hier noch zu meinem „"<a href="https://aufentha.lt/index.php/impressum/">"Impressum"</a>"“."
                    </p>
                </div>
            //</Form>
        </div>
        <noscript>"This page contains webassembly and javascript content, please enable javascript in your browser."</noscript>
    }
}