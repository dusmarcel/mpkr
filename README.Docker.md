### Entwicklung mit Hot Reload (Tailwind + Trunk)

Starte die Entwicklungsumgebung mit:

`docker compose up --build dev`

Die Anwendung ist dann unter http://localhost:8025 erreichbar.

Hinweise:
- Quellcode wird per Bind-Mount in den Container eingebunden.
- `trunk serve` beobachtet Dateien und löst bei Änderungen automatisch einen Rebuild/Reload aus.
- Tailwind wird im Dev-Container über die installierte CLI verarbeitet.
- Die Node-Abhängigkeiten (inkl. Tailwind-Paket) werden im Container installiert und im benannten Volume `node-modules` gecacht.

### Production Build und Auslieferung (nginx)

Baue und starte das Production-Setup mit:

`docker compose --profile prod up --build prod`

Die Anwendung ist dann unter http://localhost:8026 erreichbar.

### Eigenes Image bauen

`docker build -t myapp .`

Falls eine andere Zielarchitektur benötigt wird (z. B. amd64):

`docker build --platform=linux/amd64 -t myapp .`

Danach kann das Image wie gewohnt in eine Registry gepusht werden.
