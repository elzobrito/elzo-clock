# ECLOCK-069 — QA do destaque de hoje

## Cobertura

- hoje em formato `gcalcli`, com e sem códigos ANSI;
- hoje em formato ISO;
- ontem e amanhã sem destaque;
- vermelho restrito ao widget Agenda;
- reconhecimento dos títulos Agenda/Calendar;
- data injetada nos testes, independente do relógio da execução.

## Verificações

```bash
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo build --release
scripts/eclock-gcal
```
