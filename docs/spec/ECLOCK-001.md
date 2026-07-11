# ECLOCK-001 — Especificação elzo-clock

## Visão
Dashboard TUI pessoal com relógio digital grande e painéis (widgets) alimentados por comandos shell finitos. Inspiração conceitual: [clock-tui/tclock](https://github.com/akitaonrails/clock-tui) — **sem copiar código**.

## Binary
- Pacote: `elzo-clock`
- CLI: `eclock`

## Funcionalidades v0.1
1. Relógio digital (escala 1–3), data e segundos opcionais
2. Até 3 widgets laterais (comando → stdout)
3. Config XDG: `~/.config/elzo-clock/config.toml`
4. Atalhos: q/r/s/d/+/-
5. **Widget ESAA** via helper `eclock-esaa` (verify + eligible + suppressed)

## Widget ESAA (obrigatório no default)
- Comando: `eclock-esaa` ou `eclock-esaa <root>`
- Env: `ECLOCK_ESAA_ROOT`
- Detecta `.roadmap/` no cwd/ancestrais e candidatos em `~/…`
- Saída legível para TUI (não JSON bruto)

## Fora de escopo v0.1
Timer/stopwatch, scroll mouse, temas múltiplos, >3 widgets, PyPI do eclock

## Critérios de aceite
- `cargo build --release` gera `eclock`
- `eclock-esaa` roda contra workspace ESAA e imprime verify/eligible
- Workspace governado com `esaa verify` ok
