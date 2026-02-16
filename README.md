# MPKR

Migrationsrechtlicher Prozesskostenrechner

## About

Dieser Prozesskostenrechner berechnet gesetzliche Gebühren auf der Grundlage des Rechtsanwaltsvergütungsgesetzes (RVG), des Gerichtskostengesetzes (GKG), des Streitwertkatalogs des Bundesverwaltungsgerichts und meiner Erfahrung mit der Interpretation dieser Vorgaben durch die Verwaltungsgerichte vornehmlich in NRW. Der Rechner dient nur einer unverbindlichen Orientierung und kann eine fachkundige Beratung nicht ersetzen. Seine Nutzung erfolgt insofern auf eigene Gefahr. Es kann eine Vielzahl von Gründen geben, warum ein Gericht höhere oder niedrigere Kosten festsetzt, als von diesem Rechner ermittelt. Auch möchten viele Rechtsanwält*innen Vergütungsvereinbarungen schließen, die zum Teil deutlich von den gesetzlich vorgesehenen Gebühren abweichen. Der Rechner geht zudem auch davon aus, dass sich die Behördenseite nicht anwaltlich vertreten lässt. Tatsächlich lassen sich Behörden in migrationsrechtlichen Streitigkeiten erfahungsgemäß selten anwaltlich vertreten. Völlig ausgeschlossen ist es aber auch nicht. Ggf. würden hierdurch weitere Kosten entstehen, die dieser Rechner nicht berücksichtigt. Außerdem berücksichtigt dieser Rechner keine Gebühren, die durch Behörden im Verwaltungsverfahren erhoben werden. Widerspruchs- und Remonstrationsverfahren werden ebenfalls (noch?) nicht abgebildet, was vor allem daran liegt, dass ich hauptsächlich in NRW tätig bin, wo es kaum noch Widerspruchsverfahren gibt. Die Anrechnung der Geschäftsgebühr auf die Verfahrensgebühr (Vorbemerkung 4 zu Teil 3 VV RVG) erfolgt immer auf die 1. Instanz des Hauptsacheverfahrens, da dies auch in der Praxis nahezu immer der Fall sein wird. Soweit zumindest theoretisch auch Fälle konstruiert werden können, in denen die Anrechnung auf die Verfahrensgebühr in einer höheren Insatz erfolgt, bleiben diese Fälle hier um der Einfachheit willen unberücksichtigt.

## Installation

Der Rechner kann mit Docker oder mit Trunk installiert werden. In beiden Fällen ist es erforderlich, das Repository zunächst zu klonen:

```console
$  git clone https://github.com/dusmarcel/mpkr.git
```

Anschließend wechselt man in das entsprechende Verzeichnis:

```console
$ cd mpkr
```

### Installation mit Docker

#### Entwicklung (Hot Reload)

Wenn Docker installiert ist und der Daemon läuft, kann die Entwicklungsumgebung mit

```console
$ docker compose up --build dev
```

gebaut und gestartet werden. Die Anwendung ist unter http://localhost:8025 erreichbar.

Hinweise:
- Die Quelltexte werden in den Container gemountet.
- `trunk serve` beobachtet Änderungen und löst automatisch Rebuild/Reload aus.
- Tailwind wird im Container verarbeitet.
- Node-Abhängigkeiten (inkl. Tailwind) werden im Container installiert und in einem benannten Volume gecacht.

#### Produktion (statisches Serving via nginx)

Für das Produktionsprofil:

```console
$ docker compose --profile prod up --build prod
```

Die Anwendung ist dann unter http://localhost:8026 erreichbar.

### Installation mit Trunk

Da der Rechner vollständig clientseitig ausgeführt wird, sind serverseitig keine besonderen Vorbereitungen erforderlich. Die Installation sollte auf jedem üblichen Webserver möglich sein. Für die Installation wird [Rust](https://www.rust-lang.org/tools/install) benötigt, ferner das Target wasm32-unknown-unknown:

```console
$ rustup target add wasm32-unknown-unknown
```

Trunk muss auch noch installiert werden:

```console
$ cargo install trunk
```

Die benötigten JavaScript- und WASM-Dateien werden erzeugt mit:

```console
$ trunk build --release
```

Trunk erzeugt einen Unterordner `dist`. Dessen Inhalt muss jetzt nur noch in das gewünschte Verzeichnis des Webservers verschoben oder kopiert werden.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
