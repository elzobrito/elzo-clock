# Política de publicação (GitHub)

## O que entra no repositório público

| Incluir | Exemplos |
|---------|----------|
| Código e testes | `src/`, `Cargo.toml`, `Cargo.lock` |
| Helpers versionados | `scripts/eclock-esaa`, `eclock-gcal`, `eclock-sysinfo`, … |
| Docs de produto/segurança | `README.md`, `docs/spec/`, `docs/security/`, `docs/qa/` (sem PII) |
| CI | `.github/workflows/` |
| Licença | `LICENSE` (MIT) |
| Contratos ESAA (templates) | `.roadmap/*.yaml`, `*.schema.json`, `PROJECTION_SPEC.md`, etc. |
| Exemplo de config | `config.example.toml` |

## O que fica só local (`.gitignore`)

| Excluir | Motivo |
|---------|--------|
| `.roadmap/activity.jsonl` + lock | Ledger operacional do operador |
| `.roadmap/roadmap.json`, `issues.json`, `lessons.json` | Projeções derivadas do ledger |
| `.roadmap/artifacts/` | Snapshots de file-effects (podem conter conteúdo sensível de tasks) |
| `.roadmap/*.lock.json` | Estado de plugins/roadmaps |
| `reports/` | Saída da auditoria security-audit (paths de máquina) |
| `security-audit-input.json` | Input local da auditoria |
| `.env*` | Secrets |
| `/target/` | Build |

Clone fresco: se quiser ESAA, rode `python -m esaa --root . init` (ou bootstrap) no workspace. O histórico de activity **deste** desenvolvimento não é parte do artefato open-source.

## PII e paths

- Não versionar e-mails pessoais, client secrets OAuth ou tokens.
- Exemplos usam placeholders: `your@email.example`, `~/path/to/...`.
- Config do usuário permanece em `~/.config/elzo-clock/` (fora do git).

## Release checklist

1. `cargo test`
2. Revisar `git status` / sem paths absolutos de máquina no diff
3. Commit na branch principal
4. Remote + push
5. (Opcional) tag `v0.1.0` e GitHub Release
