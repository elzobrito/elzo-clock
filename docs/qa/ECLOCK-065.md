# ECLOCK-065 — Qualidade e dependências

## Resultado

| Verificação | Resultado |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --all-targets -- -D warnings` | PASS |
| `cargo test --all-targets` | PASS — 10 testes |
| `cargo build --release` | PASS |
| `cargo audit` | PASS — 0 vulnerabilidades |

Ferramenta instalada: `cargo-audit 0.22.2`. A base RustSec consultada continha 1.159 advisories e foram verificadas 126 dependências do lockfile.

## Warnings RustSec tratados

O audit reportou dois warnings transitivos, ambos introduzidos por `ratatui 0.29.0`:

- `RUSTSEC-2024-0436`: `paste 1.0.15` está sem manutenção;
- `RUSTSEC-2026-0002`: `lru 0.12.5::IterMut` possui alerta de soundness.

Não há uso direto dessas crates no `elzo-clock`; especificamente, a aplicação não chama `lru::IterMut`. A mitigação atual é manter o alerta documentado e exigir nova auditoria ao atualizar `ratatui`, evitando uma troca de versão principal sem teste visual e de regressão da TUI.
