# ECLOCK-063 — Cobertura automatizada

## Escopo

- parsing de configuração válida e rejeição de TOML inválido;
- limites mínimos de refresh e timeout;
- timeout real de comandos;
- truncamento seguro em fronteira UTF-8;
- execução assíncrona e prevenção de refresh duplicado;
- todos os glifos suportados nas escalas 1–3;
- seleção e fallback dos temas documentados.

## Comandos

```bash
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
cargo build --release
```

Os resultados atuais são registrados na conclusão governada da tarefa ECLOCK-063.
