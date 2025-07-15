use leptos::prelude::*;

// Rechtliche Hinweise

#[component]
pub fn Notes() -> impl IntoView {
    view! {
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
                Prozesskostenhilfe, daher werden für die außergerichtliche Vertretung immer die Wergebühren nach § 13 RVG ausgewiesen.
                Für die Abrechnung gegenüber Mandant*innen sind in aller Regel die Gebühren nach § 13 RVG in der jeweils linken Spalte maßgeblich.
                Die PKH-Gebühren werden meist nur für die Abrechnung gegenüber der Staatskasse benötigt, wenn Prozesskostenhilfe bewilligt wurde.
                Dass die PKH-Gebühren und ihre Differenz zu den Wertgebühren nach § 13 RVG hier gesondert ausgewiesen werden, versteht sich
                daher eher als Serviceleistung für Anwaltskanzleien. Für Rechtssuchende hingegen sind sie grundsätzlich irrelevant, da
                es für sie nur auf die Wertgebühren gemäß § 13 RVG ankommt."
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