# ECLOCK-071 — QA do layout operacional

Com três widgets, o painel identificado exatamente como `ESAA` recebe proporção 2:1:1, independentemente de sua posição. Sem ESAA, permanece a divisão igual de 1:1:1.

```bash
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo build --release
```
